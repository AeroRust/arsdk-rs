use anyhow::Result as AnyResult;
use arsdk_rs::command::Feature::{Common as CommonFeature, JumpingSumo as JumpingSumoFeature};
use arsdk_rs::common;
use arsdk_rs::frame::{BufferID, Frame, Type as FrameType};
use arsdk_rs::jumping_sumo::Anim;
use arsdk_rs::jumping_sumo::Class::*;
use arsdk_rs::jumping_sumo::PilotingID::*;
use arsdk_rs::Drone;
use chrono::{offset::Utc, DateTime};
use std::net::IpAddr;

pub use arsdk_rs::jumping_sumo::PilotState;

pub struct JumpingSumo {
    drone: Drone,
}

const TURN_ANGLE: i8 = 30;
const FORWARD_SPEED: i8 = 100;

impl JumpingSumo {
    pub fn new(addr: IpAddr) -> AnyResult<Self> {
        let js = Self {
            drone: Drone::new(addr)?,
        };

        // Note that you should generate both strings from a single timestamp
        // to avoid any loop error at midnight
        let now = chrono::offset::Utc::now();

        js.send_date(now)?;
        js.send_time(now)?;

        Ok(js)
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
        let frame = Frame::for_drone(&self.drone, FrameType::Data, BufferID::CDNonAck, feature);

        self.drone.send_frame(frame)
    }

    pub fn jump(&self) -> AnyResult<()> {
        let feature = JumpingSumoFeature(Animations(Anim::Jump));
        let frame = Frame::for_drone(
            &self.drone,
            FrameType::DataWithAck,
            BufferID::CDAck,
            feature,
        );

        self.drone.send_frame(frame)
    }

    fn send_date(&self, date: DateTime<Utc>) -> AnyResult<()> {
        let feature = CommonFeature(common::Class::Common(common::Common::CurrentDate(date)));

        let frame = Frame::for_drone(
            &self.drone,
            FrameType::DataWithAck,
            BufferID::CDAck,
            feature,
        );

        self.drone.send_frame(frame)
    }

    fn send_time(&self, date: DateTime<Utc>) -> AnyResult<()> {
        let feature = CommonFeature(common::Class::Common(common::Common::CurrentTime(date)));
        let frame = Frame::for_drone(
            &self.drone,
            FrameType::DataWithAck,
            BufferID::CDAck,
            feature,
        );

        self.drone.send_frame(frame)
    }
}
