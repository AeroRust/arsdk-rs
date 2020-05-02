use crate::{command, Drone};
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use std::{convert::TryFrom};

pub trait Data {
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    frame_type: Type,
    buffer_id: BufferID,
    sequence_id: u8,
    feature: Option<command::Feature>,
}

impl Frame {
    pub fn new(
        frame_type: Type,
        buffer_id: BufferID,
        sequence_id: u8,
        feature: Option<command::Feature>,
    ) -> Self {
        Self {
            frame_type,
            buffer_id,
            sequence_id,
            feature,
        }
    }

    pub fn for_drone(
        drone: &Drone,
        frame_type: Type,
        buffer_id: BufferID,
        feature: Option<command::Feature>,
    ) -> Frame {
        Frame::new(frame_type, buffer_id, drone.inner.sequence_id(buffer_id), feature)
    }
}

// impl IntoRawMessage for Frame {
//     fn into_raw(self) -> RawMessage {
//         let ser_feature = self.feature.map(|f| f.serialize());
//         // Frame size 3 bytes + 4 bytes (u32) + ser_feature.len()
//         let buf_len = 7 + ser_feature.as_ref().map(|f| f.len()).unwrap_or_default();

//         let mut buf = Vec::with_capacity(buf_len);
//         buf.push(self.frame_type.into());
//         buf.push(self.buffer_id.into());
//         buf.push(self.sequence_id);
//         // buffer size as u32 (4 bytes)
//         buf.extend(&(buf_len as u32).to_le_bytes());

//         if let Some(feature) = ser_feature {
//             buf.extend(feature);
//         }

//         RawMessage(buf)
//     }
// }

// --------------------- Types --------------------- //

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Uninitialized = 0, // ARNETWORKAL_FRAME_TYPE_UNINITIALIZED 0
    Ack = 1,           // ARNETWORKAL_FRAME_TYPE_ACK 1
    Data = 2,          // ARNETWORKAL_FRAME_TYPE_DATA 2
    LowLatency = 3,    // ARNETWORKAL_FRAME_TYPE_DATA_LOW_LATENCY 3
    DataWithAck = 4,   // ARNETWORKAL_FRAME_TYPE_DATA_WITH_ACK 4
    Max = 5,           // ARNETWORKAL_FRAME_TYPE_MAX 5
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BufferID {
    /// pings from device
    PING = 0,
    /// respond to pings
    PONG = 1,
    /// C:
    /// #define BD_NET_CD_NONACK_ID 10
    ///
    /// PyParrot:
    /// 'SEND_NO_ACK': 10, # not-ack commandsandsensors (piloting and camera rotations)
    CDNonAck = 10,
    /// C:
    /// `#define BD_NET_CD_ACK_ID 11`
    ///
    /// PyParrot:
    /// 'SEND_WITH_ACK': 11, # ack commandsandsensors (all piloting commandsandsensors)
    CDAck = 11,
    /// C:
    /// `#define BD_NET_CD_EMERGENCY_ID 12`
    ///
    /// PyParrot:
    /// `'SEND_HIGH_PRIORITY': 12, # emergency commandsandsensors`
    CDEmergency = 12,
    /// C:
    /// #define BD_NET_CD_VIDEO_ACK_ID 13
    ///
    /// PyParrot:
    /// `'VIDEO_ACK': 13, # ack for video`
    CDVideoAck = 13,
    /// C:
    /// #define BD_NET_DC_VIDEO_DATA_ID 125
    ///
    /// PyParrot:
    /// `'VIDEO_DATA' : 125, # video data`
    DCVideo = 125,
    /// C:
    /// #define BD_NET_DC_EVENT_ID 126
    ///
    /// PyParrot:
    // 'NO_ACK_DRONE_DATA' : 126, # data from drone (including battery and others), no ack
    ///
    DCEvent = 126,
    /// C:
    /// #define BD_NET_DC_NAVDATA_ID 127
    ///
    /// PyParrot:
    /// `'ACK_DRONE_DATA' : 127, # drone data that needs an ack`
    DCNavdata = 127,
    // @TODO: Find the corresponding C definition if there is one and name the new enum variant!
    // PyParrot:
    // `'ACK_FROM_SEND_WITH_ACK': 139  # 128 + buffer id for 'SEND_WITH_ACK' is 139`
    // ACKFromSendWithAck = 139,
}

