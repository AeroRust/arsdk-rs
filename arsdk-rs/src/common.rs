use chrono::{offset::Utc, DateTime};
use std::ffi::CString;

#[derive(Debug, PartialEq, Eq, Clone)]
/// u8
pub enum Class {
    Network,        // ARCOMMANDS_ID_COMMON_CLASS_NETWORK = 0,
    NetworkEvent,   // ARCOMMANDS_ID_COMMON_CLASS_NETWORKEVENT = 1,
    Settings,       // ARCOMMANDS_ID_COMMON_CLASS_SETTINGS = 2,
    SettingsState,  // ARCOMMANDS_ID_COMMON_CLASS_SETTINGSSTATE = 3,
    Common(Common), // ARCOMMANDS_ID_COMMON_CLASS_COMMON = 4,
    /// ARCOMMANDS_ID_COMMON_CLASS_COMMONSTATE = 5,
    ///
    /// Bytes: 2 127 0 [12 0 0 0] [0] [5] [1 0] 100
    /// Common Common BatterStateChanged 100%
    /// _percent: u8
    ///
    /// eARCOMMANDS_ID_COMMON_COMMONSTATE_CMD: u16
    ////
    /// typedef enum {
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_ALLSTATESCHANGED = 0,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_BATTERYSTATECHANGED = 1,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_MASSSTORAGESTATELISTCHANGED = 2,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_MASSSTORAGEINFOSTATELISTCHANGED = 3,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_CURRENTDATECHANGED = 4,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_CURRENTTIMECHANGED = 5,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_MASSSTORAGEINFOREMAININGLISTCHANGED = 6,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_WIFISIGNALCHANGED = 7,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_SENSORSSTATESLISTCHANGED = 8,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_PRODUCTMODEL = 9,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_COUNTRYLISTKNOWN = 10,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_DEPRECATEDMASSSTORAGECONTENTCHANGED = 11,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_MASSSTORAGECONTENT = 12,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_MASSSTORAGECONTENTFORCURRENTRUN = 13,
    ///     ARCOMMANDS_ID_COMMON_COMMONSTATE_CMD_VIDEORECORDINGTIMESTAMP = 14,
    /// } eARCOMMANDS_ID_COMMON_COMMONSTATE_CMD;
    CommonState,
    Overheat,                // ARCOMMANDS_ID_COMMON_CLASS_OVERHEAT = 6,
    OverheatState,           // ARCOMMANDS_ID_COMMON_CLASS_OVERHEATSTATE = 7,
    Controller,              // ARCOMMANDS_ID_COMMON_CLASS_CONTROLLER = 8,
    WifiSettings,            // ARCOMMANDS_ID_COMMON_CLASS_WIFISETTINGS = 9,
    WifiSettingsState,       // ARCOMMANDS_ID_COMMON_CLASS_WIFISETTINGSSTATE = 10,
    Mavlink,                 // ARCOMMANDS_ID_COMMON_CLASS_MAVLINK = 11,
    MavlinkState,            // ARCOMMANDS_ID_COMMON_CLASS_MAVLINKSTATE = 12,
    FlightPlanSettings,      // ARCOMMANDS_ID_COMMON_CLASS_FLIGHTPLANSETTINGS = 32,
    FlightPlanSettingsState, // ARCOMMANDS_ID_COMMON_CLASS_FLIGHTPLANSETTINGSSTATE = 33,
    Calibration,             // ARCOMMANDS_ID_COMMON_CLASS_CALIBRATION = 13,
    CalibrationState,        // ARCOMMANDS_ID_COMMON_CLASS_CALIBRATIONSTATE = 14,
    CameraSettingsState,     // ARCOMMANDS_ID_COMMON_CLASS_CAMERASETTINGSSTATE = 15,
    Gps,                     // ARCOMMANDS_ID_COMMON_CLASS_GPS = 16,
    FlightPlanState,         // ARCOMMANDS_ID_COMMON_CLASS_FLIGHTPLANSTATE = 17,
    FlightPlanEvent,         // ARCOMMANDS_ID_COMMON_CLASS_FLIGHTPLANEVENT = 19,
    ArLibsVersionsState,     // ARCOMMANDS_ID_COMMON_CLASS_ARLIBSVERSIONSSTATE = 18,
    Audio,                   // ARCOMMANDS_ID_COMMON_CLASS_AUDIO = 20,
    AudioState,              // ARCOMMANDS_ID_COMMON_CLASS_AUDIOSTATE = 21,
    HeadLights,              // ARCOMMANDS_ID_COMMON_CLASS_HEADLIGHTS = 22,
    HeadLightsState,         // ARCOMMANDS_ID_COMMON_CLASS_HEADLIGHTSSTATE = 23,
    Animations,              // ARCOMMANDS_ID_COMMON_CLASS_ANIMATIONS = 24,
    AnimationsState,         // ARCOMMANDS_ID_COMMON_CLASS_ANIMATIONSSTATE = 25,
    Accessory,               // ARCOMMANDS_ID_COMMON_CLASS_ACCESSORY = 26,
    AccessoryState,          // ARCOMMANDS_ID_COMMON_CLASS_ACCESSORYSTATE = 27,
    Charger,                 // ARCOMMANDS_ID_COMMON_CLASS_CHARGER = 28,
    ChargerState,            // ARCOMMANDS_ID_COMMON_CLASS_CHARGERSTATE = 29,
    Runstate,                // ARCOMMANDS_ID_COMMON_CLASS_RUNSTATE = 30,
    Factory,                 // ARCOMMANDS_ID_COMMON_CLASS_FACTORY = 31,
    Unknown {
        class: u8,
        data: Vec<u8>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Common {
    /// ARCOMMANDS_ID_COMMON_COMMON_CMD_ALLSTATES = 0,
    AllStates,
    /// ARCOMMANDS_ID_COMMON_COMMON_CMD_CURRENTDATE = 1,
    CurrentDate(DateTime<Utc>),
    /// ARCOMMANDS_ID_COMMON_COMMON_CMD_CURRENTTIME = 2,
    CurrentTime(DateTime<Utc>),
    /// ARCOMMANDS_ID_COMMON_COMMON_CMD_REBOOT = 3,
    Reboot,
}

// "yyyy-MM-dd"forCommon.Common.CurrentDate.  Ex:2015-08-27
fn format_date(date: &DateTime<Utc>) -> CString {
    let format = date.format("%Y-%m-%d").to_string();

    CString::new(format.as_bytes()).expect("CString::new failed with formatted date")
}
// "’T’HHmmssZZZ"forCommon.Common.CurrentTime. Ex:T101527+0200.
fn format_time(time: &DateTime<Utc>) -> CString {
    let format = time.format("T%H%M%S%z").to_string();

    CString::new(format.as_bytes()).expect("CString::new failed with formatted date")
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for &Class {
    fn into(self) -> u8 {
        use Class::*;
        match self {
            Network => 0,
            NetworkEvent => 1,
            Settings => 2,
            SettingsState => 3,
            Common(_) => 4,
            CommonState => 5,
            Overheat => 6,
            OverheatState => 7,
            Controller => 8,
            WifiSettings => 9,
            WifiSettingsState => 10,
            Mavlink => 11,
            MavlinkState => 12,
            FlightPlanSettings => 32,
            FlightPlanSettingsState => 33,
            Calibration => 13,
            CalibrationState => 14,
            CameraSettingsState => 15,
            Gps => 16,
            FlightPlanState => 17,
            FlightPlanEvent => 19,
            ArLibsVersionsState => 18,
            Audio => 20,
            AudioState => 21,
            HeadLights => 22,
            HeadLightsState => 23,
            Animations => 24,
            AnimationsState => 25,
            Accessory => 26,
            AccessoryState => 27,
            Charger => 28,
            ChargerState => 29,
            Runstate => 30,
            Factory => 31,
            Unknown { class, .. } => *class,
        }
    }
}

impl Into<u8> for Common {
    fn into(self) -> u8 {
        match self {
            Self::AllStates => 0,
            Self::CurrentDate(_) => 1,
            Self::CurrentTime(_) => 2,
            Self::Reboot => 3,
        }
    }
}

pub mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Class {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let class = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => Self::Network,
                1 => Self::NetworkEvent,
                2 => Self::Settings,
                3 => Self::SettingsState,
                4 => {
                    let common = src.gread_with(&mut offset, ctx)?;

                    Self::Common(common)
                }
                // 5 => Self::CommonState,
                // 6 => Self::Overheat,
                // 7 => Self::OverheatState,
                // 8 => Self::Controller,
                // 9 => Self::WifiSettings,
                // 10 => Self::WifiSettingsState,
                // 11 => Self::Mavlink,
                // 12 => Self::MavlinkState,
                // 13 => Self::Calibration,
                // 14 => Self::CalibrationState,
                // 15 => Self::CameraSettingsState,
                // 16 => Self::Gps,
                // 17 => Self::FlightPlanState,
                // 18 => Self::ArLibsVersionsState,
                // 19 => Self::FlightPlanEvent,
                // 20 => Self::Audio,
                // 21 => Self::AudioState,
                // 22 => Self::HeadLights,
                // 23 => Self::HeadLightsState,
                // 24 => Self::Animations,
                // 25 => Self::AnimationsState,
                // 26 => Self::Accessory,
                // 27 => Self::AccessoryState,
                // 28 => Self::Charger,
                // 29 => Self::ChargerState,
                // 30 => Self::Runstate,
                // 31 => Self::Factory,
                // 32 => Self::FlightPlanSettings,
                // 33 => Self::FlightPlanSettingsState,
                unknown_class => {
                    let mut feature_data = [0_u8; 256];
                    let actual_written = feature_data.gwrite_with(&src[offset..], &mut 0, ())?;

                    assert_eq!(actual_written, feature_data[..actual_written].len());

                    offset += actual_written;

                    Self::Unknown {
                        class: unknown_class,
                        data: feature_data[..actual_written].to_vec(),
                    }
                }
            };

            Ok((class, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Class {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u8>((&self).into(), &mut offset, ctx)?;

            match self {
                Self::Common(common) => {
                    this.gwrite_with(common, &mut offset, ctx)?;
                }
                _ => unimplemented!("Not all Class are impled"),
            };

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for Common {
        type Error = Error;

        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            use Common::*;
            let mut offset = 0;

            let common = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => AllStates,
                // @TODO: FIX THIS!
                1 => CurrentDate(Utc::now()),
                // @TODO: FIX THIS!
                2 => CurrentTime(Utc::now()),
                3 => Reboot,
                value => {
                    return Err(Error::OutOfBound {
                        value: value.into(),
                        param: "Common".to_string(),
                    })
                }
            };

            Ok((common, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Common {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u8>(self.into(), &mut offset, ctx)?;

            match self {
                Self::CurrentDate(date) => {
                    let date = format_date(&date);
                    // null terminated C string
                    this.gwrite_with(date.as_bytes_with_nul(), &mut offset, ())?;
                }
                Self::CurrentTime(time) => {
                    // null terminated C string
                    let time = format_time(&time);

                    // null terminated C string
                    this.gwrite_with(time.as_bytes_with_nul(), &mut offset, ())?;
                }
                _ => unimplemented!("Not all Common are impled"),
            }

            Ok(offset)
        }
    }
}
// --------------------- Tests --------------------- //

#[cfg(test)]
mod common_tests {
    use super::*;
    // use chrono::prelude::*;
    // #[test]
    // fn test_format_time() {
    //     // `2014-07-08T09:10:11Z`
    //     let test_time = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11);
    //     assert_eq!(
    //         "2014-07-08".to_string(),
    //         String::from_utf8_lossy(&*format_date(&test_time))
    //     );

    //     assert_eq!(
    //         "T091011+0000".to_string(),
    //         String::from_utf8_lossy(&*format_time(&test_time))
    //     );
    // }

    #[test]
    fn test_class() {
        assert_class(Class::Network, 0);
        assert_class(Class::NetworkEvent, 1);
        assert_class(Class::Settings, 2);
        assert_class(Class::SettingsState, 3);
        assert_class(Class::Common(Common::AllStates), 4);
        assert_class(Class::CommonState, 5);
        assert_class(Class::Overheat, 6);
        assert_class(Class::OverheatState, 7);
        assert_class(Class::Controller, 8);
        assert_class(Class::WifiSettings, 9);
        assert_class(Class::WifiSettingsState, 10);
        assert_class(Class::Mavlink, 11);
        assert_class(Class::MavlinkState, 12);
        assert_class(Class::FlightPlanSettings, 32);
        assert_class(Class::FlightPlanSettingsState, 33);
        assert_class(Class::Calibration, 13);
        assert_class(Class::CalibrationState, 14);
        assert_class(Class::CameraSettingsState, 15);
        assert_class(Class::Gps, 16);
        assert_class(Class::FlightPlanState, 17);
        assert_class(Class::FlightPlanEvent, 19);
        assert_class(Class::ArLibsVersionsState, 18);
        assert_class(Class::Audio, 20);
        assert_class(Class::AudioState, 21);
        assert_class(Class::HeadLights, 22);
        assert_class(Class::HeadLightsState, 23);
        assert_class(Class::Animations, 24);
        assert_class(Class::AnimationsState, 25);
        assert_class(Class::Accessory, 26);
        assert_class(Class::AccessoryState, 27);
        assert_class(Class::Charger, 28);
        assert_class(Class::ChargerState, 29);
        assert_class(Class::Runstate, 30);
        assert_class(Class::Factory, 31);
    }

    #[test]
    fn test_common() {
        assert_common(Common::AllStates, 0);
        assert_common(Common::CurrentDate(chrono::offset::Utc::now()), 1);
        assert_common(Common::CurrentTime(chrono::offset::Utc::now()), 2);
        assert_common(Common::Reboot, 3);
    }

    fn assert_class(dc: Class, v: u8) {
        let as_u8: u8 = (&dc).into();
        assert_eq!(v, as_u8);
    }

    fn assert_common(c: Common, v: u8) {
        let as_u8: u8 = c.into();
        assert_eq!(v, as_u8);
    }
}
