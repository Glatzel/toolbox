fn parse(value: &str) -> mischief::Result<&str> {
    match value {
        "ok" => Ok(value),
        _ => mischief::bail!("invalid value"),
    }
}

fn main() {
    assert!(parse("ok").is_ok());
    assert!(parse("bad").is_err());
}
