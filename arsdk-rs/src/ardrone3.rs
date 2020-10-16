mod gps_state;
mod piloting;
mod piloting_state;

pub use gps_state::GPSState;
pub use piloting::{pcmd::PCMD, Piloting};
pub use piloting_state::PilotingState;

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
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTINGSTATE = 4
    PilotingState(PilotingState),
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

pub mod scroll_impl {
    use super::*;
    use crate::{frame::Error, parse::read_unknown};
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for ArDrone3 {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;
            let ardrone3 = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => Self::Piloting(src.gread_with::<Piloting>(&mut offset, ctx)?),
                // 1 => Self::Camera,
                // 2 => Self::PilotingSettings,
                // 3 => Self::MediaRecordEvent,
                4 => Self::PilotingState(src.gread_with::<PilotingState>(&mut offset, ctx)?),
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
                unknown_ardrone3 => Self::Unknown {
                    ardrone3: unknown_ardrone3,
                    data: read_unknown(src, &mut offset)?,
                },
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
                Self::PilotingState(piloting_state) => {
                    this.gwrite_with(piloting_state, &mut offset, ctx)?;
                }
                _ => unimplemented!("Not all ArDrone3 Classes are impled!"),
            }

            Ok(offset)
        }
    }
}
