use std::error::Error;

use jumpingsumo_rs::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let drone_ip: std::net::IpAddr = "192.168.2.1".parse()?;

    let js = JumpingSumo::connect(drone_ip.into())?;

    js.forward()?;

    Ok(())
}
