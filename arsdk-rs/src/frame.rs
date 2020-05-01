use crate::{command, Drone};
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use std::convert::TryFrom;

pub trait IntoRawFrame {
    fn into_raw(self) -> RawFrame;
}
pub struct RawFrame(pub Vec<u8>);

pub trait Data {
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct Frame {
    frame_type: Type,
    buffer_id: BufferID,
    sequence_id: u8,
    feature: command::Feature,
}

impl Frame {
    pub fn new(
        frame_type: Type,
        buffer_id: BufferID,
        sequence_id: u8,
        feature: command::Feature,
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
        feature: command::Feature,
    ) -> Frame {
        Frame::new(frame_type, buffer_id, drone.sequence_id(buffer_id), feature)
    }
}

impl IntoRawFrame for Frame {
    fn into_raw(self) -> RawFrame {
        let ser_feature = self.feature.serialize();
        // Frame size 3 bytes + 4 bytes (u32) + ser_feature.len()
        let buf_len = 7 + ser_feature.len();

        let mut buf = Vec::with_capacity(buf_len);
        buf.push(self.frame_type.into());
        buf.push(self.buffer_id.into());
        buf.push(self.sequence_id);
        // buffer size as u32 (4 bytes)
        buf.extend(&(buf_len as u32).to_le_bytes());
        buf.extend(ser_feature);

        RawFrame(buf)
    }
}

// --------------------- Types --------------------- //

#[derive(Debug, PartialEq, Clone, Copy)]
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
