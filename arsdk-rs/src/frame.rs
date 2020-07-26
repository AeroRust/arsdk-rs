use crate::{command, Drone};
use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use std::convert::TryFrom;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameType {
    Known(Frame),
    Unknown(UnknownFrame),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Unknown Frame
pub struct UnknownFrame {
    key: String,
    value: u64,
    data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub frame_type: Type,
    pub buffer_id: BufferID,
    pub sequence_id: u8,
    /// Example of empty feature:
    /// ```bash
    /// [2020-07-25T18:51:13Z DEBUG arsdk_rs] Bytes: 1 139 0 8 0 0 0 1
    /// [2020-07-25T18:51:13Z INFO  arsdk_rs::parse] Frame: Frame { frame_type: Ack, buffer_id: ACKFromSendWithAck, sequence_id: 0, feature: Some(ArDrone3(None)) }
    /// ```
    pub feature: Option<command::Feature>,
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
        Frame::new(
            frame_type,
            buffer_id,
            drone.inner.sequence_id(buffer_id),
            feature,
        )
    }
}

// --------------------- Types --------------------- //
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    /// ARNETWORKAL_FRAME_TYPE_UNINITIALIZED 0
    Uninitialized = 0,
    /// ARNETWORKAL_FRAME_TYPE_ACK 1
    Ack = 1,
    /// ARNETWORKAL_FRAME_TYPE_DATA 2
    Data = 2,
    /// ARNETWORKAL_FRAME_TYPE_DATA_LOW_LATENCY 3
    LowLatency = 3,
    /// ARNETWORKAL_FRAME_TYPE_DATA_WITH_ACK 4
    DataWithAck = 4,
    /// ARNETWORKAL_FRAME_TYPE_MAX 5
    Max = 5,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type::")?;

        match self {
            Self::Uninitialized => write!(f, "Uninitialized"),
            Self::Ack => write!(f, "Ack"),
            Self::Data => write!(f, "Data"),
            Self::LowLatency => write!(f, "LowLatency"),
            Self::DataWithAck => write!(f, "DataWithAck"),
            Self::Max => write!(f, "Max"),
        }
    }
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
    /// @TODO: Find the corresponding C definition if there is one and name the new enum variant!
    /// PyParrot:
    /// `'ACK_FROM_SEND_WITH_ACK': 139  # 128 + buffer id for 'SEND_WITH_ACK' is 139`
    /// Type = 1
    /// BufferId = 139
    /// Sequence = 1
    /// length = 8
    /// Feature = 1
    /// 1 139 1 8 0 0 0 1
    ACKFromSendWithAck = 139,
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
            139 => Ok(Self::ACKFromSendWithAck),
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
            Self::ACKFromSendWithAck => 139,
        }
    }
}

impl fmt::Display for BufferID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BufferID::")?;

        match self {
            Self::PING => write!(f, "PING"),
            Self::PONG => write!(f, "PONG"),
            Self::CDNonAck => write!(f, "CDNonAck"),
            Self::CDAck => write!(f, "CDAck"),
            Self::CDEmergency => write!(f, "CDEmergency"),
            Self::CDVideoAck => write!(f, "CDVideoAck"),
            Self::DCVideo => write!(f, "DCVideo"),
            Self::DCEvent => write!(f, "DCEvent"),
            Self::DCNavdata => write!(f, "DCNavdata"),
            Self::ACKFromSendWithAck => write!(f, "ACKFromSendWithAck"),
        }
    }
}

