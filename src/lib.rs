use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use dashmap::DashMap;
use pnet::datalink;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::mpsc::{channel, Sender};

const DEFAULT_ADDR: &str = "192.168.2.1";
const INIT_PORT: u16 = 44444;
const LISTEN_PORT: u16 = 43210;

pub mod command;
pub mod common;
pub mod frame;
pub mod jumping_sumo;

pub struct Drone {
    // Each frame::BufferID gets its own sequence_id
    sequence_ids: DashMap<frame::BufferID, u8>,
    sender: Sender<Vec<u8>>,
}

impl Drone {
    pub fn build_frame(
        &self,
        frame_type: frame::Type,
        buffer_id: frame::BufferID,
        feature: command::Feature,
    ) -> frame::Frame {
        frame::Frame::new(frame_type, buffer_id, feature, self.sequence_id(buffer_id))
    }

    pub fn send_frame(&self, frame: frame::Frame) -> AnyResult<()> {
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
    fn sequence_id(&self, buffer_id: frame::BufferID) -> u8 {
        if let Some(mut sequence_id) = self.sequence_ids.get_mut(&buffer_id) {
            let command_id = *sequence_id;
            *sequence_id += 1;
            command_id
        } else {
            self.sequence_ids.insert(buffer_id, 1);
            1
        }
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
            sequence_ids: DashMap::new(),
            sender,
        })
    }
}
