use log::info;
use std::error::Error;

use bebop2::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;
// use smol::Timer;

fn async_setup_logger() {
    let logger = env_logger::Logger::from_default_env();

    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();
    async_setup_logger();

    let drone = Bebop2::connect(PARROT_SPHINX_CONFIG)?;

    info!("Takeoff!");
    drone.take_off()?;

    info!("Wait 5 seconds and fly UP");
    delay_for(Duration::from_secs(5)).await;
    for i in 0..254 {
        drone.up(i)?;
        delay_for(Duration::from_millis(25)).await;
    }

    info!("Wait 5 seconds and fly DOWN");
    delay_for(Duration::from_secs(5)).await;

    for i in 0..220 {
        drone.down(i)?;
        delay_for(Duration::from_millis(25)).await;
    }

    info!("Hover for 4 seconds before landing");
    delay_for(Duration::from_secs(4)).await;

    for _ in 0..50 {
        drone.landing()?;
        delay_for(Duration::from_millis(25)).await;
    }

    loop {}
    // })
}
