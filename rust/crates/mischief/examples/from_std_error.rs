use std::fs::File;

use mischief::WrapErr;

fn main() -> mischief::Result<()> {
    let _ = File::open("fake").wrap_err("error wrapper")?;
    Ok(())
}
