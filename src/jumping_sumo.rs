use crate::frame::Data;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JumpType {
    LONG,    // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    HIGH,    // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    DEFAULT, // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Class {
    Piloting(PilotingID), // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    PilotingState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTINGSTATE = 1,
    Animations(Anim),     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ANIMATIONS = 2,
    AnimationsState,      // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ANIMATIONSSTATE = 3,
    SettingsState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SETTINGSSTATE = 5,
    MediaRecord,          // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORD = 6,
    MediaRecordState,     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORDSTATE = 7,
    NetworkSettings,      // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSETTINGS = 8,
    NetworkSettingsState, // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSETTINGSSTATE = 9,
    Network,              // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORK = 10,
    NetworkState,         // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSTATE = 11,
    AutioSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_AUDIOSETTINGS = 12,
    AudioSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_AUDIOSETTINGSSTATE = 13,
    Roadplan,             // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ROADPLAN = 14,
    RoadplanState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ROADPLANSTATE = 15,
    SpeedSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SPEEDSETTINGS = 16,
    SpeedSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SPEEDSETTINGSSTATE = 17,
    MediaStreaming,       // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIASTREAMING = 18,
    MediaStreamingState,  // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIASTREAMINGSTATE = 19,
    MediaRecordEvent,     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORDEVENT = 20,
    VideoSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_VIDEOSETTINGS = 21,
    VideoSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_VIDEOSETTINGSSTATE = 22,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Anim {
    JumpStop,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPSTOP = 0,
    JumpCancel,      // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPCANCEL = 1,
    JumpLoad,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPLOAD = 2,
    Jump,            // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMP = 3,
    SimpleAnimation, // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_SIMPLEANIMATION = 4,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PilotingID {
    Pilot(PilotState), // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_PCMD = 0,
    Posture,           // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_POSTURE = 1,
    AddCapOffset,      // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_ADDCAPOFFSET = 2,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct PilotState {
    pub flag: u8,
    pub speed: i8,
    pub turn: i8,
}

impl Data for Class {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.clone().into());
        match &self {
            Class::Piloting(piloting_id) => {
                buf.extend(piloting_id.serialize());
            }
            Class::Animations(animation) => {
                buf.push(animation.clone().into());
            }
            _ => {}
        }

        buf
    }
}

impl Data for PilotingID {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2);
        let piloting_u16: u16 = self.clone().into();
        buf.push((piloting_u16 >> 8) as u8);
        buf.push(piloting_u16 as u8);
        match self {
            PilotingID::Pilot(pilot_state) => {
                buf.extend(pilot_state.serialize());
            }
            _ => {}
        }
        buf
    }
}

impl Data for PilotState {
    fn serialize(&self) -> Vec<u8> {
        vec![self.flag as u8, self.speed as u8, self.turn as u8]
    }
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for Class {
    fn into(self) -> u8 {
        match self {
            Self::Piloting(_) => 0,
            Self::PilotingState => 1,
            Self::Animations(_) => 2,
            Self::AnimationsState => 3,
            Self::SettingsState => 5,
            Self::MediaRecord => 6,
            Self::MediaRecordState => 7,
            Self::NetworkSettings => 8,
            Self::NetworkSettingsState => 9,
            Self::Network => 10,
            Self::NetworkState => 11,
            Self::AutioSettings => 12,
            Self::AudioSettingsState => 13,
            Self::Roadplan => 14,
            Self::RoadplanState => 15,
            Self::SpeedSettings => 16,
            Self::SpeedSettingsState => 17,
            Self::MediaStreaming => 18,
            Self::MediaStreamingState => 19,
            Self::MediaRecordEvent => 20,
            Self::VideoSettings => 21,
            Self::VideoSettingsState => 22,
        }
    }
}

impl Into<u8> for Anim {
    fn into(self) -> u8 {
        match self {
            Self::JumpStop => 0,
            Self::JumpCancel => 1,
            Self::JumpLoad => 2,
            Self::Jump => 3,
            Self::SimpleAnimation => 4,
        }
    }
}

impl Into<u16> for PilotingID {
    fn into(self) -> u16 {
        match self {
            Self::Pilot(_) => 0,
            Self::Posture => 1,
            Self::AddCapOffset => 2,
        }
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod jumping_dumo_tests {
    use super::*;

    #[test]
    fn test_piloting_command() {
        assert_piloting(PilotingID::Pilot(PilotState::default()), 0);
        assert_piloting(PilotingID::Posture, 1);
        assert_piloting(PilotingID::AddCapOffset, 2);
    }

    #[test]
    fn test_anim() {
        assert_anim(Anim::JumpStop, 0);
        assert_anim(Anim::JumpCancel, 1);
        assert_anim(Anim::JumpLoad, 2);
        assert_anim(Anim::Jump, 3);
        assert_anim(Anim::SimpleAnimation, 4);
    }

    #[test]
    fn test_class() {
        assert_class(Class::Piloting(PilotingID::Pilot(PilotState::default())), 0);
        assert_class(Class::PilotingState, 1);
        assert_class(Class::Animations(Anim::Jump), 2);
        assert_class(Class::AnimationsState, 3);
        assert_class(Class::SettingsState, 5);
        assert_class(Class::MediaRecord, 6);
        assert_class(Class::MediaRecordState, 7);
        assert_class(Class::NetworkSettings, 8);
        assert_class(Class::NetworkSettingsState, 9);
        assert_class(Class::Network, 10);
        assert_class(Class::NetworkState, 11);
        assert_class(Class::AutioSettings, 12);
        assert_class(Class::AudioSettingsState, 13);
        assert_class(Class::Roadplan, 14);
        assert_class(Class::RoadplanState, 15);
        assert_class(Class::SpeedSettings, 16);
        assert_class(Class::SpeedSettingsState, 17);
        assert_class(Class::MediaStreaming, 18);
        assert_class(Class::MediaStreamingState, 19);
        assert_class(Class::MediaRecordEvent, 20);
        assert_class(Class::VideoSettings, 21);
        assert_class(Class::VideoSettingsState, 22);
    }

    fn assert_class(dc: Class, v: u8) {
        let as_u8: u8 = dc.into();
        assert_eq!(v, as_u8);
    }

    fn assert_anim(a: Anim, v: u8) {
        let as_u8: u8 = a.into();
        assert_eq!(v, as_u8);
    }

    fn assert_piloting(pc: PilotingID, v: u16) {
        let as_u8: u16 = pc.into();
        assert_eq!(v, as_u8);
    }
}
