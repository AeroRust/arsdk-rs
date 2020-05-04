/// eARCOMMANDS_ID_ARDRONE3_PILOTING_CMD
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ArDrone3 {
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_FLATTRIM = 0
    FlatTrim = 0,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_TAKEOFF = 1
    TakeOff = 1,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_PCMD = 2
    PCMD = 2,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_LANDING = 3
    Landing = 3,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_EMERGENCY = 4
    Emergency = 4,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_NAVIGATEHOME = 5
    NavigateHome = 5,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_AUTOTAKEOFFMODE = 6
    AutoTakeOffMode = 6,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_MOVEBY = 7
    MoveBy = 7,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_USERTAKEOFF = 8
    UserTakeOff = 8,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_CIRCLE = 9
    Circle = 9,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_MOVETO = 10
    MoveTo = 10,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_CANCELMOVETO = 11
    CancelMoveTo = 11,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_STARTPILOTEDPOI = 12
    StartPilotedPOI = 12,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_STOPPILOTEDPOI = 13
    StopPilotedPOI = 13,
}

pub mod scroll_impl {
    use super::*;
    use crate::MessageError;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for ArDrone3 {
        type Error = MessageError;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            use ArDrone3::*;

            let mut offset = 0;

            // TODO: CHeck if ArDrone3 should be read as u16!!!
            let ardrone3 = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => FlatTrim,
                1 => TakeOff,
                2 => PCMD,
                3 => Landing,
                4 => Emergency,
                5 => NavigateHome,
                6 => AutoTakeOffMode,
                7 => MoveBy,
                8 => UserTakeOff,
                9 => Circle,
                10 => MoveTo,
                11 => CancelMoveTo,
                12 => StartPilotedPOI,
                13 => StopPilotedPOI,
                value => {
                    return Err(MessageError::OutOfBound {
                        value: value.into(),
                        param: "ArDrone3".to_string(),
                    })
                }
            };

            Ok((ardrone3, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for ArDrone3 {
        type Error = MessageError;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            // TODO: Fix when we have more options
            Ok(this.pwrite_with::<u16>(self as u16, 0, ctx)?)
        }
    }
}
