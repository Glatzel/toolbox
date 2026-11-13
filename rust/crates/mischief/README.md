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

`#[derive(IDiagnosis)]` implements structured diagnostics for a struct or enum
and adds an inherent `into_report()` method. Add `#[diagnosis(display)]` to
generate `Display`, and `#[diagnosis(error)]` to generate
`core::error::Error`. The `error` flag requires `Debug` and `Display`.

### Complete struct example

The following struct is a diagnostic node. `LoadError` reads its message and
optional code directly from marked fields, while the
`#[diagnosis(source)]` field becomes the next item in the diagnosis chain.

```rust
use mischief::{DescriptionDisplay, IDiagnosis, MischiefError, Severity};

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    display,
    error,
    severity = Severity::Warning,
    help = "check the file and its permissions",
    url = "https://example.com/errors/E_LOAD"
)]
struct LoadError {
    #[diagnosis(description)]
    message: String,
    #[diagnosis(code)]
    code: Option<String>,
    #[diagnosis(source)]
    cause: MischiefError,
}

fn main() {
    let error = LoadError {
        message: "failed to load config.toml: permission denied".into(),
        code: Some("E_LOAD".into()),
        cause: MischiefError::builder("permission denied").build(),
    };

    assert_eq!(
        error.description(),
        "failed to load config.toml: permission denied"
    );
    assert_eq!(
        error.to_string(),
        "failed to load config.toml: permission denied"
    );
    assert_eq!(
        format!("{}", DescriptionDisplay(&error)),
        "failed to load config.toml: permission denied"
    );

    assert_eq!(error.code(), Some("E_LOAD"));
    assert_eq!(error.severity(), Some(Severity::Warning));
    assert_eq!(error.help(), Some("check the file and its permissions"));
    assert_eq!(
        error.source().map(IDiagnosis::description),
        Some("permission denied")
    );

    let report = error.into_report();
    assert_eq!(report.error().code(), Some("E_LOAD"));
    assert_eq!(
        report.error().source().and_then(IDiagnosis::description),
        Some("permission denied")
    );
}
```

The important pieces are:

- `#[diagnosis(description)]` reads the message from a string-like field.
- `#[diagnosis(code)]` reads an optional code from the marked field.
- `display` makes `to_string()` use the field-backed description.
- `source` exposes one field through `IDiagnosis::source()` and `Error::source()`.
- `into_report()` copies the structured diagnostic chain into a `Report`.

### Complete enum example

Enums use the same attributes. Enum-level attributes are defaults inherited by
variants, while variant-level attributes override those defaults. The example
also distinguishes a normal wrapper, `transparent`, and
`transparent(foreign)`:

```rust
use std::io;

use mischief::{IDiagnosis, Severity};

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    display,
    error,
    description = "core failure",
    severity = Severity::Warning
)]
enum CoreFailure {
    InvalidState,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    display,
    error,
    description = "application failure",
    code = "E_APPLICATION"
)]
enum ApplicationError {
    // Remove this wrapper from reports and expose CoreFailure directly.
    #[diagnosis(transparent)]
    Core(#[diagnosis(from)] CoreFailure),

    // std::io::Error is not an IDiagnosis, so mark it as foreign.
    // This also removes the wrapper and generates From<std::io::Error>.
    #[diagnosis(transparent(foreign))]
    Io(#[diagnosis(from)] io::Error),

    // This remains an ordinary diagnostic node and reads metadata from fields.
    Invalid {
        #[diagnosis(description)]
        message: String,
        #[diagnosis(code)]
        code: Option<String>,
    },
}

fn main() {
    let core = ApplicationError::from(CoreFailure::InvalidState);
    assert_eq!(core.description(), "core failure");
    assert_eq!(core.severity(), Some(Severity::Warning));
    assert_eq!(core.to_string(), "core failure");

    let core_report = core.into_report();
    assert_eq!(core_report.error().description(), "core failure");
    assert_eq!(core_report.error().severity(), Some(Severity::Warning));

    let io = ApplicationError::from(io::Error::other("read failed"));
    assert_eq!(io.description(), "application failure");
    assert_eq!(io.code(), Some("E_APPLICATION"));
    assert_eq!(io.to_string(), "read failed");
    assert!(core::error::Error::source(&io).is_none());

    let io_report = io.into_report();
    assert_eq!(io_report.error().description(), "read failed");
    assert!(io_report.downcast_ref::<io::Error>().is_some());

    let invalid = ApplicationError::Invalid {
        message: "invalid input: unexpected token".into(),
        code: Some("E_INPUT".into()),
    };
    assert_eq!(invalid.description(), "invalid input: unexpected token");
    assert_eq!(invalid.code(), Some("E_INPUT"));
    assert_eq!(invalid.to_string(), "invalid input: unexpected token");
}
```

### Choosing how to nest errors

| Form | Inner type requirement | Effect |
| --- | --- | --- |
| `#[diagnosis(source)]` | `IDiagnosis` | Keeps the outer error as a report node and appends the inner diagnosis as its source. |
| `#[diagnosis(source(foreign))]` | `Error + Send + Sync + 'static` | Keeps the outer node while retaining the concrete foreign error as a report source. |
| `#[diagnosis(transparent)]` | `IDiagnosis + Display + Error` | Removes the wrapper and forwards all diagnostic metadata, `Display`, and `Error::source()` to the inner value. |
| `#[diagnosis(transparent(foreign))]` | `Display + Error + Send + Sync + 'static` | Removes the wrapper, forwards `Display`, and retains the foreign error as the report root for downcasting. |

Both transparent forms require exactly one field and the type-level
`display` and `error` flags. `transparent` cannot add metadata of its own.
`transparent(foreign)` uses the enum-level description, or the variant name
when no default exists, for `description()`; formatted descriptions and
`Display` use the foreign error's text.

Additional metadata rules:

- `description = "..."` defines a static description template.
- `#[diagnosis(description)]` marks a field implementing `AsRef<str>` as the
  description.
- `code = "..."` defines a static diagnostic code.
- `#[diagnosis(code)]` marks a field as the code; the field may be string-like
  or an `Option` containing a string-like value.
- Tuple fields use `{0}`, `{1}`, and named fields use `{name}`.
- `{0:?}` and `{name:#x}` format specifications are supported.
- `{{` and `}}` escape literal braces.
- Enum-level default descriptions cannot contain placeholders.
- `description()` returns the template unchanged; renderers and
  `DescriptionDisplay` call `fmt_description()` for the formatted text.

More runnable examples are in the crate’s `examples/` directory:

```text
cargo run -p mischief --example enum_derive
cargo run -p mischief --example enum_forward
cargo run -p mischief --example library_boundary
cargo run -p mischief --example application_boundary
cargo run -p mischief --example thiserror_upstream
cargo run -p mischief --example enum_transparent
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
