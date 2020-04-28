use anyhow::Result as AnyResult;
use jumpingsumo_rs::{JumpingSumo, PilotState};
fn main() -> AnyResult<()> {
    // default addr is 192.168.2.1.
    // specify Some("XXX.XXX.XXX.XXX") otherwise
    let js = JumpingSumo::new(None)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let turn_right = PilotState {
            flag: 1,
            speed: 0,
            turn: i8::MAX,
        };
        js.drive(turn_right)?;
    }
}
