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
    // roll: 0,
    // pitch: forward / backward,
    // yaw: strafe left / right,
    // gaz: up / down
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

    pub fn up(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 0,
            yaw: 0,
            gaz: 100,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn down(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 0,
            yaw: 0,
            gaz: -100,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn backward(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: -100,
            yaw: 0,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn forward(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 100,
            yaw: 0,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn strafe_left(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: -100,
            pitch: 0,
            yaw: 0,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn strafe_right(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 100,
            pitch: 0,
            yaw: 0,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn turn_left(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: false,
            roll: 0,
            pitch: 0,
            yaw: -128,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn turn_right(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: false,
            roll: 0,
            pitch: 0,
            yaw: 127,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
        }))));

        let frame = Frame::for_drone(&self.drone, Type::Data, BufferID::CDNonAck, Some(feature));

        self.drone.send_frame(frame)
    }

    pub fn stop(&self) -> Result<(), Error> {
        let feature = Feature::ArDrone3(Some(ArDrone3::Piloting(Piloting::PCMD(PCMD {
            flag: true,
            roll: 0,
            pitch: 0,
            yaw: 0,
            gaz: 0,
            timestamp: Utc::now(),
            sequence_id: self.drone.piloting_id(),
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
