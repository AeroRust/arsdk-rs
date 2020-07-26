use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
/// u8
pub enum ArDrone3 {
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTING = 0
    Piloting(Piloting),
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_CAMERA = 1
    Camera,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTINGSETTINGS = 2
    PilotingSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIARECORDEVENT = 3
    MediaRecordEvent,
    // /**
    //  * @brief Drone landing state
    //  */
    // typedef enum
    // {
    //     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_LINEAR = 0,    ///< Linear landing
    //     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_SPIRAL = 1,    ///< Spiral landing
    //     ARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE_MAX
    // } eARCOMMANDS_ARDRONE3_PILOTINGSTATE_LANDINGSTATECHANGED_STATE;
    //
    // currIndexInBuffer = ARCOMMANDS_ReadWrite_AddU16ToBuffer (buffer, ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_SPEEDCHANGED, currIndexInBuffer, buffLen);
    // currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedX, currIndexInBuffer, buffLen);
    // currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedY, currIndexInBuffer, buffLen);
    // currIndexInBuffer = ARCOMMANDS_ReadWrite_AddFloatToBuffer (buffer, _speedZ, currIndexInBuffer, buffLen);
    //
    // typedef enum {
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLATTRIMCHANGED = 0,
    //
    //
    //
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_FLYINGSTATECHANGED = 1,
    // [2020-07-26T08:05:46Z DEBUG arsdk_rs::listener] Bytes: 4 126 0 15 0 0 0 1 4 1 0 7 0 0 0
    // [2020-07-26T08:05:46Z INFO  arsdk_rs::parse] Frame: Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [1, 0, 7, 0, 0, 0] }))) }
    //
    // Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [1, 0, 0, 0, 0, 0] }))) }
    // u16 [1, 0] - FLYINGSTATECHANGED
    //
    // ----------------------------------------------------------------------
    //
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ALERTSTATECHANGED = 2,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_NAVIGATEHOMESTATECHANGED = 3,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_POSITIONCHANGED = 4,
    //
    //
    //
    //----------------------------------------------------------------------
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_SPEEDCHANGED = 5,
    // Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 0, feature: Some(ArDrone3(Some(Unknown { ardrone3: 4, data: [5, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0] }))) }
    // u16 [5, 0] - SPEEDCHANGED
    // 3 x (4?) per float [0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0]
    //
    //----------------------------------------------------------------------
    //
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ATTITUDECHANGED = 6,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_AUTOTAKEOFFMODECHANGED = 7,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_ALTITUDECHANGED = 8,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_GPSLOCATIONCHANGED = 9,
    //
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_LANDINGSTATECHANGED = 10,
    // u16 [1, 0] - LANDINGSTATECHANGED
    // u32 [0, 0, 0, 0] -
    //
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_AIRSPEEDCHANGED = 11,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_MOVETOCHANGED = 12,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_MOTIONSTATE = 13,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_PILOTEDPOI = 14,
    //     ARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD_RETURNHOMEBATTERYCAPACITY = 15,
    // } eARCOMMANDS_ID_ARDRONE3_PILOTINGSTATE_CMD;
    //
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTINGSTATE = 4
    PilotingState {
        data: Vec<u8>,
    },
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ANIMATIONS = 5
    Animations,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTINGSETTINGSSTATE = 6
    PilotingSettingsState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIARECORD = 7
    MediaRecord,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIARECORDSTATE = 8
    MediaRecordState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_NETWORKSETTINGS = 9
    NetworkSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_NETWORKSETTINGSSTATE = 10
    NetworkSettingsState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_SPEEDSETTINGS = 11
    SpeedSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_SPEEDSETTINGSSTATE = 12
    SpeedSettingsState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_NETWORK = 13
    Network,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_NETWORKSTATE = 14
    NetworkState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_SETTINGSSTATE = 16
    SettingsState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PICTURESETTINGS = 19
    PictureSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PICTURESETTINGSSTATE = 20
    PictureSettingsState,

    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIASTREAMING = 21
    MediaStreaming(MediaStreaming),
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIASTREAMINGSTATE = 22
    ///
    /// TODO: More info on this command
    /// On how to start the video stream, look at:
    /// arsdk-native/packages/libARController/Sources/ARCONTROLLER_Stream.c:219
    ///
    /// Possible values:
    /// - ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_ENABLED
    ///     Starts stream
    /// - ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_DISABLED
    ///
    /// ```c
    /// /**
    ///  * @brief Current video streaming status.
    ///  */
    /// typedef enum
    /// {
    ///     ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_ENABLED = 0,    ///< Video streaming is enabled.
    ///     ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_DISABLED = 1,    ///< Video streaming is disabled.
    ///     ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_ERROR = 2,    ///< Video streaming failed to start.
    ///     ARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED_MAX
    /// } eARCOMMANDS_ARDRONE3_MEDIASTREAMINGSTATE_VIDEOENABLECHANGED_ENABLED;
    /// ```
    MediaStreamingState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSETTINGS = 23
    GPSSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSETTINGSSTATE = 24
    GPSSettingsState,
    /// Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 69,
    /// feature: Some(Unknown { feature: 1, data: [25, 0, 0, 243, 0] }) }
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_CAMERASTATE = 25
    /// 1. u16:
    /// - ARCOMMANDS_ID_ARDRONE3_CAMERASTATE_CMD_ORIENTATION = 0,
    ///     * _tilt: u8
    ///     * _pan: u8
    /// - ARCOMMANDS_ID_ARDRONE3_CAMERASTATE_CMD_DEFAULTCAMERAORIENTATION = 1,
    /// - ARCOMMANDS_ID_ARDRONE3_CAMERASTATE_CMD_ORIENTATIONV2 = 2,
    ///     * _tilt: float?!
    ///     * _pan: float?!
    ///     See also `ARCOMMANDS_ReadWrite_AddFloatToBuffer`:
    ///         > // Add a float to the buffer
    ///         > // Returns -1 if the buffer is not big enough
    ///         > // Returns the new offset in the buffer on success
    ///         > int32_t ARCOMMANDS_ReadWrite_AddFloatToBuffer (uint8_t *buffer, float newVal, int32_t oldOffset, int32_t buffCap)
    ///         > {
    ///         >     union {
    ///         >            float f;
    ///         >            uint32_t u32;
    ///         >     } val = { .f = newVal };
    ///         >     return ARCOMMANDS_ReadWrite_AddU32ToBuffer (buffer, val.u32, oldOffset, buffCap);
    ///         > }
    /// - ARCOMMANDS_ID_ARDRONE3_CAMERASTATE_CMD_DEFAULTCAMERAORIENTATIONV2 = 3,
    /// - ARCOMMANDS_ID_ARDRONE3_CAMERASTATE_CMD_VELOCITYRANGE = 4,
    ///
    /// 2. _tilt - u8
    /// 3. _pan - u8
    CameraState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ANTIFLICKERING = 29
    AntiFlickering,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ANTIFLICKERINGSTATE = 30
    AntiFlickeringState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSTATE = 31
    /// TODO: use the GPSState struct
    GPSState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PROSTATE = 32
    ProState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ACCESSORYSTATE = 33
    AccessoryState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTINGEVENT = 34
    PilotingEvent,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_SOUND = 35
    Sound,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_SOUNDSTATE = 36
    SoundState,
    Unknown {
        ardrone3: u8,
        data: Vec<u8>,
    },
}

