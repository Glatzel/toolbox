use std::env;

const TEMPLATE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/shook.toml"));

pub(super) fn execute() -> mischief::Result<()> {
    let dest = env::current_dir()?.join("shook.toml");
    std::fs::write(&dest, TEMPLATE)?;
    Ok(())
}
