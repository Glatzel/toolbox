use std::fs::File;

use mischief::{Severity, WrapErr, mischief};

#[test]
fn report_error() {
    let e: Result<i32, mischief::Report> = Err("first error")
        .map_err(|e| mischief!("{}", e))
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
        .map_err(|e| mischief!("{:?}", e))
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error")?;
    Ok(())
}
#[test]
fn report_from_error() -> mischief::Result<()> {
    let f = File::open("fake").wrap_err("error wrapper");
    match f {
        Ok(_) => panic!(),
        Err(e) => println!("{e}"),
    }
    Ok(())
}