// --------------------- Conversion impls --------------------- //
impl TryFrom<u8> for Type {
    type Error = AnyError;
    fn try_from(v: u8) -> AnyResult<Self> {
        match v {
            0 => Ok(Self::Uninitialized),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Data),
            3 => Ok(Self::LowLatency),
            4 => Ok(Self::DataWithAck),
            5 => Ok(Self::Max),
            _ => Err(anyhow!("{} is not a valid Type variant", v)),
        }
    }
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        match self {
            Self::Uninitialized => 0,
            Self::Ack => 1,
            Self::Data => 2,
            Self::LowLatency => 3,
            Self::DataWithAck => 4,
            Self::Max => 5,
        }
    }
}

impl TryFrom<u8> for BufferID {
    type Error = AnyError;
    fn try_from(v: u8) -> AnyResult<Self> {
        match v {
            0 => Ok(Self::PING),
            1 => Ok(Self::PONG),
            10 => Ok(Self::CDNonAck),
            11 => Ok(Self::CDAck),
            12 => Ok(Self::CDEmergency),
            13 => Ok(Self::CDVideoAck),
            125 => Ok(Self::DCVideo),
            126 => Ok(Self::DCEvent),
            127 => Ok(Self::DCNavdata),
            _ => Err(anyhow!("{} is not a valid frame ID variant", v)),
        }
    }
}

impl Into<u8> for BufferID {
    fn into(self) -> u8 {
        match self {
            Self::PING => 0,
            Self::PONG => 1,
            Self::CDNonAck => 10,
            Self::CDAck => 11,
            Self::CDEmergency => 12,
            Self::CDVideoAck => 13,
            Self::DCVideo => 125,
            Self::DCEvent => 126,
            Self::DCNavdata => 127,
        }
    }
}

