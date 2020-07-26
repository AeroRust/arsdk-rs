use crate::{
    command::Feature,
    frame::{BufferID, Error, Frame, Type},
    print_buf, Drone, FrameType,
};
use log::{error, info};
use scroll::{Pread, LE};

/// - Parses Frames
/// - Sends PING response to cmd Sender
/// - Logs unknown frames
pub(crate) fn handle_bytes(drone: &Drone, raw_frames: &[u8]) {
    let frames = parse_message_frames(&raw_frames);

    for result in frames.iter() {
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
        match result {
            // PING-PONG
            Ok(FrameType::Known(frame)) if frame.buffer_id == BufferID::PING => {
                if let Err(err) = drone.send_pong(frame.feature.clone()) {
                    error!("Sending Frame to Commander: {}", err)
                }
            }
            // Data that needs Ack coming from the Drone
            Ok(FrameType::Known(frame)) if frame.buffer_id == BufferID::DCNavdata => {
                let ack_type = Type::Ack;
                let ack_buffer = BufferID::CDAck;
                let ack_sequence_id = frame.sequence_id;
                let feature = frame.feature.as_ref().and_then(|feature| {
                    match feature {
                        // we return an empty ArDrone3 as the drone does for our Frames
                        Feature::ArDrone3(_) => Some(Feature::ArDrone3(None)),
                        //
                        _ => None,
                    }
                });

                let ack_frame = Frame {
                    frame_type: ack_type,
                    buffer_id: ack_buffer,
                    feature,
                    sequence_id: ack_sequence_id,
                };
                if let Err(err) = drone.send_frame(ack_frame) {
                    error!("Sending Frame to Commander: {}", err)
                }
            }
            // we handled the error cases in the upper portion of the function
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
