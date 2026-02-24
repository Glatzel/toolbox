fn main() -> mischief::Result<()> {
    mischief::bail!("bail");
}
#[test]
fn test() {
    let result = main();
    assert!(result.is_err())
}
