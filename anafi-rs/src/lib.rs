use arsdk_rs::{
    command::Feature,
    frame::{BufferID, Frame, Type},
};

pub use arsdk_rs::{
    ardrone3::{ArDrone3, MediaStreaming, Piloting, PCMD},
    prelude::*,
};

pub mod prelude {
    pub use crate::Anafi;
    pub use arsdk_rs::{
        ardrone3::{ArDrone3, Piloting, PCMD},
        prelude::*,
    };
}

pub struct Anafi {
    drone: Drone,
}

impl Anafi {
    pub fn connect(config: Config) -> Result<Self, ConnectionError> {
        let drone = Drone::connect(config)?;

        Ok(Self { drone })
    }

    /// - Captain #Ferris 🦀 :Take off... 🛫
    pub fn take_off(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::TakeOff)));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn up(&self, sequence_id: u8) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 0,
            yaw: 0,
            gaz: 100,
            timestamp: Utc::now(),
            sequence_id,
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn down(&self, sequence_id: u8) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 0,
            yaw: 0,
            gaz: -100,
            timestamp: Utc::now(),
            sequence_id,
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn landing(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::Landing)));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }
}
