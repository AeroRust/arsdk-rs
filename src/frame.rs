use crate::command;
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use std::convert::TryFrom;

pub trait IntoRawFrame {
    fn into_raw(self) -> RawFrame;
}
pub struct RawFrame(pub Vec<u8>);

pub trait Data {
    fn serialize(&self) -> Vec<u8>;
}

pub struct Frame {
    frame_type: Type,
    id: ID,
    sequence_id: u8,
    feature: command::Feature,
}

impl Frame {
    pub fn new(frame_type: Type, id: ID, feature: command::Feature, sequence_id: u8) -> Self {
        Self {
            frame_type,
            id,
            sequence_id,
            feature,
        }
    }
}

impl IntoRawFrame for Frame {
    fn into_raw(self) -> RawFrame {
        // Frame size without data
        let mut buf = Vec::with_capacity(10);
        buf.push(self.frame_type.into());
        buf.push(self.id.into());
        buf.push(self.sequence_id);
        // frame size as u32
        buf.extend(&[0; 4]);
        buf.extend(self.feature.serialize());

        // frame size as u32
        let buf_size = buf.len() as u32;
        buf[5] = (buf_size >> 24) as u8;
        buf[4] = (buf_size >> 16) as u8;
        buf[3] = (buf_size >> 8) as u8;
        buf[6] = (buf_size) as u8;

        RawFrame(buf)
    }
}

// --------------------- Types --------------------- //

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Uninitialized, // ARNETWORKAL_FRAME_TYPE_UNINITIALIZED
    Ack,           // ARNETWORKAL_FRAME_TYPE_ACK
    Data,          // ARNETWORKAL_FRAME_TYPE_DATA
    LowLatency,    // ARNETWORKAL_FRAME_TYPE_DATA_LOW_LATENCY
    DataWithAck,   // ARNETWORKAL_FRAME_TYPE_DATA_WITH_ACK
    Max,           // ARNETWORKAL_FRAME_TYPE_MAX
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ID {
    CDNonAck,    //#define BD_NET_CD_NONACK_ID 10
    CDAck,       //#define BD_NET_CD_ACK_ID 11
    CDEmergency, // #define BD_NET_CD_EMERGENCY_ID 12
    CDVideoAck,  // #define BD_NET_CD_VIDEO_ACK_ID 13
    DCVideo,     // #define BD_NET_DC_VIDEO_DATA_ID 125
    DCEvent,     // #define BD_NET_DC_EVENT_ID 126
    DCNavdata,   // #define BD_NET_DC_NAVDATA_ID 127
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

impl TryFrom<u8> for ID {
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

impl Into<u8> for ID {
    fn into(self) -> u8 {
        match self {
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
    use crate::jumping_sumo::*;

    use std::convert::TryInto;

    #[test]
    fn test_jumpingsumo_move_command() {
        let expected_message = "0x2 0xa 0x67 0x0 0x0 0x0 0xe 0x3 0x0 0x0 0x0 0x1 0x0 0x9c";

        let pilot_state = PilotState {
            flag: 1,
            speed: 0,
            turn: -100,
        };

        let frame = Frame {
            frame_type: Type::Data,
            id: ID::CDNonAck,
            sequence_id: 103,
            feature: command::Feature::JumpingSumo(Class::Piloting(PilotingID::Pilot(pilot_state))),
        };

        let buf = frame.into_raw().0;

        let actual_message = buf
            .iter()
            .map(|b| format!("0x{:x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(expected_message, actual_message);
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
        assert_command(ID::CDNonAck, 10);
        assert_command(ID::CDAck, 11);
        assert_command(ID::CDEmergency, 12);
        assert_command(ID::CDVideoAck, 13);
        assert_command(ID::DCVideo, 125);
        assert_command(ID::DCEvent, 126);
        assert_command(ID::DCNavdata, 127);
    }

    fn assert_frame(t: Type, v: u8) {
        assert_eq!(t, v.try_into().unwrap());
        let as_u8: u8 = t.into();
        assert_eq!(v, as_u8);
    }

    fn assert_command(c: ID, v: u8) {
        assert_eq!(c, v.try_into().unwrap());
        let as_u8: u8 = c.into();
        assert_eq!(v, as_u8);
    }
}
