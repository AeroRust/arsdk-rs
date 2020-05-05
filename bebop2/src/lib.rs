use anyhow::Result as AnyResult;
use arsdk_rs::{
    command::Feature,
    frame::{BufferID, Frame, Type},
};

pub use arsdk_rs::{ardrone3::{Piloting, ArDrone3}, prelude::*};

pub mod prelude {
    pub use crate::Bebop2;
    pub use arsdk_rs::{ardrone3::{Piloting, ArDrone3}, prelude::*};
}

pub struct Bebop2 {
    drone: Drone,
}

impl Bebop2 {
    pub fn connect(config: Config) -> AnyResult<Self> {
        let drone = Drone::connect(config)?;

        Ok(Self { drone })
    }

    /// - Captain #Ferris 🦀 :Take off... 🛫
    pub fn take_off(&self) -> AnyResult<()> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::TakeOff)));

        let frame = Frame::for_drone(
            &self.drone,
            Type::DataWithAck,
            BufferID::CDAck,
            Some(feature),
        );

        self.drone.send_frame(frame)
    }

    pub fn landing(&self) -> AnyResult<()> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::Landing)));

        let frame = Frame::for_drone(
            &self.drone,
            Type::DataWithAck,
            BufferID::CDAck,
            Some(feature),
        );

        self.drone.send_frame(frame)
    }
}
