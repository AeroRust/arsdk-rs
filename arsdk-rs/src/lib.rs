use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use pnet::datalink;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::mpsc::{channel, Sender};

pub const INIT_PORT: u16 = 44444;
pub const LISTEN_PORT: u16 = 43210;
pub const PARROT_SPHINX_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 202, 0, 1));
pub const PARROT_SPHINX_CONFIG: Config = Config {
    drone_addr: PARROT_SPHINX_IP,
    // @TODO: Once we fix the Date Time sending, set to `TRUE`
    send_datetime: false,
};

pub mod ardrone3;
pub mod command;
pub mod common;
pub mod frame;
mod handshake;
pub mod jumping_sumo;

pub(crate) use handshake::perform_handshake;

pub mod prelude {
    pub use crate::{Config, Drone, PARROT_SPHINX_CONFIG, PARROT_SPHINX_IP};
}

#[derive(Debug)]
pub struct Config {
    pub drone_addr: IpAddr,
    /// Wheather or not it should send:
    ///
    /// ```rust
    /// let now: DateTime<Utc> = Utc::now()
    /// ```
    pub send_datetime: bool,
}

pub struct Drone {
    // Each frame::BufferID gets its own sequence_id
    sequence_ids: DashMap<frame::BufferID, u8>,
    sender: Sender<Vec<u8>>,
}

impl Drone {
    /// Connects to a drone
    ///
    /// * Spawns Listener at LISTENER_PORT
    /// * Perfroms Handshake at INIT_PORT
    /// * Spawns Command sender at `c2d_port`
    pub fn connect(config: Config) -> AnyResult<Self> {
        let local_ip = local_ip(config.drone_addr).ok_or_else(|| {
            anyhow!(
                "couldn't find local ip in the target network {}",
                config.drone_addr
            )
        })?;

        let local_listener = SocketAddr::new(local_ip, LISTEN_PORT);
        spawn_listener(local_listener)?;

        let init_addr = SocketAddr::new(config.drone_addr, INIT_PORT);
        let handshake_response = perform_handshake(init_addr, local_listener.port())?;

        let cmd_sender_target = SocketAddr::new(config.drone_addr, handshake_response.c2d_port);

        println!("spawning cmd sender on {}", cmd_sender_target);
        let sender = spawn_cmd_sender(local_ip, cmd_sender_target)?;

        let drone = Self {
            sequence_ids: DashMap::new(),
            sender,
        };

        if config.send_datetime {
            drone.send_datetime(Utc::now())?;
        }

        Ok(drone)
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

    pub fn send_datetime(&self, date: DateTime<Utc>) -> AnyResult<()> {
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

fn spawn_listener(addr: impl ToSocketAddrs) -> AnyResult<()> {
    let listener = UdpSocket::bind(addr)?;
    std::thread::spawn(move || loop {
        let mut buf = [0; 256];
        if let Ok((bytes_read, origin)) = listener.recv_from(&mut buf) {
            println!("Read {} bytes from {} ", bytes_read, origin.ip());
            let octal: Vec<String> = buf[0..bytes_read].iter().map(|byte| format!("{:#o}", byte)).collect();
            println!("{}", octal.join(" "));
        }
    });

    Ok(())
}

fn print_message(buf: &[u8]) {
    for b in buf.iter() {
        print!("{:#o}", b);
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
