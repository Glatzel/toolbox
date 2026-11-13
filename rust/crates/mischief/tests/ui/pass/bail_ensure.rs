use mischief::{bail, ensure};

fn bail_case(value: bool) -> mischief::Result<()> {
    if value {
        bail!("bail {}", value, code = "E_BAIL");
    }
    Ok(())
}

fn ensure_case(value: bool) -> mischief::Result<()> {
    ensure!(value, "ensure {}", value, help = "retry");
    Ok(())
}

fn main() {
    let _ = bail_case(false);
    let _ = ensure_case(true);
}
