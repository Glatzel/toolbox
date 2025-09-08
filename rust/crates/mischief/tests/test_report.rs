use std::fs::File;

use mischief::{IntoMischief, WrapErr, mischief};

#[test]
fn report_error() {
    let e: Result<i32, mischief::Report> = Err("first error")
        .map_err(|e| {
            mischief!(
                "{}",
                e,
                severity = mischief::Severity::Warning,
                code = "E404",
                url = "https://github.com/Glatzel/toolbox",
                help = "Try again."
            )
        })
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error");
    match e {
        Ok(_) => panic!(),
        Err(report) => println!("{:?}", report),
    }
}
#[test]
fn report_error_long() {
    let e: Result<i32, mischief::Report> = Err("Failed to parse configuration file due to invalid syntax near line 42; unexpected token found that prevents correct interpretation of the settings provided.")
        .map_err(|e| {
            mischief!(
                "{}",
                e,
                severity = mischief::Severity::Warning,
                code = "E404",
                url = "https://github.com/Glatzel/toolbox",
                help = "To resolve this issue, please ensure all required dependencies are installed, network connectivity is stable, and configuration files are correctly formatted; you can enable verbose logging for more detailed diagnostic information."
            )
        })
        .wrap_err("Error: Connection to the database server timed out after multiple retries, possibly caused by network instability, firewall restrictions, or an unreachable endpoint.")
        .wrap_err_with(|| "Error: Attempted to access a resource that is not available in the current execution environment, which may indicate missing dependencies, restricted permissions, or an incorrect build target.");
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
    let f = File::open("fake").into_mischief().wrap_err("error wrapper");
    match f {
        Ok(_) => panic!(),
        Err(e) => println!("{e:?}"),
    }
    Ok(())
}
