use std::error::Error;
use std::net::IpAddr;

use jumpingsumo_rs::JumpingSumo;

fn main() -> Result<(), Box<dyn Error>> {
    let drone_address = "192.168.2.1".parse::<IpAddr>()?;

    let js = JumpingSumo::new(drone_address)?;

    js.forward()?;

    Ok(())
}
