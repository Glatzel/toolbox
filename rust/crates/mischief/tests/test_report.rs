use mischief::{IntoMischief, Report, WrapErr};

#[test]
fn report_error() {
    let e: core::result::Result<i32, Report> = Err("first error")
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
        .into_mischief()
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error")?;
    Ok(())
}
