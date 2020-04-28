use crate::frame::Data;
use chrono::{offset::Utc, DateTime};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Class {
    Network,                 // ARCOMMANDS_ID_COMMON_CLASS_NETWORK = 0,
    NetworkEvent,            // ARCOMMANDS_ID_COMMON_CLASS_NETWORKEVENT = 1,
    Settings,                // ARCOMMANDS_ID_COMMON_CLASS_SETTINGS = 2,
    SettingsState,           // ARCOMMANDS_ID_COMMON_CLASS_SETTINGSSTATE = 3,
    Common(Common),          // ARCOMMANDS_ID_COMMON_CLASS_COMMON = 4,
    CommonState,             // ARCOMMANDS_ID_COMMON_CLASS_COMMONSTATE = 5,
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Common {
    AllStates,                  // ARCOMMANDS_ID_COMMON_COMMON_CMD_ALLSTATES = 0,
    CurrentDate(DateTime<Utc>), // ARCOMMANDS_ID_COMMON_COMMON_CMD_CURRENTDATE = 1,
    CurrentTime(DateTime<Utc>), // ARCOMMANDS_ID_COMMON_COMMON_CMD_CURRENTTIME = 2,
    Reboot,                     // ARCOMMANDS_ID_COMMON_COMMON_CMD_REBOOT = 3,
}

impl Data for Class {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.clone().into());
        match self {
            Self::Common(common_command) => {
                buf.extend(common_command.serialize());
            }
            _ => {}
        }
        buf
    }
}

impl Data for Common {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.clone().into());
        match self {
            Self::CurrentDate(date) => {
                buf.extend(format_date(date));
                // null terminated C string
                buf.push(0);
            }
            Self::CurrentTime(time) => {
                buf.extend(format_time(time)); // null terminated C string
                buf.push(0);
            }
            _ => {}
        }
        buf
    }
}

// "yyyy-MM-dd"forCommon.Common.CurrentDate.  Ex:2015-08-27
fn format_date(date: &DateTime<Utc>) -> Vec<u8> {
    date.format("%Y-%m-%d").to_string().as_bytes().to_vec()
}
// "’T’HHmmssZZZ"forCommon.Common.CurrentTime. Ex:T101527+0200.
fn format_time(time: &DateTime<Utc>) -> Vec<u8> {
    time.format("T%H%M%S%z").to_string().as_bytes().to_vec()
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for Class {
    fn into(self) -> u8 {
        match self {
            Self::Network => 0,
            Self::NetworkEvent => 1,
            Self::Settings => 2,
            Self::SettingsState => 3,
            Self::Common(_) => 4,
            Self::CommonState => 5,
            Self::Overheat => 6,
            Self::OverheatState => 7,
            Self::Controller => 8,
            Self::WifiSettings => 9,
            Self::WifiSettingsState => 10,
            Self::Mavlink => 11,
            Self::MavlinkState => 12,
            Self::FlightPlanSettings => 32,
            Self::FlightPlanSettingsState => 33,
            Self::Calibration => 13,
            Self::CalibrationState => 14,
            Self::CameraSettingsState => 15,
            Self::Gps => 16,
            Self::FlightPlanState => 17,
            Self::FlightPlanEvent => 19,
            Self::ArLibsVersionsState => 18,
            Self::Audio => 20,
            Self::AudioState => 21,
            Self::HeadLights => 22,
            Self::HeadLightsState => 23,
            Self::Animations => 24,
            Self::AnimationsState => 25,
            Self::Accessory => 26,
            Self::AccessoryState => 27,
            Self::Charger => 28,
            Self::ChargerState => 29,
            Self::Runstate => 30,
            Self::Factory => 31,
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

// --------------------- Tests --------------------- //

#[cfg(test)]
mod common_tests {
    use super::*;
    use chrono::prelude::*;
    #[test]
    fn test_format_time() {
        // `2014-07-08T09:10:11Z`
        let test_time = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11);
        assert_eq!(
            "2014-07-08".to_string(),
            String::from_utf8_lossy(&*format_date(&test_time))
        );

        assert_eq!(
            "T091011+0000".to_string(),
            String::from_utf8_lossy(&*format_time(&test_time))
        );
    }

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

    fn test_common() {
        assert_common(Common::AllStates, 0);
        assert_common(Common::CurrentDate(chrono::offset::Utc::now()), 1);
        assert_common(Common::CurrentTime(chrono::offset::Utc::now()), 2);
        assert_common(Common::Reboot, 3);
    }

    fn assert_class(dc: Class, v: u8) {
        let as_u8: u8 = dc.into();
        assert_eq!(v, as_u8);
    }

    fn assert_common(c: Common, v: u8) {
        let as_u8: u8 = c.into();
        assert_eq!(v, as_u8);
    }
}
