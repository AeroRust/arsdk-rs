use log::info;
use std::error::Error;

use anafi_rs::prelude::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let drone_ip: std::net::IpAddr = "192.168.42.1".parse()?;
    let drone = Anafi::connect(drone_ip.into())?;

    std::thread::sleep(Duration::from_secs(10));

    log::warn!("Takeoff!");

    for _ in 1..50 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        drone.take_off()?;
    }

    log::warn!("Wait 5 seconds and get down");
    std::thread::sleep(Duration::from_secs(5));

    for _ in 1..50 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        drone.landing()?;
    }

    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}
