use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use chrono::{offset::Utc, DateTime};
use dashmap::DashMap;
use pnet::datalink;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::mpsc::{channel, Sender};

pub const INIT_PORT: u16 = 44444;
pub const LISTEN_PORT: u16 = 43210;
pub const PARROT_SPHINX_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 202, 0, 1));

pub mod ardrone3;
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
    /// Connects to drone

    /// Spawns Listener at LISTENER_PORT
    /// Perfroms Handshake at INIT_PORT
    /// Spawns Command sender at `c2d_port`
    pub fn new(addr: IpAddr) -> AnyResult<Self> {
        let local_ip = local_ip(addr)
            .ok_or_else(|| anyhow!("couldn't find local ip in the target network {}", addr))?;

        let local_listener = SocketAddr::new(local_ip, LISTEN_PORT);
        spawn_listener(local_listener)?;

        let init_addr = SocketAddr::new(addr, INIT_PORT);
        let handshake_response = perform_handshake(init_addr, local_listener.port())?;

        let cmd_sender_addr = SocketAddr::new(addr, handshake_response.c2d_port);

        println!("spawning cmd sender on {}", cmd_sender_addr);
        let sender = spawn_cmd_sender(local_ip, cmd_sender_addr)?;
        Ok(Self {
            sequence_ids: DashMap::new(),
            sender,
        })
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

    pub fn send_date_time(&self, date: DateTime<Utc>) -> AnyResult<()> {
        use command::Feature::Common;
        use common::Class;
        use frame::{BufferID, Frame, Type};

        let date_feature = Common(Class::Common(common::Common::CurrentDate(date)));

        let frame = Frame::for_drone(self, Type::DataWithAck, BufferID::CDAck, date_feature);

        self.send_frame(frame)?;

        let time_feature = Common(Class::Common(common::Common::CurrentTime(date)));
        let frame = Frame::for_drone(self, Type::DataWithAck, BufferID::CDAck, time_feature);

        self.send_frame(frame)
    }

    pub(crate) fn sequence_id(&self, buffer_id: frame::BufferID) -> u8 {
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
    // #[serde(skip_serializing_if = "Option::is_some")]
    // arstream2_client_stream_port: Option<u16>,
    // #[serde(skip_serializing_if = "Option::is_some")]
    // arstream2_client_control_port: Option<u16>,
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

fn perform_handshake(target: SocketAddr, d2c_port: u16) -> AnyResult<HandshakeResponse> {
    let handshake_message = HandshakeMessage {
        controller_name: "Drone-rs".to_string(),
        controller_type: "computer".to_string(),
        d2c_port,
        // arstream2_client_stream_port: Some(44445),
        // arstream2_client_control_port: Some(44446),
    };

    println!(
        "connecting controller {}",
        handshake_message.controller_name,
    );

    let mut handshake_stream =
        retry(10, target).ok_or_else(|| anyhow!("Couldn't connect for handshake {}", target))?;

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
                String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "unknown".to_string())
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

fn spawn_cmd_sender(local_ip: IpAddr, target_addr: SocketAddr) -> AnyResult<Sender<Vec<u8>>> {
    let local_addr = SocketAddr::new(local_ip, target_addr.port());

    let socket = UdpSocket::bind(local_addr)
        .map_err(|e| anyhow!("Couldn't bind to local socket {} - {}", local_addr, e))?;

    let (tx, rx) = channel::<Vec<u8>>();
    std::thread::spawn(move || loop {
        let frame_to_send = rx.recv().expect("couldn't receive frame.");

        print_message(&frame_to_send);
        let size = socket
            .send_to(&frame_to_send, target_addr)
            .expect("something terrible happened");
        println!("sent {}", size);
    });
    Ok(tx)
}

fn retry(retry: usize, target: SocketAddr) -> Option<TcpStream> {
    let timeout = std::time::Duration::from_secs(2);
    for retry_time in 0..retry {
        match TcpStream::connect_timeout(&target, timeout) {
            Ok(stream) => return Some(stream),
            Err(err) => eprintln!("Error connecting to Tcp ({}): {}", retry_time, err),
        };
    }

    None
}