impl Into<u8> for &ArDrone3 {
    fn into(self) -> u8 {
        match self {
            ArDrone3::Piloting(_) => 0,
            ArDrone3::Camera => 1,
            ArDrone3::PilotingSettings => 2,
            ArDrone3::MediaRecordEvent => 3,
            ArDrone3::PilotingState { .. } => 4,
            ArDrone3::Animations => 5,
            ArDrone3::PilotingSettingsState => 6,
            ArDrone3::MediaRecord => 7,
            ArDrone3::MediaRecordState => 8,
            ArDrone3::NetworkSettings => 9,
            ArDrone3::NetworkSettingsState => 10,
            ArDrone3::SpeedSettings => 11,
            ArDrone3::SpeedSettingsState => 12,
            ArDrone3::Network => 13,
            ArDrone3::NetworkState => 14,
            ArDrone3::SettingsState => 16,
            ArDrone3::PictureSettings => 19,
            ArDrone3::PictureSettingsState => 20,
            ArDrone3::MediaStreaming(_) => 21,
            ArDrone3::MediaStreamingState => 22,
            ArDrone3::GPSSettings => 23,
            ArDrone3::GPSSettingsState => 24,
            ArDrone3::CameraState => 28,
            ArDrone3::AntiFlickering => 29,
            ArDrone3::AntiFlickeringState => 30,
            ArDrone3::GPSState => 31,
            ArDrone3::ProState => 32,
            ArDrone3::AccessoryState => 33,
            ArDrone3::PilotingEvent => 34,
            ArDrone3::Sound => 35,
            ArDrone3::SoundState => 36,
            ArDrone3::Unknown { ardrone3, .. } => *ardrone3,
        }
    }
}

