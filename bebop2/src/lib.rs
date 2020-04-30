use anyhow::Result as AnyResult;
use arsdk_rs::{
    command::Feature,
    frame::{BufferID, Frame, Type},
};
use chrono::Utc;
use std::net::IpAddr;

pub use arsdk_rs::{ardrone3::ArDrone3, Drone, PARROT_SPHINX_IP};

pub struct Bebop2 {
    drone: Drone,
}

impl Bebop2 {
    pub fn connect(addr: IpAddr) -> AnyResult<Self> {
        let drone = Drone::new(addr)?;

        drone.send_date_time(Utc::now())?;

        Ok(Self { drone })
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
