
use crate::frame::Data;

/// eARCOMMANDS_ID_ARDRONE3_PILOTING_CMD
#[repr(u8)]
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
    StartPilotdPOI = 12,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_STOPPILOTEDPOI = 13
    StopPilotedPOI = 13,
}


impl Data for ArDrone3 {
    fn serialize(&self) -> Vec<u8> {
        // todo: Fix this hardcoded value

        let take_off: u16 = 1;

        take_off.to_be_bytes().to_vec()
    }

}
