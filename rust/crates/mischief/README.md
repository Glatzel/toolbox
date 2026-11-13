# mischief

Structured diagnostics, layered source chains, configurable rendering, and
ergonomic early-error macros.

## Quick start

```rust
use mischief::{IDiagnosis, MischiefError, Report, Result, WrapErr};

let diagnosis = MischiefError::builder("parse failed")
    .code("E_PARSE")
    .help("check the input")
    .build();
let result: Result<()> = Err(Report::new(diagnosis)).wrap_err("load config");
let report = result.expect_err("the result should be an error");

assert_eq!(report.error().description(), "load config");
assert_eq!(
    report.error().source().and_then(|source| source.code()),
    Some("E_PARSE")
);
```

## Macros

```rust
use mischief::{ensure, mischief, IDiagnosis, Result};

let report = mischief!("connect to {}", "db", code = "E_CONNECT", help = "retry");
assert_eq!(report.error().code(), Some("E_CONNECT"));

fn validate(ok: bool) -> Result<()> {
    ensure!(ok, "invalid input", code = "E_INPUT");
    Ok(())
}

assert!(validate(false).is_err());
```

`bail!` returns immediately. `ensure!` checks a condition first. Both accept
`source`, `code`, `severity`, `help`, and `url`, and remain available without
the `macros` feature.

## Derived diagnoses

```rust
use mischief::{IDiagnosis, MischiefError};

#[derive(IDiagnosis)]
#[diagnosis(description = "invalid input", code = "E_INPUT")]
struct InputError {
    #[diagnosis(source)]
    source: Option<MischiefError>,
}

let error = InputError { source: None };
assert_eq!(error.code(), Some("E_INPUT"));
```

Use `#[diagnosis(description = field)]` to delegate the description to an
`AsRef<str>` field. Mark one field with `#[diagnosis(source)]`.

Enums use the same attributes, with enum-level metadata inherited by variants:

```rust
use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "invalid input", code = "E_INPUT")]
enum InputError {
    Missing,
    #[diagnosis(description_from = 0)]
    Invalid(String),
}

assert_eq!(InputError::Invalid("name".into()).description(), "name");
```

More runnable examples are in the crate’s `examples/` directory:

```text
cargo run -p mischief --example enum_derive
cargo run -p mischief --example enum_forward
cargo run -p mischief --example library_boundary
cargo run -p mischief --example application_boundary
```

## Features

| Feature | Effect |
| --- | --- |
| `std` | Enables the verbose tree renderer. |
| `fancy` | Enables color, hyperlinks, Unicode trees, and text wrapping; implies `std`. |
| `macros` | Re-exports `mischief!` and `IDiagnosis`; `bail!` and `ensure!` remain available. |
| `backtrace` | Adds debug-build backtraces; implies `std`. |
| `textwrap-*` | Selects text-wrapping backend behavior. |

Without `std`, `Report` prints descriptions only. The facade requires `alloc`;
the lower-level `mischief-core` crate exposes `alloc` as an optional feature.

The minimum supported Rust version is 1.85.
