use log::info;
use std::error::Error;

use anafi_rs::prelude::*;
use std::time::Duration;

// https://www.dema.ch/media/catalog/product/pdf/1976008063/pdf_file_3/en_US/white-paper-anafi-usa-v1.5.2_en.pdf
// https://github.com/RIAEvangelist/node-parrot-drone/blob/master/docs/ardrone3.md

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let drone_ip: std::net::IpAddr = "192.168.42.1".parse()?;
    let drone = Anafi::connect(drone_ip.into())?;

    drone.take_off()?;

    std::thread::sleep(Duration::from_secs(2));
    log::warn!("UP!");
    drone.up()?;
    std::thread::sleep(Duration::from_secs(2));

    // log::warn!("forward!");
    // drone.forward()?;
    // std::thread::sleep(Duration::from_secs(1));
    // drone.stop()?;

    // log::warn!("backward!");
    // drone.backward()?;
    // std::thread::sleep(Duration::from_secs(1));
    // drone.stop()?;

    log::warn!("left!");
    drone.strafe_left()?;
    std::thread::sleep(Duration::from_secs(1));

    log::warn!("right!");
    drone.strafe_right()?;
    std::thread::sleep(Duration::from_secs(1));

    // log::warn!("turn left!");
    // for _ in 0..30 {
    //     drone.turn_left()?;
    //     std::thread::sleep(Duration::from_millis(300));
    // }

    // log::warn!("turn right!");
    // for _ in 0..30 {
    //     drone.turn_right()?;
    //     std::thread::sleep(Duration::from_millis(300));
    // }

    log::warn!("DOWN!");
    drone.down()?;
    std::thread::sleep(Duration::from_secs(2));

    std::thread::sleep(Duration::from_secs(2));
    log::warn!("LAND!");
    drone.landing()?;

    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}
