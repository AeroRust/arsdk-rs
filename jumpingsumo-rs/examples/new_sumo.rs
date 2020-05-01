use std::error::Error;

use jumpingsumo_rs::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let js = JumpingSumo::connect(PARROT_SPHINX_CONFIG)?;

    js.forward()?;

    Ok(())
}
