use mischief::mischief;

fn main() {
    let _ = mischief!(
        "duplicate source",
        source = 1,
        source = 2,
    );
}
