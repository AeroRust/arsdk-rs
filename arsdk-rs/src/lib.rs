use crate::frame::Frame;
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use log::info;
use pnet::datalink;
use scroll::{ctx::TryIntoCtx, Pread, LE};
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

pub type Result<T> = std::result::Result<T, MessageError>;

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
                info!("Received: {} bytes from {}", bytes_read, origin);
                info!("Bytes: {}", print_buf(&buf[..bytes_read]));


                for frame_result in parse_message_frames(&buf[..bytes_read]) {
                    let frame = match frame_result {
                        Ok(frame) => {
                            // info!("Frame: {:?}", &frame);

                            Some(frame)
                        }
                        Err(err) => {
                            info!("Unknown Frame: {}", err);
                            info!("Bytes: {}", print_buf(&buf[..bytes_read]));

                            None
                        }
                    };

                    // PING-PONG
                    match frame {
                        Some(frame) if frame.buffer_id == frame::BufferID::PING => {
                            // info!("Received PING: {:?}", frame.feature);

                            let frame_type = frame::Type::Data;
                            let buffer_id = frame::BufferID::PONG;

                            // send the same feature back
                            let pong =
                                frame::Frame::for_drone(&drone, frame_type, buffer_id, frame.feature);

                            drone.send_frame(pong).expect("Should PONG successfully!");
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    Ok(())
}

fn print_buf(buf: &[u8]) -> String {
    buf.iter()
        .map(|byte| format!("{}", byte))
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

        let frame = frame_to_send.pread_with::<Frame>(0, LE);

        match &frame {
            Ok(frame) => match frame.feature {
                Some(command::Feature::ArDrone3(_)) => {
                    info!(
                        "Sent Frame (length: {}) => {:?}",
                        frame_to_send.len(),
                        &frame
                    );
                }
                _ => {}
            }
            _ => {}
        }
        // info!(
        //     "Sent Frame (length: {}) => {:?}",
        //     frame_to_send.len(),
        //     &frame
        // );

        let size = socket
            .send_to(&frame_to_send, target_addr)
            .expect("something terrible happened");

        assert_eq!(size, frame_to_send.len())
    });

    Ok(())
}

/// we receive 2 frames sometimes
fn parse_message_frames(buf: &[u8]) -> Vec<Result<Frame>> {
    let mut offset = 0;
    // reasonable given that we receive at most (MAYBE?!) 2 messages
    let mut frames = Vec::with_capacity(3);
    while offset != buf.len() {
        let frame = buf.gread_with(&mut offset, LE);

        frames.push(frame);
    }

    frames
}
#[cfg(test)]
mod test {
    use super::*;
    use command::Feature;
    use frame::{Type, BufferID};

    #[test]
    fn receiving_two_frames_at_once() {

        let received: [u8; 58] = [
            // Type::Data = 2
            // BufferID::PING = 0
            // Sequence: 4
            // Frame size: 23
            // Feature: 9
            2, 0, 4, 23, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 57, 252, 225, 47, 0, 0, 0, 0,
            // Type::Data = 2
            // BufferID::DCNavdata = 127
            // Sequence: 71
            // Frame size: 35
            // Feature: 1
            2, 127, 71, 35, 0, 0, 0, 1, 4, 4, 0, 0, 0, 0, 0, 0, 64, 127, 64, 0, 0, 0, 0, 0, 64, 127, 64, 0, 0, 0, 0, 0, 64, 127, 64
        ];
        let expected_first_frame_length = 23; // [23, 0, 0, 0]

        let frames = parse_message_frames(&received);

        assert_eq!(2, frames.len());

        let first_expected = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::PING,
            sequence_id: 4,
            feature: Some(Feature::Unknown {
                feature: 9,
                // data starts at 8th bytes
                data: received[8..expected_first_frame_length].to_vec(),
            }),
        };

        // data starts at 8th bytes
        let second_expected = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::DCNavdata,
            sequence_id: 71,
            feature: Some(Feature::Unknown {
                feature: 1,
                data: {
                    let offset = expected_first_frame_length + 8;
                    received[offset..].to_vec()
                },
            }),
        };

        assert_eq!(&first_expected, frames[0].as_ref().expect("Should deserialize first (PING) frame"));
        assert_eq!(&second_expected, frames[1].as_ref().expect("Should deserialize second (DCNavdata) frame"));
    }

    #[test]
    #[ignore]
    fn receiving_two_frames_at_once_2() {
        let message: [u8; 46] = [
            // frist frame
            2, 0, 23, 23, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 123, 61, 147, 11, 0, 0, 0, 0,
            // second frame
            2, 127, 26, 23, 0, 0, 0, 1, 4, 5, 0, 22, 144, 125, 184, 167, 42, 112, 58, 252, 101, 132, 185];
    }
}
