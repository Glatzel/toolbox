use std::fs::File;

use mischief::{IntoMischief, WrapErr};

fn main() -> mischief::Result<()> {
    let _ = File::open("fake")
        .into_mischief()
        .wrap_err(mischief::mischief!(
            "mischief wrapper",
            #[cfg(feature = "fancy")]
            help = "some help"
        ))
        .wrap_err("error wrapper")?;
    Ok(())
}
#[test]
fn test() {
    let result = main();
    assert!(result.is_err())
}
