use crate::{
    command::Feature,
    frame::{BufferID, Error, Frame, Type},
    print_buf, Drone, FrameType,
};
use log::{error, info};
use scroll::{Pread, Pwrite, LE};

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
            Err(err) => error!("Receiving Frame: {:?}", err),
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

    let mut tried = 1;
    // try to read all the buf length & limit to 3 Frames of read
    while offset != buf.len() && tried <= 3 {
        let frame = buf.gread_with(&mut offset, LE);

        frames.push(frame);
        tried += 1;
    }

    frames
}

/// Helper function to read unknown / not implemented Frames
pub(crate) fn read_unknown(src: &[u8], offset: &mut usize) -> Result<Vec<u8>, crate::frame::Error> {
    let mut feature_data = [0_u8; 256];
    let actual_written = feature_data.gwrite_with(&src[*offset..], &mut 0, ())?;

    assert_eq!(actual_written, feature_data[..actual_written].len());

    *offset += actual_written;

    Ok(feature_data[..actual_written].to_vec())
}

#[cfg(test)]
mod parse_message_frames {
    use super::*;
    use crate::jumping_sumo as js;
    use crate::{
        command::Feature,
        frame::{BufferID, Frame, FrameType, Type},
    };
    #[test]
    fn test_parsable_messages() {
        let jump_message: [u8; 15] = [
            0x4, 0xb, 0x1, 0xf, 0x0, 0x0, 0x0, 0x3, 0x2, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        let jump_frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 1,
            feature: Some(Feature::JumpingSumo(js::Class::Animations(js::Anim::Jump))),
        };

        let move_message: [u8; 14] = [
            0x2, 0xa, 0x67, 0xe, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x9c,
        ];

        let pilot_state = js::PilotState {
            flag: true,
            speed: 0,
            turn: -100,
        };

        let move_frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::CDNonAck,
            sequence_id: 103,
            feature: Some(Feature::JumpingSumo(js::Class::Piloting(
                js::PilotingID::Pilot(pilot_state),
            ))),
        };

        let buf = {
            let mut vec = jump_message.to_vec();
            vec.extend_from_slice(&move_message);
            vec
        };

        let expected = [FrameType::Known(jump_frame), FrameType::Known(move_frame)];

        let actual = parse_message_frames(&buf);

        assert_eq!(expected.len(), actual.len());

        for (expected, parsed) in expected.iter().zip(actual) {
            let actual = parsed.expect("This should be Ok(_)");

            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_feature_common_none() {
        let buf: [u8; 8] = [1, 139, 0, 8, 0, 0, 0, 0];

        let frame = Frame {
            frame_type: Type::Ack,
            buffer_id: BufferID::ACKFromSendWithAck,
            sequence_id: 0,
            feature: Some(Feature::Common(None)),
        };

        let actual = parse_message_frames(&buf);

        assert_eq!(actual.len(), 1);
        let actual = actual
            .into_iter()
            .next()
            .expect("Should have 1 parsed frame")
            .expect("Should be Ok(_)");

        assert_eq!(FrameType::Known(frame), actual);
    }

    #[test]

    /// [2] Type - Data
    /// [0] BufferID - Ping
    /// [1] Sequence id 1
    /// [23,0,0,0] length 23
    /// [0, 0, 0, 0, 0, 0, 0, 233, 72, 37, 42, 0, 0, 0, 0] Ping's Gibberish Data
    ///
    /// Second:
    /// [4] Type - DataWithAck
    /// [126] BufferID - DCEvent
    /// [1] Sequence id 1
    /// [12, 0, 0, 0] Length 12
    /// [0] Feature - Common
    /// [14] Class - CalibrationState
    /// [1, 0, 0] - x y z axis
    ///
    fn test_two_frames_ping_and_common_class_calibration_state_unknown() {
        let buf: [u8; 35] = [
            // first:
            2, 0, 1, 23, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 233, 72, 37, 42, 0, 0, 0, 0,
            // second:
            4, 126, 1, 12, 0, 0, 0, 0, 14, 1, 0, 0,
        ];

        let expected = [
            FrameType::Known(Frame {
                frame_type: Type::Data,
                buffer_id: BufferID::PING,
                sequence_id: 1,
                feature: Some(Feature::Unknown {
                    feature: 3,
                    data: vec![0, 0, 0, 0, 0, 0, 0, 233, 72, 37, 42, 0, 0, 0, 0],
                }),
            }),
            FrameType::Known(Frame {
                frame_type: Type::DataWithAck,
                buffer_id: BufferID::DCEvent,
                sequence_id: 1,
                feature: Some(Feature::Common(Some(crate::common::Class::Unknown {
                    // CalibrationState
                    class: 14,
                    // x y z axis
                    data: vec![1, 0, 0],
                }))),
            }),
        ];

        let actual = parse_message_frames(&buf);
        assert_eq!(expected.len(), actual.len());

        for (expected, parsed) in expected.iter().zip(actual) {
            let actual = parsed.expect("This should be Ok(_)");

            assert_eq!(expected, &actual);
        }
    }
}
