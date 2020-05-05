use std::error::Error;

use bebop2::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box <dyn Error>> {
    env_logger::init();

    let drone = Bebop2::connect(PARROT_SPHINX_CONFIG)?;

    tokio::time::delay_for(std::time::Duration::from_secs(1)).await;

    drone.take_off()?;
    // tokio::time::delay_for(std::time::Duration::from_secs(5)).await;
    // drone.landing()?;

    loop {
        // loop endlessly

    }

    Ok(())
}
