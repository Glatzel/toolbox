use mischief::{WrapErr, mischief};

fn foo() -> std::result::Result<i32, &'static str> { Err("fake error") }
fn main() -> mischief::Result<()> {
    foo()
        .map_err(|e| mischief!("{}", e))
        .wrap_err("error wrapper")?;
    Ok(())
}
