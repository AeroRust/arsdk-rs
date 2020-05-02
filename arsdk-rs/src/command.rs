use crate::ardrone3::ArDrone3;
use crate::common;
use crate::frame::Data;
use crate::jumping_sumo;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum Feature {
    Common(common::Class),            // ARCOMMANDS_ID_FEATURE_COMMON = 0,
    ArDrone3(ArDrone3),               // ARCOMMANDS_ID_FEATURE_ARDRONE3 = 1,
    Minidrone,                        // ARCOMMANDS_ID_FEATURE_MINIDRONE = 2,
    JumpingSumo(jumping_sumo::Class), // ARCOMMANDS_ID_FEATURE_JUMPINGSUMO = 3,
    SkyController,                    // ARCOMMANDS_ID_FEATURE_SKYCONTROLLER = 4,
    PowerUp,                          // ARCOMMANDS_ID_FEATURE_POWERUP = 8,
    Generic,                          // ARCOMMANDS_ID_FEATURE_GENERIC = 133,
    FollowMe,                         // ARCOMMANDS_ID_FEATURE_FOLLOW_ME = 134,
    Wifi,                             // ARCOMMANDS_ID_FEATURE_WIFI = 135,
    RC,                               // ARCOMMANDS_ID_FEATURE_RC = 136,
    DroneManager,                     // ARCOMMANDS_ID_FEATURE_DRONE_MANAGER = 137,
    Mapper,                           // ARCOMMANDS_ID_FEATURE_MAPPER = 138,
    Debug,                            // ARCOMMANDS_ID_FEATURE_DEBUG = 139,
    ControllerInfo,                   // ARCOMMANDS_ID_FEATURE_CONTROLLER_INFO = 140,
    MapperMini,                       // ARCOMMANDS_ID_FEATURE_MAPPER_MINI = 141,
    ThermalCam,                       // ARCOMMANDS_ID_FEATURE_THERMAL_CAM = 142,
    Animation,                        // ARCOMMANDS_ID_FEATURE_ANIMATION = 144,
    SequoiaCam,                       // ARCOMMANDS_ID_FEATURE_SEQUOIA_CAM = 147,
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for Feature {
    fn into(self) -> u8 {
        match self {
            Self::Common(_) => 0,
            Self::ArDrone3(_) => 1,
            Self::Minidrone => 2,
            Self::JumpingSumo(_) => 3,
            Self::SkyController => 4,
            Self::PowerUp => 8,
            Self::Generic => 133,
            Self::FollowMe => 134,
            Self::Wifi => 135,
            Self::RC => 136,
            Self::DroneManager => 137,
            Self::Mapper => 138,
            Self::Debug => 139,
            Self::ControllerInfo => 140,
            Self::MapperMini => 141,
            Self::ThermalCam => 142,
            Self::Animation => 144,
            Self::SequoiaCam => 147,
        }
    }
}

impl Data for Feature {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.clone().into());
        match &self {
            Feature::JumpingSumo(js) => {
                buf.extend(js.serialize());
            }
            Feature::Common(common) => {
                buf.extend(common.serialize());
            }
            Feature::ArDrone3(drone) => {
                buf.extend(drone.serialize());
            }
            _ => {}
        }
        buf
    }
}

pub mod scroll_impl {
    use super::*;
    use crate::MessageError;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Feature {
        type Error = MessageError;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], endian: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let feature = match src.gread_with::<u8>(&mut offset, endian)? {
                0 => {
                    let common = src.gread_with(&mut offset, endian)?;

                    Self::Common(common)
                }
                1 => {
                    let ardrone3 = src.gread_with(&mut offset, endian)?;

                    Self::ArDrone3(ardrone3)
                }
                2 => Self::Minidrone,
                3 => {
                    let js_class = src.gread_with(&mut offset, endian)?;

                    Self::JumpingSumo(js_class)
                }
                4 => Self::SkyController,
                8 => Self::PowerUp,
                133 => Self::Generic,
                134 => Self::FollowMe,
                135 => Self::Wifi,
                136 => Self::RC,
                137 => Self::DroneManager,
                138 => Self::Mapper,
                139 => Self::Debug,
                140 => Self::ControllerInfo,
                141 => Self::MapperMini,
                142 => Self::ThermalCam,
                144 => Self::Animation,
                147 => Self::SequoiaCam,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "Feature".to_string(),
                    })
                }
            };

            Ok((feature, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Feature {
        type Error = scroll::Error;

        fn try_into_ctx(self, this: &mut [u8], _ctx: Endian) -> Result<usize, Self::Error> {
            let ser_feature = self.serialize();
            let written = this.pwrite_with(ser_feature.as_slice(), 0, ())?;

            Ok(written)
        }
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod command_tests {
    use super::*;
    #[test]
    fn test_feature() {
        assert_feature(
            Feature::Common(common::Class::Common(common::Common::AllStates)),
            0,
        );
        assert_feature(Feature::ArDrone3(ArDrone3::TakeOff), 1);
        assert_feature(Feature::Minidrone, 2);
        assert_feature(
            Feature::JumpingSumo(jumping_sumo::Class::Piloting(
                jumping_sumo::PilotingID::Pilot(jumping_sumo::PilotState::default()),
            )),
            3,
        );
        assert_feature(Feature::SkyController, 4);
        assert_feature(Feature::PowerUp, 8);
        assert_feature(Feature::Generic, 133);
        assert_feature(Feature::FollowMe, 134);
        assert_feature(Feature::Wifi, 135);
        assert_feature(Feature::RC, 136);
        assert_feature(Feature::DroneManager, 137);
        assert_feature(Feature::Mapper, 138);
        assert_feature(Feature::Debug, 139);
        assert_feature(Feature::ControllerInfo, 140);
        assert_feature(Feature::MapperMini, 141);
        assert_feature(Feature::ThermalCam, 142);
        assert_feature(Feature::Animation, 144);
        assert_feature(Feature::SequoiaCam, 147);
    }

    fn assert_feature(f: Feature, v: u8) {
        let as_u8: u8 = f.into();
        assert_eq!(v, as_u8);
    }
}
