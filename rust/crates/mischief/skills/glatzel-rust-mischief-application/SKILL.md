---
name: glatzel-rust-mischief-application
description: Use when adding mischief to a Rust application, CLI, service entry point, or user-facing error boundary, especially when choosing Report, bail, ensure, wrap_err, metadata, downcasting, or verbose terminal rendering.
---

# Mischief for Rust Applications

## Overview

Applications should preserve context while converting technical failures into
actionable user-facing diagnostics. Use `Report` at the application boundary.

## Use the Main Boundary

```rust
use std::path::Path;

use mischief::{IntoMischief, WrapErr, ensure};

fn load_config(path: &Path) -> mischief::Result<String> {
    let text = std::fs::read_to_string(path).into_mischief()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    ensure!(!text.is_empty(), "configuration is empty", help = "provide at least one setting");
    Ok(text)
}

fn main() -> mischief::Result<()> {
    let _config = load_config(Path::new("config.toml"))?;
    Ok(())
}
```

Add context at operation boundaries, not around every call. Use
`wrap_err_with` when constructing the message has a cost.

## Choose the Macro

| Situation | Macro |
| --- | --- |
| Always fail from the current function | `bail!` |
| Fail when an invariant is false | `ensure!` |
| Construct a diagnostic value | `mischief!` |

All accept `source`, `code`, `severity`, `help`, and `url`. `bail!` and
`ensure!` do not require the `macros` feature; `mischief!` does.

## Handle Sources

Wrap foreign errors as they cross each application boundary. Downcast only at
the layer that has a recovery policy:

```rust
if let Some(io_error) = report.downcast_ref::<std::io::Error>() {
    if io_error.kind() == std::io::ErrorKind::NotFound {
        // choose fallback configuration
    }
}
```

Do not downcast merely to stringify an error; `Report` and `IDiagnosis` already
preserve descriptions and metadata.

Runnable example: `cargo run -p mischief --example application_boundary`.

## Render and Log

- `std` selects the verbose tree renderer.
- `fancy` adds color, hyperlinks, Unicode trees, and wrapping.
- Without `std`, `Report` prints descriptions only.
- Print the final report once, normally with `eprintln!("{report}")` or by
  returning `mischief::Result<()>` from `main`.
- Use structured application logging for developer-only details. Do not include
  secrets, credentials, or unstable internal identifiers in diagnostic
  metadata.

## Avoid

- Formatting and printing the same report at multiple layers.
- Using `unwrap` or `expect` for recoverable application failures.
- Returning `Report` as though it implements `Error`; it does not.
- Adding `help` or `url` metadata that is not actionable for users.

## Quick Reference

| Goal | API |
| --- | --- |
| Application result | `mischief::Result<T>` |
| User-facing tree | `eprintln!("{report}")` |
| Add operation context | `wrap_err` / `wrap_err_with` |
| Conditional failure | `ensure!` |
| Unconditional failure | `bail!` |
| Recover a source type | `report.downcast_ref::<T>()` |
