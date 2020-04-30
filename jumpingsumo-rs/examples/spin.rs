use std::error::Error;

use jumpingsumo_rs::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let js = JumpingSumo::connect(PARROT_SPHINX_CONFIG)?;

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