pub mod impl_scroll {
    use super::*;
    use crate::command::Feature;

    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for FrameType {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut actual_buf_len = 0;

            match src.gread_with(&mut actual_buf_len, ctx) {
                Ok(frame) => Ok((FrameType::Known(frame), actual_buf_len)),
                Err(Error::OutOfBound { param, value }) => {
                    let mut data = [0_u8; 256];
                    let actual_written = data.gwrite_with(&src[actual_buf_len..], &mut 0, ())?;

                    assert_eq!(actual_written, data[..actual_written].len());

                    actual_buf_len += actual_written;
                    let unknown_frame = FrameType::Unknown(UnknownFrame {
                        key: param,
                        value,
                        data: data[..actual_written].to_vec(),
                    });
                    Ok((unknown_frame, actual_buf_len))
                }
                Err(err) => Err(err),
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for Frame {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut actual_buf_len = 0;
            let frame_type = src.gread_with(&mut actual_buf_len, ctx)?;
            let buffer_id = src.gread_with(&mut actual_buf_len, ctx)?;
            let sequence_id = src.gread_with(&mut actual_buf_len, ctx)?;
            let buf_len: u32 = src.gread_with(&mut actual_buf_len, ctx)?;

            // TODO: Fix this as it might fail, use TryFrom<u32>
            let buf_len_usize = buf_len as usize;

            let feature = if buf_len > 7 {
                // we can receive multiple frames, so the feature should be limited
                // to buf_len from source

                let feature =
                    src[..buf_len_usize].gread_with::<Feature>(&mut actual_buf_len, ctx)?;

                Some(feature)
            } else {
                None
            };

            if buf_len_usize != actual_buf_len {
                Err(Error::BytesLength {
                    expected: buf_len,
                    // @TODO: actual_buf_len as u32 can fail (TryFrom is impled for usize)
                    actual: actual_buf_len as u32,
                })
            } else {
                Ok((
                    Frame {
                        frame_type,
                        buffer_id,
                        sequence_id,
                        feature,
                    },
                    actual_buf_len,
                ))
            }
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Frame {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u8>(self.frame_type.into(), &mut offset, ctx)?;
            this.gwrite_with::<u8>(self.buffer_id.into(), &mut offset, ctx)?;
            this.gwrite_with::<u8>(self.sequence_id, &mut offset, ctx)?;

            let buf_length_offset = offset;
            // reserve bytes for the buffer length (u32)
            this.gwrite_with::<u32>(0, &mut offset, ctx)?;

            if let Some(feature) = self.feature {
                this.gwrite_with::<Feature>(feature, &mut offset, ctx)?;
            };

            // 7 bytes + feature_length bytes = buf.length
            this.pwrite_with::<u32>(offset as u32, buf_length_offset, ctx)?;

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for Type {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], _ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let frame_value = src.gread::<u8>(&mut offset)?;

            let frame_type = match frame_value {
                0 => Self::Uninitialized,
                1 => Self::Ack,
                2 => Self::Data,
                3 => Self::LowLatency,
                4 => Self::DataWithAck,
                5 => Self::Max,
                value => {
                    return Err(Error::OutOfBound {
                        value: value.into(),
                        param: "Type".to_string(),
                    })
                }
            };

            Ok((frame_type, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Type {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            Ok(this.pwrite_with::<u8>(self.into(), 0, ctx)?)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for BufferID {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], _ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let id_value = src.gread::<u8>(&mut offset)?;

            let buffer_id = match id_value {
                0 => Self::PING,
                1 => Self::PONG,
                10 => Self::CDNonAck,
                11 => Self::CDAck,
                12 => Self::CDEmergency,
                13 => Self::CDVideoAck,
                125 => Self::DCVideo,
                126 => Self::DCEvent,
                127 => Self::DCNavdata,
                139 => Self::ACKFromSendWithAck,
                value => {
                    return Err(Error::OutOfBound {
                        value: value.into(),
                        param: "BufferID".to_string(),
                    })
                }
            };

            Ok((buffer_id, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for BufferID {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            Ok(this.pwrite_with::<u8>(self.into(), 0, ctx)?)
        }
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod frame_tests {
    use super::*;
    use crate::{
        common::{self, Class as CommonClass},
        jumping_sumo::*,
    };
    use chrono::{TimeZone, Utc};
    use scroll::{Pread, Pwrite, LE};

    use std::convert::TryInto;

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

        let frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::CDNonAck,
            sequence_id: 103,
            feature: Some(command::Feature::JumpingSumo(Class::Piloting(
                PilotingID::Pilot(pilot_state),
            ))),
        };

        assert_frames_match(&message, frame)
    }

    #[test]
    #[ignore]
    fn test_common_date_command() {
        let message = [
            0x4, 0xb, 0x1, 0x15, 0x0, 0x0, 0x0, 0x4, 0x1, 0x0, 0x32, 0x30, 0x32, 0x30, 0x2d, 0x30,
            0x34, 0x2d, 0x32, 0x36, 0x0,
        ];

        let date = Utc.ymd(2020, 04, 26).and_hms(15, 06, 11);

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 1,
            feature: Some(command::Feature::Common(CommonClass::Common(
                common::Common::CurrentDate(date),
            ))),
        };

        assert_frames_match(&message, frame);
    }

    #[test]
    #[ignore]
    fn test_common_time_command() {
        let message: [u8; 22] = [
            0x4, 0xb, 0x2, 0x15, 0x0, 0x0, 0x0, // Feature::Common
            0x0, // common::Class::Common
            0x4, // Current time
            0x2, // 12 bytes incl nul
            0x0, 0x54, 0x31, 0x35, 0x30, 0x36, 0x31, 0x31, 0x30, 0x30, 0x30, 0x0,
        ];

        let date = Utc.ymd(2020, 04, 26).and_hms(15, 06, 11);

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 2,
            feature: Some(command::Feature::Common(CommonClass::Common(
                common::Common::CurrentTime(date),
            ))),
        };

        assert_frames_match(&message, frame);
    }

    #[test]
    fn test_jumpingsumo_move_command() {
        let message: [u8; 14] = [
            0x2, 0xa, 0x67, 0xe, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x9c,
        ];

        let pilot_state = PilotState {
            flag: true,
            speed: 0,
            turn: -100,
        };

        let frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::CDNonAck,
            sequence_id: 103,
            feature: Some(command::Feature::JumpingSumo(Class::Piloting(
                PilotingID::Pilot(pilot_state),
            ))),
        };

        assert_frames_match(&message, frame);
    }

    #[test]
    fn test_jumpingsumo_jump_command() {
        //                              type buf  seq  [         len      ] [JS  Anim Jump       DATA                 ]
        let message: [u8; 15] = [
            0x4, 0xb, 0x1, 0xf, 0x0, 0x0, 0x0, 0x3, 0x2, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        let frame = Frame {
            frame_type: Type::DataWithAck,
            buffer_id: BufferID::CDAck,
            sequence_id: 1,
            feature: Some(command::Feature::JumpingSumo(Class::Animations(Anim::Jump))),
        };

        assert_frames_match(&message, frame);
    }

    #[test]
    fn test_ping_feature_from_anafi4k() {
        let message: [u8; 15] = [2, 0, 2, 15, 0, 0, 0, 155, 216, 221, 13, 0, 0, 0, 0];
        let frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::PING,
            sequence_id: 2,
            feature: Some(command::Feature::Unknown {
                feature: 155,
                data: vec![216, 221, 13, 0, 0, 0, 0],
            }),
        };

        assert_frames_match(&message, frame);
    }

    #[test]
    #[ignore]
    // TODO: Impl CommonState!
    fn test_feature_common_state() {
        let message: [u8; 12] = [
            2, 127, 20, 12, 0, 0, 0, // common
            0, // Common State
            5, //
            1, 0, 100,
        ];

        let frame = Frame {
            frame_type: Type::Data,
            buffer_id: BufferID::DCNavdata,
            sequence_id: 20,
            feature: Some(command::Feature::Common(CommonClass::CommonState)),
        };

        assert_frames_match(&message, frame);
    }

    fn assert_frames_match(expected: &[u8], frame: Frame) {
        // Check the value at the Frame length bytes 3 to 7
        let buf_len: u32 = (&expected[3..7])
            .pread_with(0, LE)
            .expect("should read a u32");

        assert_eq!(buf_len as usize, expected.len());

        // Deserialize a Frame
        assert_eq!(
            frame,
            expected
                .pread_with::<Frame>(0, LE)
                .expect("Should deserialize"),
        );
        let mut actual = [0_u8; 256];
        assert!(
            actual.len() > buf_len as usize,
            "Whoopsy... our serialization buffer is not that big!"
        );

        let mut offset = 0;
        let actual_written = actual
            .gwrite_with(frame, &mut offset, LE)
            .expect("Should serialize");

        assert_eq!(expected, &actual[..offset]);
        assert_eq!(buf_len as usize, actual_written);
    }
    // 0x2 0xb 0x1 0xf 0x0 0x0 0x0 0x3 0x2 0x3 0x0 0x0 0x0 0x0 0x0

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
