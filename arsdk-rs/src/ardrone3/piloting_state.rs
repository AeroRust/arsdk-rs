/// u16
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PilotingState {
    /// typedef enum {
    ///     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLATTRIMCHANGED = 0,
    ///     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLYINGSTATECHANGED = 1,
    /// }
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLATTRIMCHANGED = 0,
    FlatTrimChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLYINGSTATECHANGED = 1,
    ///
    /// Example 1:
    /// ```bash
    /// [2020-07-26T08:05:46Z DEBUG arsdk_rs::listener] Bytes: 4 126 0 15 0 0 0 1 4 1 0 7 0 0 0
    /// [2020-07-26T08:05:46Z INFO  arsdk_rs::parse] Frame: Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [1, 0, 7, 0, 0, 0] }))) }
    /// ```
    ///
    /// Example 2:
    /// ```bash
    /// Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [1, 0, 0, 0, 0, 0] }))) }
    /// ```
    /// u16 [1, 0] - FLYINGSTATECHANGED
    FlyingStateChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ALERTSTATECHANGED = 2,
    AlterStateChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_NAVIGATEHOMESTATECHANGED = 3,
    NavigateHomeStateChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_POSITIONCHANGED = 4,
    PositionChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_SPEEDCHANGED = 5,
    ///
    /// Example:
    /// Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [5, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0] }))) }
    /// u16 [5, 0] - SPEEDCHANGED
    /// 3 x (4?) per float [0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0]
    ///
    /// ```c
    /// currIndexInBuffer = ARCOMMANDS_ReadWrite_AddU16ToBuffer (buffer, ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_SPEEDCHANGED, currIndexInBuffer, buffLen);
    /// currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedX, currIndexInBuffer, buffLen);
    /// currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedY, currIndexInBuffer, buffLen);
    /// currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedZ, currIndexInBuffer, buffLen);
    /// ```
    SpeedChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ATTITUDECHANGED = 6,
    /// Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 40, feature: Some(ArDrone3(Some(PilotingState { data: [6, 0, 44, 49, 49, 55, 153, 38, 7, 185, 107, 25, 201, 63] }))) }
    AttitudeChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_AUTOTAKEOFFMODECHANGED = 7,
    AutoTakeOffModeChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ALTITUDECHANGED = 8,
    AltitudeChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_GPSLOCATIONCHANGED = 9,
    GpsLocationChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_LANDINGSTATECHANGED = 10,
    ///
    /// ```c
    /// /*
    ///  * @brief Drone landing state
    ///  */
    /// typedef enum
    /// {
    ///     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_LINEAR = 0,    ///< Linear landing
    ///     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_SPIRAL = 1,    ///< Spiral landing
    ///     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_MAX
    /// } eARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE;
    /// ```
    LandingStateChanged,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_AIRSPEEDCHANGED = 11,
    AirspeedChanged,
    ///  ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_MOVETOCHANGED = 12,
    MoveToChanged,
    ///  ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_MOTIONSTATE = 13,
    MotionState,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_PILOTEDPOI = 14,
    PilotedPOI,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_RETURNHOMEBATTERYCAPACITY = 15,
    ReturnHomeBatteryCapacity,
    Unknown {
        piloting_state: u16,
        data: Vec<u8>,
    },
}

impl Into<u16> for &PilotingState {
    fn into(self) -> u16 {
        use PilotingState::*;

        match self {
            FlatTrimChanged => 0,
            FlyingStateChanged => 1,
            AlterStateChanged => 2,
            NavigateHomeStateChanged => 3,
            PositionChanged => 4,
            SpeedChanged => 5,
            AttitudeChanged => 6,
            AutoTakeOffModeChanged => 7,
            AltitudeChanged => 8,
            GpsLocationChanged => 9,
            LandingStateChanged => 10,
            AirspeedChanged => 11,
            MoveToChanged => 12,
            MotionState => 13,
            PilotedPOI => 14,
            ReturnHomeBatteryCapacity => 15,
            Unknown { piloting_state, .. } => *piloting_state,
        }
    }
}

pub mod scroll_impl {
    use super::*;
    use crate::{frame::Error, parse::read_unknown};
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for PilotingState {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let piloting_state = match src.gread_with::<u16>(&mut offset, ctx)? {
                unknown => Self::Unknown {
                    piloting_state: unknown,
                    data: read_unknown(src, &mut offset)?,
                },
            };

            Ok((piloting_state, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for PilotingState {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u16>((&self).into(), &mut offset, ctx)?;

            match self {
                PilotingState::Unknown { data, .. } => {
                    this.gwrite_with(data.as_slice(), &mut offset, ())?;
                }
                _ => unimplemented!("Not all PilotingState are impled!"),
            }

            Ok(offset)
        }
    }
}
