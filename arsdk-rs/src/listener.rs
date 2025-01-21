use crate::{parse::handle_bytes, print_buf, Drone};
use log::debug;
use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, TryRecvError};

pub struct Listener {
    pub drone: Drone,
    pub socket: UdpSocket,
}

impl Listener {
    /// Blocking listener in a infinite loop
    pub fn listen(&self, shutdown_receiver: Receiver<()>) {
        while shutdown_receiver.try_recv() == Err(TryRecvError::Empty) {
            let mut buf = [0_u8; 40960];
            if let Ok((bytes_read, origin)) = self.socket.recv_from(&mut buf) {
                debug!("Received: {} bytes from {}", bytes_read, origin);
                debug!("Bytes: {}", print_buf(&buf[..bytes_read]));

                handle_bytes(&self.drone, &buf[..bytes_read]);
            }
        }

        log::error!("listener shutting down");
    }
}
