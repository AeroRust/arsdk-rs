use std::error::Error;
use std::net::IpAddr;

use bebop2::Bebop2;

fn main() -> Result<(), Box<dyn Error>> {
    // let drone_address = "10.202.0.254".parse::<IpAddr>()?;
    let drone_address = "10.202.0.1".parse::<IpAddr>()?;
    // let drone_address = "192.168.42.1".parse::<IpAddr>()?;

    let drone = Bebop2::new(drone_address)?;

    drone.take_off()?;

    Ok(())
}
