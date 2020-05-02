use std::error::Error;

use bebop2::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let drone = Bebop2::connect(PARROT_SPHINX_CONFIG)?;

    // drone.take_off()?;

    std::thread::sleep(std::time::Duration::from_secs(20));

    Ok(())
}
