use crate::{parse::handle_bytes, print_buf, Drone};
use log::debug;
use std::net::UdpSocket;

pub struct Listener {
    pub drone: Drone,
    pub socket: UdpSocket,
}

impl Listener {
    /// Blocking listener in a infinite loop
    pub fn listen(&self) {
        loop {
            let mut buf = [0_u8; 256];
            if let Ok((bytes_read, origin)) = self.socket.recv_from(&mut buf) {
                debug!("Received: {} bytes from {}", bytes_read, origin);
                debug!("Bytes: {}", print_buf(&buf[..bytes_read]));

                handle_bytes(&self.drone, &buf[..bytes_read]);
            }
        }
    }
}
