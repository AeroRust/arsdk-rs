use anyhow::Result as AnyResult;
use arsdk_rs::{
    command::Feature,
    frame::{BufferID, Frame, Type},
};

pub use arsdk_rs::{ardrone3::ArDrone3, prelude::*};

pub mod prelude {
    pub use crate::Bebop2;
    pub use arsdk_rs::{ardrone3::ArDrone3, prelude::*};
}

pub struct Bebop2 {
    drone: Drone,
}

impl Bebop2 {
    pub fn connect(config: Config) -> AnyResult<Self> {
        let drone = Drone::connect(config)?;

        Ok(Self { drone })
    }

    // ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_NAVIGATEHOME
    // ARCOMMANDS_ID_ARDRONE3_PILOTING_CMD_AUTOTAKEOFFMODE

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
