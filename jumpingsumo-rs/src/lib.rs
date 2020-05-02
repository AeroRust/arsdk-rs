use anyhow::Result as AnyResult;
use arsdk_rs::{
    command::Feature::JumpingSumo as JumpingSumoFeature,
    frame::{BufferID, Frame, Type as FrameType},
    jumping_sumo::PilotState,
    jumping_sumo::{Anim, Class::*, PilotingID::*},
    Config, Drone,
};

pub mod prelude {
    pub use crate::JumpingSumo;
    pub use arsdk_rs::{jumping_sumo::PilotState, prelude::*};
}

pub struct JumpingSumo {
    drone: Drone,
}

const TURN_ANGLE: i8 = 30;
const FORWARD_SPEED: i8 = 100;

impl JumpingSumo {
    pub fn connect(config: Config) -> AnyResult<Self> {
        Ok(Self {
            drone: Drone::connect(config)?,
        })
    }

    pub fn forward(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: FORWARD_SPEED,
            turn: 0,
        })
    }

    pub fn backwards(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: -FORWARD_SPEED,
            turn: 0,
        })
    }

    pub fn turn_left(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: 0,
            turn: -TURN_ANGLE,
        })
    }

    pub fn turn_right(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: 0,
            turn: TURN_ANGLE,
        })
    }

    pub fn stop(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: false,
            speed: 0,
            turn: 0,
        })
    }

    pub fn drive(&self, state: PilotState) -> AnyResult<()> {
        let feature = JumpingSumoFeature(Piloting(Pilot(state)));
        let frame = Frame::for_drone(&self.drone, FrameType::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn jump(&self) -> AnyResult<()> {
        let feature = JumpingSumoFeature(Animations(Anim::Jump));
        let frame = Frame::for_drone(
            &self.drone,
            FrameType::DataWithAck,
            BufferID::CDAck,
            Some(feature),
        );

        self.drone.send_frame(frame)
    }
}
