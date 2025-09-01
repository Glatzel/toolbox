use mischief::{IntoMischief, Report, WrapErr};

#[test]
fn report_new_and_append_error() {
    let e: core::result::Result<i32, Report> = Err(1).into_mischief().wrap_err("Second error");
    let e = e.wrap_err_with(|| "Third error");
    match e {
        Ok(_) => panic!(),
        Err(report) => println!("{:?}", report),
    }
}
