use std::fs::File;

use mischief::{IntoMischief, MischiefError, WrapErr};

#[test]
fn report_error() {
    let e: Result<i32, mischief::Report> = Err("first error")
        .map_err(|e| MischiefError::from_display(e))
        .into_mischief()
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error");
    match e {
        Ok(_) => panic!(),
        Err(report) => println!("{:?}", report),
    }
}

#[test]
fn report_ok() -> mischief::Result<()> {
    Ok::<i32, mischief::Result<()>>(2i32)
        .map_err(|e| MischiefError::from_debug(e))
        .into_mischief()
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error")?;
    Ok(())
}
#[test]
fn report_from_error() -> mischief::Result<()> {
    let f = File::open("fake").into_mischief().wrap_err("test wrapper");
    match f {
        Ok(_) => panic!(),
        Err(e) => println!("{e:?}"),
    }
    Ok(())
}
