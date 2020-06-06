use crate::frame::Frame;
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use pnet::datalink;
use scroll::Pread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{
    mpsc::{sync_channel, Receiver, SyncSender},
    Arc,
};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Message parsing error")]
    Scroll(#[from] scroll::Error),
    #[error("Out of bound value {value} for {param}")]
    OutOfBound {
        // TODO: See how should we handle this for each individual case
        // Use the largest possible value
        value: u64,
        param: String,
    },
    #[error("Expected {expected} bytes, got {actual}")]
    BytesLength { expected: u32, actual: u32 },
}

#[derive(Debug)]
pub struct Config {
    pub drone_addr: IpAddr,
    /// Wheather or not to set after connecting (by sending a frame) the current DateTime to the Drone:
    ///
    /// ```rust
    /// use chrono::{DateTime, Utc};
    /// let now: DateTime<Utc> = Utc::now();
    /// ```
    pub send_datetime: bool,
}

impl<I> From<I> for Config
where
    I: Into<IpAddr>,
{
    fn from(ip: I) -> Self {
        Self {
            drone_addr: ip.into(),
            send_datetime: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Drone {
    inner: Arc<DroneInner>,
}

#[derive(Debug)]
struct DroneInner {
    // Each frame::BufferID gets its own sequence_id
    sequence_ids: DashMap<frame::BufferID, u8>,
    sender: SyncSender<Vec<u8>>,
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

        // @TODO: Check if we're going to miss any messages between spawning the listener and the receiver of commands
        let (tx_cmd, rx_cmd) = sync_channel(200);

        let drone = Self {
            inner: Arc::new(DroneInner {
                sequence_ids: DashMap::new(),
                sender: tx_cmd,
            }),
        };

        let local_listener = SocketAddr::new(local_ip, LISTEN_PORT);
        spawn_listener(drone.clone(), local_listener)?;

        let init_addr = SocketAddr::new(config.drone_addr, INIT_PORT);
        let handshake_response = perform_handshake(init_addr, local_listener.port())?;

        let cmd_sender_target = SocketAddr::new(config.drone_addr, handshake_response.c2d_port);

        println!("spawning cmd sender on {}", cmd_sender_target);
        spawn_cmd_sender(rx_cmd, local_ip, cmd_sender_target)?;

        if config.send_datetime {
            drone.send_datetime(Utc::now())?;
        }

        Ok(drone)
    }

    pub fn send_frame(&self, frame: frame::Frame) -> AnyResult<()> {
        use scroll::{ctx::TryIntoCtx, LE};

        let mut raw_message = [0_u8; 2048];
        let written = frame.try_into_ctx(&mut raw_message, LE)?;

        self.send_raw_message(&raw_message[0..written])
    }

    pub fn send_raw_message(&self, raw_message: &[u8]) -> AnyResult<()> {
        self.inner
            .sender
            .send(raw_message.to_vec())
            .map_err(AnyError::new)
    }

    pub fn send_datetime(&self, date: DateTime<Utc>) -> AnyResult<()> {
        use command::Feature::Common;
        use common::Class;
        use frame::{BufferID, Type};

        let date_feature = Common(Class::Common(common::Common::CurrentDate(date)));

        let frame = Frame::for_drone(
            &self,
            Type::DataWithAck,
            BufferID::CDAck,
            Some(date_feature),
        );

        self.send_frame(frame)?;

        let time_feature = Common(Class::Common(common::Common::CurrentTime(date)));
        let frame = Frame::for_drone(
            &self,
            Type::DataWithAck,
            BufferID::CDAck,
            Some(time_feature),
        );

        self.send_frame(frame)
    }
}

impl DroneInner {
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

fn spawn_listener(drone: Drone, addr: impl ToSocketAddrs) -> AnyResult<()> {
    let listener = UdpSocket::bind(addr)?;
    std::thread::spawn(move || {
        let drone = drone.clone();
        loop {
            let mut buf = [0_u8; 256];
            if let Ok((bytes_read, origin)) = listener.recv_from(&mut buf) {
                if buf[1] == frame::BufferID::PING as u8 {
                    println!(
                        "Received: {} bytes from {} Bytes: {}",
                        bytes_read,
                        origin,
                        print_buf(&buf[..bytes_read - 1])
                    );

                    let frame_type = frame::Type::Data;
                    let buffer_id = frame::BufferID::PONG;

                    let pong = frame::Frame::for_drone(&drone, frame_type, buffer_id, None);

                    drone.send_frame(pong).expect("Should PONG successfully!");
                }
            }
        }
    });

    Ok(())
}

fn print_buf(buf: &[u8]) -> String {
    buf.iter()
        .map(|byte| format!("{:#x}", byte))
        .collect::<Vec<_>>()
        .join(" ")
}

fn spawn_cmd_sender(
    rx: Receiver<Vec<u8>>,
    local_ip: IpAddr,
    target_addr: SocketAddr,
) -> AnyResult<()> {
    let local_addr = SocketAddr::new(local_ip, target_addr.port());

    let socket = UdpSocket::bind(local_addr)
        .map_err(|e| anyhow!("Couldn't bind to local socket {} - {}", local_addr, e))?;

    std::thread::spawn(move || loop {
        let frame_to_send = rx.recv().expect("couldn't receive frame.");

        use scroll::LE;
        let frame = frame_to_send.pread_with::<Frame>(0, LE);

        println!(
            "Sent Frame (length: {}) => {:#?}",
            frame_to_send.len(),
            &frame
        );

        let size = socket
            .send_to(&frame_to_send, target_addr)
            .expect("something terrible happened");

        assert_eq!(size, frame_to_send.len())
    });

    Ok(())
}
