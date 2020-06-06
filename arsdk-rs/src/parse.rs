use crate::{
    frame::{BufferID, Error},
    print_buf, Drone, FrameType,
};
use log::{error, info};
use scroll::{Pread, LE};

/// - Parses Frames
/// - Sends PING response to cmd Sender
/// - Logs unknown frames
pub(crate) fn handle_bytes(drone: &Drone, raw_frames: &[u8]) {
    let frames = parse_message_frames(&raw_frames);
    let received: Vec<_> = parse_message_frames(&raw_frames);

    for result in received.iter() {
        match result {
            Ok(FrameType::Known(frame)) => info!("Frame: {:?}", frame),
            Ok(FrameType::Unknown(unknown)) => {
                info!("Unknown Frame: {:?}", unknown);
                info!("Bytes: {}", print_buf(raw_frames));
            }
            Err(err) => error!("Receiving Frame: {}", err),
        }
    }

    for result in frames.iter() {
        // PING-PONG
        match result {
            Ok(FrameType::Known(frame)) if frame.buffer_id == BufferID::PING => {
                if let Err(err) = drone.send_pong(frame.feature.clone()) {
                    error!("Sending Frame to Commander: {}", err)
                }
            }
            Err(err) => error!("Sending Frame: {}", err),
            _ => {}
        }
    }
}

/// Parses the Frames from a buffer
pub(crate) fn parse_message_frames(buf: &[u8]) -> Vec<Result<FrameType, Error>> {
    let mut offset = 0;
    // TODO: Check how many frames can we receive at once
    // reasonable given that we receive at most (MAYBE?!) 2 frames
    let mut frames = Vec::with_capacity(3);
    while offset != buf.len() {
        let frame = buf.gread_with(&mut offset, LE);

        frames.push(frame);
    }

    frames
}
