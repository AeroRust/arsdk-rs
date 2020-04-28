use std::error::Error;
use std::net::IpAddr;
use jumpingsumo_rs::{JumpingSumo, PilotState};
fn main() -> Result<(), Box<dyn Error>> {
    let drone_address = "192.168.2.1".parse::<IpAddr>()?;

    let js = JumpingSumo::new(drone_address)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let turn_right = PilotState {
            flag: true,
            speed: 0,
            turn: i8::MAX,
        };
        js.drive(turn_right)?;
    }
}
