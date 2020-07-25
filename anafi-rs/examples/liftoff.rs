use log::info;
use std::error::Error;

use anafi_rs::prelude::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let drone_ip: std::net::IpAddr = "192.168.42.1".parse()?;
    let drone = Anafi::connect(drone_ip.into())?;

    info!("Takeoff!");

    for i in 0..50 {
        drone.take_off()?;
    }

    info!("Wait 5 seconds and fly UP");
    std::thread::sleep(Duration::from_secs(5));

    for i in 0..50 {
        drone.landing()?;
    }

    Ok(())
}
