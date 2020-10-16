/// u16
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GPSState {
    /// ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_NUMBEROFSATELLITECHANGED = 0
    ///
    /// > Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 2,
    /// > feature: Some(Unknown { feature: 1, data: [31, 0, 0, 12] }) }
    /// u16 => ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_NUMBEROFSATELLITECHANGED = [0, 0]
    /// u8 => _numberOfSatellite = 12
    NumberOfSatelliteChanged(u8),
    /// ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_HOMETYPEAVAILABILITYCHANGED = 1
    ///
    /// 1. Type (u32):
    /// - ARCOMMANDS_ARDRONE3_GPSSTATE_HOMETYPEAVAILABILITYCHANGED_TYPE_TAKEOFF = 0
    ///     > The drone has enough information to return to the take off position
    /// - ARCOMMANDS_ARDRONE3_GPSSTATE_HOMETYPEAVAILABILITYCHANGED_TYPE_PILOT = 1
    ///     > The drone has enough information to return to the pilot position
    /// - ARCOMMANDS_ARDRONE3_GPSSTATE_HOMETYPEAVAILABILITYCHANGED_TYPE_FIRST_FIX = 2
    ///     > The drone has not enough information, it will return to the first GPS fix
    /// - ARCOMMANDS_ARDRONE3_GPSSTATE_HOMETYPEAVAILABILITYCHANGED_TYPE_FOLLOWEE = 3
    ///     > The drone has enough information to return to the target of the current (or last) follow me
    /// - ARCOMMANDS_ARDRONE3_GPSSTATE_HOMETYPEAVAILABILITYCHANGED_TYPE_MAX
    ///  TODO: Check what the `MAX` does!
    ///
    /// Last argument is:
    /// - uint8_t *_available
    ///     > 1 if this type is available, 0 otherwise
    HomeTypeAvailabilityChanged,
    /// ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_HOMETYPECHOSENCHANGED = 2,
    HomeTypeChosenChanged,
}

mod scroll_impl {
    // TODO: impl TryFromCtx

    // TODO: impl TryIntoCtx
}
