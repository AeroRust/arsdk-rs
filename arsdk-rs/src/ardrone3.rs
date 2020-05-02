use crate::frame::Data;

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

impl Data for ArDrone3 {
    fn serialize(&self) -> Vec<u8> {
        let take_off: u16 = 1;

        take_off.to_le_bytes().to_vec()
    }
}

pub mod scroll_impl {
    use super::*;
    use scroll::{ctx, Endian, Pread};

    impl<'a> ctx::TryFromCtx<'a, Endian> for ArDrone3 {
        type Error = scroll::Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], endian: Endian) -> Result<(Self, usize), Self::Error> {
            use ArDrone3::*;

            let offset = &mut 0;

            let ardrone3 = match src.gread_with::<u16>(offset, endian)? {
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
                _ => return Err(scroll::Error::Custom("Out of range".into())),
            };

            Ok((ardrone3, *offset))
        }
    }
}
