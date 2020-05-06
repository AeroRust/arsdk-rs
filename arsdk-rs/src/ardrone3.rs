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
    PilotingState,
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
    MediaStreaming,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_MEDIASTREAMINGSTATE = 22
    MediaStreamingState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSETTINGS = 23
    GPSSettings,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSETTINGSSTATE = 24
    GPSSettingsState,
    /// Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 69,
    /// feature: Some(Unknown { feature: 1, data: [25, 0, 0, 243, 0] }) }
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_CAMERASTATE = 25
    CameraState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ANTIFLICKERING = 29
    AntiFlickering,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_ANTIFLICKERINGSTATE = 30
    AntiFlickeringState,
    /// ARCOMMANDS_ID_ARDRONE3_CLASS_GPSSTATE = 31
    ///
    /// Frame { frame_type: DataWithAck, buffer_id: DCEvent, sequence_id: 2,
    /// feature: Some(Unknown { feature: 1, data: [31, 0, 0, 12] }) }
    /// u16 => ARCOMMANDS_ID_ARDRONE3_GPSSTATE_CMD_NUMBEROFSATELLITECHANGED = 0
    /// u8 => _numberOfSatellite
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// eARCOMMANDS_ID_ARDRONE3_PILOTING_CMD
/// u16
pub enum Piloting {
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_FLATTRIM = 0
    FlatTrim = 0,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_TAKEOFF = 1
    TakeOff = 1,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_PCMD = 2
    /// ARCOMMANDS_Decoder_ARDrone3PilotingPCMDCb (_flag, _roll, _pitch, _yaw, _gaz, _timestampAndSeqNum, ARCOMMANDS_Decoder_ARDrone3PilotingPCMDCustom);
    /// ARCOMMANDS_Decoder_ARDrone3PilotingPCMDDecodeArgs (uint8_t *_flag, int8_t *_roll, int8_t *_pitch, int8_t *_yaw, int8_t *_gaz, uint32_t *_timestampAndSeqNum)
    /// @see https://developer.parrot.com/docs/reference/bebop_2/index.html#move-the-drone
    PCMD = 2,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_LANDING = 3
    Landing = 3,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_EMERGENCY = 4
    Emergency = 4,
    /// ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_NAVIGATEHOME = 5
    /// requires: uint8_t _start
    /// as u8
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
    use crate::frame::Error;
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
                // 4 => Self::PilotingState,
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
            match self {
                Self::Piloting(piloting) => {
                    // TODO: Impl a `Into<u8>` maybe?
                    this.gwrite_with::<u8>(0, &mut offset, ctx)?;

                    this.gwrite_with(piloting, &mut offset, ctx)?;
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
            use Piloting::*;

            let mut offset = 0;

            let piloting = match src.gread_with::<u16>(&mut offset, ctx)? {
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
            Ok(this.pwrite_with::<u16>(self as u16, 0, ctx)?)
        }
    }
}
