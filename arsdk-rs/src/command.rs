use crate::ardrone3::ArDrone3;
use crate::common;
use crate::jumping_sumo;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Feature {
    Common(common::Class),            // ARCOMMANDS_ID_FEATURE_COMMON = 0,
    ArDrone3(Option<ArDrone3>),       // ARCOMMANDS_ID_FEATURE_ARDRONE3 = 1,
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
    /// UNKOWN 149 from anafi4k
    /// Frame { frame_type: Data, buffer_id: DCNavdata, sequence_id: 14, feature: Some(Unknown { feature: 149, data: [0, 3, 0, 91, 33] }) }

    /// Temporary Enum for storing unknown Features:
    /// TODO: REMOVE!
    Unknown {
        feature: u8,
        data: Vec<u8>,
    },
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for &Feature {
    fn into(self) -> u8 {
        use Feature::*;

        match self {
            Common(_) => 0,
            ArDrone3(_) => 1,
            Minidrone => 2,
            JumpingSumo(_) => 3,
            SkyController => 4,
            PowerUp => 8,
            Generic => 133,
            FollowMe => 134,
            Wifi => 135,
            RC => 136,
            DroneManager => 137,
            Mapper => 138,
            Debug => 139,
            ControllerInfo => 140,
            MapperMini => 141,
            ThermalCam => 142,
            Animation => 144,
            SequoiaCam => 147,
            // Temporary Enum for storing unknown Features:
            // TODO: REMOVE!
            Unknown { feature, .. } => *feature,
        }
    }
}

pub mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Feature {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let feature = match src.gread_with::<u8>(&mut offset, ctx)? {
                // 0 => {
                //     let common = src.gread_with(&mut offset, ctx)?;
                //     Self::Common(common)
                // }
                1 => {
                    let ardrone3 = if !src[offset..].is_empty() {
                        let ardrone3 = src.gread_with::<ArDrone3>(&mut offset, ctx)?;

                        Some(ardrone3)
                    } else {
                        None
                    };

                    Self::ArDrone3(ardrone3)
                }
                // 2 => Self::Minidrone,
                3 => {
                    let js_class = src.gread_with(&mut offset, ctx)?;

                    Self::JumpingSumo(js_class)
                }
                // 4 => Self::SkyController,
                // 8 => Self::PowerUp,
                // 133 => Self::Generic,
                // 134 => Self::FollowMe,
                // 135 => Self::Wifi,
                // 136 => Self::RC,
                // 137 => Self::DroneManager,
                // 138 => Self::Mapper,
                // 139 => Self::Debug,
                // 140 => Self::ControllerInfo,
                // 141 => Self::MapperMini,
                // 142 => Self::ThermalCam,
                // 144 => Self::Animation,
                // 147 => Self::SequoiaCam,
                // value => {
                //     return Err(Self::Error::OutOfBound {
                //         value: value.into(),
                //         param: "Feature".to_string(),
                //     })
                // }
                // Temporary Enum for storing unknown Features:
                // TODO: REMOVE!
                unknown_feature => {
                    let mut feature_data = [0_u8; 256];
                    let actual_written = feature_data.gwrite_with(&src[offset..], &mut 0, ())?;

                    assert_eq!(actual_written, feature_data[..actual_written].len());

                    offset += actual_written;

                    Self::Unknown {
                        feature: unknown_feature,
                        data: feature_data[..actual_written].to_vec(),
                    }
                }
            };

            Ok((feature, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Feature {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u8>((&self).into(), &mut offset, ctx)?;

            match self {
                // Self::Common(common) => {
                //     this.gwrite_with(common, &mut offset, ctx)?;
                // }
                Self::ArDrone3(ardrone3) => {
                    if let Some(ardrone3) = ardrone3 {
                        this.gwrite_with(ardrone3, &mut offset, ctx)?;
                    }
                    // else leave it empty
                }
                Self::JumpingSumo(js) => {
                    this.gwrite_with(js, &mut offset, ctx)?;
                }
                Self::Unknown { data, .. } => {
                    this.gwrite_with(data.as_slice(), &mut offset, ())?;
                }
                _ => unimplemented!("Not all Features are impled"),
            }

            Ok(offset)
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
        assert_feature(
            Feature::ArDrone3(Some(ArDrone3::Piloting(crate::ardrone3::Piloting::TakeOff))),
            1,
        );
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

    fn assert_feature(ref f: Feature, v: u8) {
        let as_u8: u8 = f.into();
        assert_eq!(v, as_u8);
    }
}