pub mod impl_scroll {
    use super::*;
    use crate::{MessageError, command::Feature};

    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Frame {
        type Error = MessageError;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], endian: Endian) -> Result<(Self, usize), Self::Error> {
            let offset = &mut 0;

            let frame_type = src.gread_with(offset, endian)?;
            let buffer_id = src.gread_with(offset, endian)?;
            let sequence_id = src.gread_with(offset, endian)?;
            let buf_len: u32 = src.gread_with(offset, endian)?;

            let feature = if buf_len > 7 {
                Some(src.gread_with(offset, endian)?)
            }else {
                None
            };

            // @TODO: offset as u32 can fail (TryFrom is impled for usize)
            if buf_len != *offset as u32 {
                return Err(Self::Error::BytesLength {
                    expected: buf_len,
                    actual: *offset as u32,
                });
            }

            Ok((
                Frame {
                    frame_type,
                    buffer_id,
                    sequence_id,
                    feature,
                },
                *offset,
            ))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Frame {
        type Error = scroll::Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let offset = &mut 0;

            this.gwrite_with::<u8>(self.frame_type.into(), offset, ctx)?;
            this.gwrite_with::<u8>(self.buffer_id.into(), offset, ctx)?;
            this.gwrite_with::<u8>(self.sequence_id, offset, ctx)?;

            let buf_length_offset = *offset;
            // reserve bytes for the buffer length (u32)
            this.gwrite_with::<u32>(0, offset, ctx)?;

            let feature_length = match self.feature {
                Some(feature) => this.gwrite_with::<Feature>(feature, offset, ctx)?,
                None => 0
            };

            // 7 bytes + feature_length bytes = buf.length
            let written = 7 + feature_length;
            this.pwrite_with::<u32>(written as u32, buf_length_offset, ctx)?;

            Ok(written)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for Type {
        type Error = scroll::Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], _endian: Endian) -> Result<(Self, usize), Self::Error> {
            let offset = &mut 0;
            let frame_value = src.gread::<u8>(offset)?;

            Type::try_from(frame_value)
                .map(|frame_type| (frame_type, *offset))
                .map_err(|err| scroll::Error::Custom(err.to_string()))
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for BufferID {
        type Error = scroll::Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], _endian: Endian) -> Result<(Self, usize), Self::Error> {
            let offset = &mut 0;
            let id_value = src.gread::<u8>(offset)?;

            BufferID::try_from(id_value)
                .map(|buffer_id| (buffer_id, *offset))
                .map_err(|err| scroll::Error::Custom(err.to_string()))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::jumping_sumo::*;
        use scroll::{Pwrite, LE};

        #[test]
        fn test_full_frame() {
            let message: [u8; 14] = [
                0x2, 0xa, 0x67, 0xe, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x9c,
            ];

            let pilot_state = PilotState {
                flag: true,
                speed: 0,
                turn: -100,
            };

            let expected_frame = Frame {
                frame_type: Type::Data,
                buffer_id: BufferID::CDNonAck,
                sequence_id: 103,
                feature: command::Feature::JumpingSumo(Class::Piloting(PilotingID::Pilot(
                    pilot_state,
                ))),
            };

            let actual_frame: Frame = message.pread_with(0, LE).unwrap();

            assert_eq!(expected_frame, actual_frame);

            let mut actual_message: [u8; 14] = [0; 14];
            actual_message
                .pwrite_with(actual_frame, 0, LE)
                .expect("whoopsy");

            assert_eq!(message, actual_message)
        }
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod frame_tests {
    use super::*;
    use crate::common::{self, Class as CommonClass};
    use crate::jumping_sumo::*;
    use chrono::{TimeZone, Utc};

    use std::convert::TryInto;

    #[test]
    fn test_common_date_command() {
        let expected_message = "0x4 0xb 0x1 0x15 0x0 0x0 0x0 0x4 0x1 0x0 0x32 0x30 0x32 0x30 0x2d 0x30 0x34 0x2d 0x32 0x36 0x0";

        let date = Utc.ymd(2020, 04, 26).and_hms(15, 06, 11);

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 1,
            feature: command::Feature::Common(CommonClass::Common(common::Common::CurrentDate(
                date,
            ))),
        };

        assert_frames_match(expected_message, frame);
    }

    #[test]
    fn test_common_time_command() {
        let expected_message = "0x4 0xb 0x2 0x15 0x0 0x0 0x0 0x4 0x2 0x0 0x54 0x31 0x35 0x30 0x36 0x31 0x31 0x30 0x30 0x30 0x0";

        let date = Utc.ymd(2020, 04, 26).and_hms(15, 06, 11);

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 2,
            feature: command::Feature::Common(CommonClass::Common(common::Common::CurrentTime(
                date,
            ))),
        };

        assert_frames_match(expected_message, frame);
    }

    #[test]
    fn test_jumpingsumo_move_command() {
        let expected_message = "0x2 0xa 0x67 0xe 0x0 0x0 0x0 0x3 0x0 0x0 0x0 0x1 0x0 0x9c";

        let pilot_state = PilotState {
            flag: true,
            speed: 0,
            turn: -100,
        };

        let frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::CDNonAck,
            sequence_id: 103,
            feature: command::Feature::JumpingSumo(Class::Piloting(PilotingID::Pilot(pilot_state))),
        };

        assert_frames_match(expected_message, frame);
    }

    #[test]
    fn test_jumpingsumo_jump_command() {
        let expected_message = "0x4 0xb 0x1 0xf 0x0 0x0 0x0 0x3 0x2 0x3 0x0 0x0 0x0 0x0 0x0";

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 1,
            feature: command::Feature::JumpingSumo(Class::Animations(Anim::Jump)),
        };

        assert_frames_match(expected_message, frame);
    }

    // 0x2 0xb 0x1 0xf 0x0 0x0 0x0 0x3 0x2 0x3 0x0 0x0 0x0 0x0 0x0

    fn assert_frames_match(output: &str, frame: Frame) {
        let buf = frame.into_raw().0;

        let actual_message = buf
            .iter()
            .map(|b| format!("0x{:x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(output, actual_message);
    }

    #[test]
    fn test_frame() {
        assert_frame(Type::Uninitialized, 0);
        assert_frame(Type::Ack, 1);
        assert_frame(Type::Data, 2);
        assert_frame(Type::LowLatency, 3);
        assert_frame(Type::DataWithAck, 4);
        assert_frame(Type::Max, 5);
    }

    #[test]
    fn test_command() {
        assert_command(BufferID::CDNonAck, 10);
        assert_command(BufferID::CDAck, 11);
        assert_command(BufferID::CDEmergency, 12);
        assert_command(BufferID::CDVideoAck, 13);
        assert_command(BufferID::DCVideo, 125);
        assert_command(BufferID::DCEvent, 126);
        assert_command(BufferID::DCNavdata, 127);
    }

    fn assert_frame(t: Type, v: u8) {
        assert_eq!(t, v.try_into().unwrap());
        let as_u8: u8 = t.into();
        assert_eq!(v, as_u8);
    }

    fn assert_command(c: BufferID, v: u8) {
        assert_eq!(c, v.try_into().unwrap());
        let as_u8: u8 = c.into();
        assert_eq!(v, as_u8);
    }
}
