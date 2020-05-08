use std::error::Error;

use bebop2::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let drone = Bebop2::connect(PARROT_SPHINX_CONFIG)?;

    // std::thread::sleep(std::time::Duration::from_ses(1));
    tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    drone.take_off()?;

    tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
    for i in 0..20 {
        drone.up(i)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    drone.landing()?;

    loop {
        // loop endlessly
    }

    Ok(())
}