/// From pyparrot:
/// For commands reference see (pyparrot/commandsandsensors/ardrone3.xml#L2965-L3008)[https://github.com/amymcgovern/pyparrot/blob/8b7091cdf9a411938566abd7962b05ef7df7adb3/pyparrot/commandsandsensors/ardrone3.xml#L2965-L3008]
///
/// For EnableVideo see (pyparrot/Bebop.py#L448-L461)[https://github.com/amymcgovern/pyparrot/blob/bf4775ec1199b282e4edde1e4a8e018dcc8725e0/pyparrot/Bebop.py#L448-L461]
///
/// ```python
/// command_tuple = self.command_parser.get_command_tuple("ardrone3", "MediaStreaming", "VideoEnable")
/// param_tuple = [1] # Enable
/// param_type_tuple = ['u8']
/// self.drone_connection.send_param_command_packet(command_tuple,param_tuple,param_type_tuple)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaStreaming {
    /// EnableVideo = 0
    /// bool: u8
    EnableVideo(bool),
    // TODO: VideoStreamMode
}

/// u16
/// TODO: Impl (de)serialization
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GPSState {
    /// ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_NUMBEROFSATELLITECHANGED = 0
    ///
    /// > Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 2,
    /// > feature: Some(Unknown { feature: 1, data: [31, 0, 0, 12] }) }
    /// u16 => ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_NUMBEROFSATELLITECHANGED = [0, 0]
    /// u8 => _numberOfSatellite = 12
    NumberOfStatelitesChanged(u8),
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
    /// Last argumet is:
    /// - uint8_t *_available
    ///     > 1 if this type is available, 0 otherwise
    HomeTypeAvailabilityChanged,
    /// ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_HOMETYPECHOSENCHANGED = 2,
    HomeTypeAChosenChanged,
}

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

/// Parrot Piloting Command
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PCMD {
    /// 1 if the roll and pitch values should be taken in consideration. 0 otherwise
    pub flag: bool,
    pub roll: i8,
    pub pitch: i8,
    pub yaw: i8,
    pub gaz: i8,
    pub timestamp: DateTime<Utc>,
    // TODO: How should we handle the `sequence_id` in order not to show it to the user?
    pub sequence_id: u8,
}

