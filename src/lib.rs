use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use pnet::datalink;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

const DEFAULT_ADDR: &str = "192.168.2.1";
const INIT_PORT: u16 = 44444;
const LISTEN_PORT: u16 = 43210;

pub mod command;
pub mod frame;
pub mod jumping_sumo;

pub struct Drone {
    cmd_count: AtomicU8,
    sender: Sender<Vec<u8>>,
}

impl Drone {
    pub fn build_frame(
        &self,
        frame_type: frame::Type,
        id: frame::ID,
        feature: command::Feature,
    ) -> frame::Frame {
        frame::Frame::new(frame_type, id, feature, self.cmd_id())
    }

    pub fn send_frame(&self, frame: frame::Frame) -> AnyResult<()> {
        // .serialize(self.cmd_id())
        self.send_raw_frame_unchecked(frame)
    }

    pub fn send_raw_frame_unchecked(&self, frame: impl frame::IntoRawFrame) -> AnyResult<()> {
        self.send(frame.into_raw().0)
    }

    fn send(&self, raw_frame: Vec<u8>) -> AnyResult<()> {
        self.sender.send(raw_frame).map_err(AnyError::new)
    }
}

impl Drone {
    fn cmd_id(&self) -> u8 {
        self.cmd_count.fetch_add(1, Ordering::SeqCst)
    }
}

// returns ip of the interface that is in the same network as the target
fn local_ip(target: IpAddr) -> Option<IpAddr> {
    datalink::interfaces()
        .into_iter()
        .filter_map(|interface| interface.ips.into_iter().find(|ip| ip.contains(target)))
        .map(|ip_network| ip_network.ip())
        .next()
}

#[derive(Serialize)]
struct HandshakeMessage {
    controller_name: String,
    controller_type: String,
    d2c_port: u16,
}

#[derive(Deserialize, Debug)]
struct HandshakeResponse {
    arstream_fragment_maximum_number: u8,
    arstream_fragment_size: u16,
    arstream_max_ack_interval: i8,
    c2d_port: u16,
    c2d_update_port: u16,
    c2d_user_port: u16,
    status: i8,
}

fn perform_handshake(target: impl ToSocketAddrs) -> AnyResult<HandshakeResponse> {
    let handshake_message = HandshakeMessage {
        controller_name: "Drone-rs".to_string(),
        controller_type: "computer".to_string(),
        d2c_port: LISTEN_PORT,
    };

    println!(
        "connecting controller {}",
        handshake_message.controller_name,
    );

    let mut handshake_stream = TcpStream::connect(target)?;

    // Send handshake
    serde_json::to_writer(&mut handshake_stream, &handshake_message)?;
    // Receive response
    let handshake_response = HandshakeResponse::deserialize(
        &mut serde_json::Deserializer::from_reader(&handshake_stream),
    )?;
    if handshake_response.status != 0 {
        anyhow!("connection refused - {:?}", handshake_response);
    }
    Ok(handshake_response)
}

fn spawn_listener(addr: impl ToSocketAddrs) -> AnyResult<()> {
    let listener = UdpSocket::bind(addr)?;
    std::thread::spawn(move || loop {
        let mut buf = [0; 1024];
        if let Ok((bytes_read, origin)) = listener.recv_from(&mut buf) {
            println!("Read {} bytes from {}", bytes_read, origin.ip());
            println!(
                "{}",
                String::from_utf8(buf.to_vec()).unwrap_or("unknown".to_string())
            );
        }
    });
    Ok(())
}

fn print_message(buf: &[u8]) {
    for b in buf.iter() {
        print!("0x{:x} ", b);
    }
    println!();
}

fn spawn_cmd_sender<'f>(
    local_ip: IpAddr,
    target_ip: IpAddr,
    port: u16,
) -> AnyResult<Sender<Vec<u8>>> {
    let socket = UdpSocket::bind((local_ip, port)).map_err(|e| {
        anyhow!(
            "Couldn't bind to local socket {}:{} - {}",
            local_ip,
            port,
            e
        )
    })?;

    let (tx, rx) = channel::<Vec<u8>>();
    std::thread::spawn(move || loop {
        let mut frame_to_send = rx.recv().expect("couldn't receive frame.");

        print_message(&frame_to_send);
        let size = socket
            .send_to(&mut frame_to_send, (target_ip, port))
            .expect("something terrible happened");
        println!("sent {}", size);
    });
    Ok(tx)
}

impl Drone {
    pub fn new(addr: Option<IpAddr>) -> AnyResult<Self> {
        let addr = addr.unwrap_or(
            DEFAULT_ADDR
                .parse()
                .expect("couldn't parse default ip address"),
        );

        let local_ip = local_ip(addr)
            .ok_or_else(|| anyhow!("couldn't find local ip in the target network {}", addr))?;

        spawn_listener((local_ip, LISTEN_PORT))?;
        let handshake_response = perform_handshake((addr, INIT_PORT))?;
        println!(
            "spawning cmd sender on {}:{}",
            addr, &handshake_response.c2d_port
        );
        let sender = spawn_cmd_sender(local_ip, addr, handshake_response.c2d_port)?;
        Ok(Self {
            cmd_count: AtomicU8::new(1),
            sender,
        })
    }

    // pub fn spin_right(&self) -> AnyResult<()> {
    //     let mut ps = PilotState {
    //         flag: 1,
    //         speed: 0,
    //         turn: 127,
    //     };
    //     self.sender
    //         .send(DroneCommand::drive(ps).serialize(self.cmd_id()));
    //     Ok(())
    // }

    // pub fn spin_left(&self) -> AnyResult<()> {
    //     let mut ps = PilotState {
    //         flag: 1,
    //         speed: 0,
    //         turn: -128,
    //     };
    //     self.sender
    //         .send(DroneCommand::drive(ps).serialize(self.cmd_id()));
    //     Ok(())
    // }

    // pub fn stop(&self) -> AnyResult<()> {
    //     let mut ps = PilotState {
    //         flag: 0,
    //         speed: 0,
    //         turn: 0,
    //     };
    //     self.sender
    //         .send(DroneCommand::drive(ps).serialize(self.cmd_id()));
    //     Ok(())
    // }
}

struct Params(Vec<Param>);

impl IntoIterator for Params {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Into<Vec<String>> for Params {
    fn into(self) -> Vec<String> {
        self.0.into_iter().map(|i| i.0).collect()
    }
}

impl From<Vec<i32>> for Params {
    fn from(v: Vec<i32>) -> Self {
        Self(v.into_iter().map(|e| e.into()).collect())
    }
}

impl From<Vec<f32>> for Params {
    fn from(v: Vec<f32>) -> Self {
        Self(v.into_iter().map(|e| e.into()).collect())
    }
}
impl From<Vec<&str>> for Params {
    fn from(v: Vec<&str>) -> Self {
        Self(v.into_iter().map(|e| e.into()).collect())
    }
}

#[derive(Clone)]
struct Param(String);

impl std::string::ToString for Param {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<i32> for Param {
    fn from(i: i32) -> Self {
        Self(i.to_string())
    }
}

impl From<f32> for Param {
    fn from(f: f32) -> Self {
        Self(f.to_bits().to_string())
    }
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        Self(format!("\"{}\"", s))
    }
}

impl From<Duration> for Param {
    fn from(d: Duration) -> Self {
        Self(format!("{}", d.as_secs().to_string()))
    }
}
