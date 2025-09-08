use std::fs::File;

use mischief::{IntoMischief, WrapErr};

fn main() -> mischief::Result<()> {
    let _ = File::open("fake")
        .into_mischief()
        .wrap_err(mischief::mischief!("mischief wrapper", help = "some help"))
        .wrap_err("error wrapper")?;
    Ok(())
}
