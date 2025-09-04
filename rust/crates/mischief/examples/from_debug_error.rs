use mischief::WrapErr;

fn foo() -> std::result::Result<i32, &'static str> { Err("fake error") }
fn main() -> mischief::Result<()> {
    foo()
        .map_err(|e| mischief::Report::from_debug(e))
        .wrap_err("error wrapper")?;
    Ok(())
}
