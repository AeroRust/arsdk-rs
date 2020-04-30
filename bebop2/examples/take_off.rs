use std::error::Error;

use bebop2::{Bebop2, PARROT_SPHINX_IP};

fn main() -> Result<(), Box<dyn Error>> {
    let drone_address = PARROT_SPHINX_IP;

    let drone = Bebop2::connect(drone_address)?;

    drone.take_off()?;

    Ok(())
}
