use std::error::Error;

use jumpingsumo_rs::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let js = JumpingSumo::connect(PARROT_SPHINX_CONFIG)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    js.jump()?;
    std::thread::sleep(std::time::Duration::from_secs(3));

    Ok(())
}