pub mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use chrono::TimeZone;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for ArDrone3 {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let ardrone3 = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => {
                    let piloting = src.gread_with::<Piloting>(&mut offset, ctx)?;

                    Self::Piloting(piloting)
                }
                // 1 => Self::Camera,
                // 2 => Self::PilotingSettings,
                // 3 => Self::MediaRecordEvent,
                4 => {
                    let mut piloting_state_data = [0_u8; 256];
                    let actual_written =
                        piloting_state_data.gwrite_with(&src[offset..], &mut 0, ())?;

                    assert_eq!(actual_written, piloting_state_data[..actual_written].len());

                    offset += actual_written;

                    Self::PilotingState {
                        data: piloting_state_data[..actual_written].to_vec(),
                    }
                }
                // 5 => Self::Animations,
                // 6 => Self::PilotingSettingsState,
                // 7 => Self::MediaRecord,
                // 8 => Self::MediaRecordState,
                // 9 => Self::NetworkSettings,
                // 10 => Self::NetworkSettingsState,
                // 11 => Self::SpeedSettings,
                // 12 => Self::SpeedSettingsState,
                // 13 => Self::Network,
                // 14 => Self::NetworkState,
                // 16 => Self::SettingsState,
                // 19 => Self::PictureSettings,
                // 20 => Self::PictureSettingsState,
                // 21 => Self::MediaStreaming,
                // 22 => Self::MediaStreamingState,
                // 23 => Self::GPSSettings,
                // 24 => Self::GPSSettingsState,
                // 25 => Self::CameraState,
                // 29 => Self::AntiFlickering,
                // 30 => Self::AntiFlickeringState,
                // 31 => Self::GPSState,
                // 32 => Self::ProState,
                // 33 => Self::AccessoryState,
                // 34 => Self::PilotingEvent,
                // 35 => Self::Sound,
                // 36 => Self::SoundState,
                // value => {
                //     return Err(MessageError::OutOfBound {
                //         value: value.into(),
                //         param: "ArDrone3".to_string(),
                //     })
                // }
                unknow_ardrone3 => {
                    let mut feature_data = [0_u8; 256];
                    let actual_written = feature_data.gwrite_with(&src[offset..], &mut 0, ())?;

                    assert_eq!(actual_written, feature_data[..actual_written].len());

                    offset += actual_written;

                    Self::Unknown {
                        ardrone3: unknow_ardrone3,
                        data: feature_data[..actual_written].to_vec(),
                    }
                }
            };

            Ok((ardrone3, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for ArDrone3 {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u8>((&self).into(), &mut offset, ctx)?;

            match self {
                Self::Piloting(piloting) => {
                    this.gwrite_with(piloting, &mut offset, ctx)?;
                }
                Self::MediaStreaming(streaming) => match streaming {
                    MediaStreaming::EnableVideo(enabled) => {
                        this.gwrite_with::<u8>(enabled.into(), &mut offset, ctx)?;
                    } // _ => unimplemented!("Not all MediaStreaming options are impled!"),
                },
                Self::PilotingState { data } => {
                    this.gwrite_with::<&[u8]>(data.as_slice(), &mut offset, ())?;
                }
                _ => unimplemented!("Not all ArDrone3 Classes are impled!"),
            }

            Ok(offset)
        }
    }

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

    impl<'a> ctx::TryFromCtx<'a, Endian> for PCMD {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let flag = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => false,
                1 => true,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "flag".to_string(),
                    })
                }
            };

            let roll = src.gread_with(&mut offset, ctx)?;
            let pitch = src.gread_with(&mut offset, ctx)?;
            let yaw = src.gread_with(&mut offset, ctx)?;
            let gaz = src.gread_with(&mut offset, ctx)?;

            let timestamp_and_seq = src.gread_with::<TimestampAndSeq>(&mut offset, ctx)?;

            Ok((
                PCMD {
                    flag,
                    roll,
                    pitch,
                    yaw,
                    gaz,
                    timestamp: timestamp_and_seq.timestamp,
                    sequence_id: timestamp_and_seq.sequence_id,
                },
                offset,
            ))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for PCMD {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u8>(self.flag.into(), &mut offset, ctx)?;
            this.gwrite_with(self.roll, &mut offset, ctx)?;
            this.gwrite_with(self.pitch, &mut offset, ctx)?;
            this.gwrite_with(self.yaw, &mut offset, ctx)?;
            this.gwrite_with(self.gaz, &mut offset, ctx)?;
            let timestamp_and_seq = TimestampAndSeq {
                timestamp: self.timestamp,
                sequence_id: self.sequence_id,
            };

            this.gwrite_with(timestamp_and_seq, &mut offset, ctx)?;

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for TimestampAndSeq {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            // we always use Little-endian
            let timestamp_and_seq = src.gread_with::<u32>(&mut offset, ctx)?.to_le_bytes();
            // 24 bits
            let timestamp_i64 = i64::from_le_bytes([
                timestamp_and_seq[0],
                timestamp_and_seq[1],
                timestamp_and_seq[2],
                0,
                0,
                0,
                0,
                0,
            ]);
            let timestamp = Utc.timestamp_millis(timestamp_i64);
            // 8 bits
            let sequence_id = timestamp_and_seq[3];

            Ok((
                Self {
                    timestamp,
                    sequence_id,
                },
                offset,
            ))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for TimestampAndSeq {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            let milliseconds = self.timestamp.timestamp_millis();
            // from byte 5 to 8 = 3 bytes
            // always use Little-endian!
            let bytes = &milliseconds.to_le_bytes()[5..];

            this.gwrite_with(bytes, &mut offset, ())?;
            this.gwrite_with(self.sequence_id, &mut offset, ctx)?;

            Ok(offset)
        }
    }

    struct TimestampAndSeq {
        timestamp: DateTime<Utc>,
        sequence_id: u8,
    }
}
