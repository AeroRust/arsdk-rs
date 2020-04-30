use anyhow::Result as AnyResult;
use chrono::Utc;
use std::net::IpAddr;
use arsdk_rs::{command::Feature, frame::{Type, Frame, BufferID}};

pub use arsdk_rs::{Drone, ardrone3::ArDrone3};


pub struct Bebop2 {
    drone: Drone,
}

impl Bebop2 {
    pub fn new(addr: IpAddr) -> AnyResult<Self> {
        let drone = Drone::new(addr)?;

        drone.send_date_time(Utc::now())?;

        Ok(Self {
            drone
        })
    }

    pub fn take_off(&self) -> AnyResult<()> {
        // Ardrone3
        // ARCOMMANDS_ID_ARDRONE3_CLASS_PILOTING
        // ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_TAKEOFF
        // strOffset = ARCOMMANDS_ReadWrite_WriteString ("ARDrone3.Piloting.TakeOff:", resString, stringLen, strOffset) ;
        // feature: u8 class: u8 cmd_take_off: u16

        let feature = Feature::ArDrone3(ArDrone3::TakeOff);

        let frame = Frame::for_drone(&self.drone, Type::DataWithAck, BufferID::CDAck, feature);

        self.drone.send_frame(frame)
    }
}
