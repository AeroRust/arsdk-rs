use anyhow::Result as AnyResult;
use jumpingsumo_rs::JumpingSumo;
fn main() -> AnyResult<()> {
    // default addr is 192.168.2.1.
    // specify Some("XXX.XXX.XXX.XXX") otherwise
    let js = JumpingSumo::new(None)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    js.jump()?;
    std::thread::sleep(std::time::Duration::from_secs(3));

    Ok(())
}
