pub(crate) mod pcmd;

use pcmd::PCMD;

/// eARCOMMANDS_ID_ARDRONE3_PILOTING_CMD
/// u16
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Piloting {
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_FLATTRIM = 0
    FlatTrim,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_TAKEOFF = 1
    TakeOff,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_PCMD = 2
    /// ARCOMMANDS_Decoder_ARDrone3PilotingPCMDCb (_flag, _roll, _pitch, _yaw, _gaz, _timestampAndSeqNum, ARCOMMANDS_Decoder_ARDrone3PilotingPCMDCustom);
    /// ARCOMMANDS_Decoder_ARDrone3PilotingPCMDDecodeArgs (uint8_t *_flag, int8_t *_roll, int8_t *_pitch, int8_t *_yaw, int8_t *_gaz, uint32_t *_timestampAndSeqNum)
    /// * @param _timestampAndSeqNum Command timestamp in milliseconds (low 24 bits) + command sequence number (high 8 bits) [0;255].
    /// 1_588_771_372_921
    /// @see https://developer.parrot.com/docs/reference/bebop_2/index.html#move-the-drone
    PCMD(PCMD),
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_LANDING = 3
    Landing,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_EMERGENCY = 4
    /// Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 0,
    /// feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [1, 0, 0, 0, 0, 0] }))) }
    Emergency,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_NAVIGATEHOME = 5
    /// requires: uint8_t _start
    /// as u8
    NavigateHome,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_AUTOTAKEOFFMODE = 6
    AutoTakeOffMode,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_MOVEBY = 7
    MoveBy,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_USERTAKEOFF = 8
    UserTakeOff,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_CIRCLE = 9
    Circle,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_MOVETO = 10
    MoveTo,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_CANCELMOVETO = 11
    CancelMoveTo,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_STARTPILOTEDPOI = 12
    StartPilotedPOI,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_STOPPILOTEDPOI = 13
    StopPilotedPOI,
}

impl Into<u16> for &Piloting {
    fn into(self) -> u16 {
        use Piloting::*;

        match self {
            FlatTrim => 0,
            TakeOff => 1,
            PCMD(_) => 2,
            Landing => 3,
            Emergency => 4,
            NavigateHome => 5,
            AutoTakeOffMode => 6,
            MoveBy => 7,
            UserTakeOff => 8,
            Circle => 9,
            MoveTo => 10,
            CancelMoveTo => 11,
            StartPilotedPOI => 12,
            StopPilotedPOI => 13,
        }
    }
}

mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Piloting {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let piloting = match src.gread_with::<u16>(&mut offset, ctx)? {
                0 => Piloting::FlatTrim,
                1 => Piloting::TakeOff,
                2 => Piloting::PCMD(src.gread_with(&mut offset, ctx)?),
                3 => Piloting::Landing,
                4 => Piloting::Emergency,
                5 => Piloting::NavigateHome,
                6 => Piloting::AutoTakeOffMode,
                7 => Piloting::MoveBy,
                8 => Piloting::UserTakeOff,
                9 => Piloting::Circle,
                10 => Piloting::MoveTo,
                11 => Piloting::CancelMoveTo,
                12 => Piloting::StartPilotedPOI,
                13 => Piloting::StopPilotedPOI,
                value => {
                    return Err(Error::OutOfBound {
                        value: value.into(),
                        param: "Piloting".to_string(),
                    })
                }
            };

            Ok((piloting, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Piloting {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u16>((&self).into(), &mut offset, ctx)?;

            match self {
                // Piloting::FlatTrim => {}
                Piloting::TakeOff => {}
                Piloting::PCMD(pcmd) => {
                    this.gwrite_with(pcmd, &mut offset, ctx)?;
                }
                // Piloting::Landing => {}
                // Piloting::Emergency => {}
                // Piloting::NavigateHome => {}
                // Piloting::AutoTakeOffMode => {}
                // Piloting::MoveBy => {}
                // Piloting::UserTakeOff => {}
                // Piloting::Circle => {}
                // Piloting::MoveTo => {}
                // Piloting::CancelMoveTo => {}
                // Piloting::StartPilotedPOI => {}
                // Piloting::StopPilotedPOI => {}
                _ => {}
            }

            Ok(offset)
        }
    }
}
