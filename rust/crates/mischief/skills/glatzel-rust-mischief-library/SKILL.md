---
name: glatzel-rust-mischief-library
description: Use when designing or changing a reusable Rust library API that uses mischief for public diagnostics, source chains, no_std plus alloc support, Cargo features, or library-owned error enums.
---

# Mischief for Rust Libraries

## Overview

Treat diagnostics as part of the library's public contract. Prefer a
library-owned error type over exposing application rendering policy.

## Choose the Public Boundary

- Default to a public enum implementing `IDiagnosis` when callers may match or
  inspect failures.
- Use `MischiefError` when the library needs a dynamic diagnostic payload at an
  internal or application-facing boundary.
- Use `Report` only when an opaque, preformatted error container is intentional.
  `Report` does not implement `Error`.
- The `IDiagnosis` derive generates only `IDiagnosis`. Implement `Error`
  manually if the library promises `core::error::Error`.

Keep terminal rendering, colors, hyperlinks, and application-specific advice in
the application. Stable diagnostic codes, descriptions, and source chains belong
in the library API.

## Select Features

| Need | Configuration |
| --- | --- |
| `no_std` with allocation | `mischief` with `default-features = false` |
| Core types without allocation | Depend on `mischief-core` with `default-features = false` |
| `mischief!` or `IDiagnosis` derive | Enable `macros` |
| Verbose renderer | Enable `std` |
| Color, hyperlinks, Unicode trees, wrapping | Enable `fancy` |

`bail!` and `ensure!` remain available without the `macros` feature. Library
code normally does not need `std`, `backtrace`, `fancy`, or text wrapping.

## Model Failures

Use enum-level metadata for defaults and override it per variant:

```rust
use mischief::{IDiagnosis, MischiefError};

#[derive(IDiagnosis)]
#[diagnosis(description = "configuration failed", code = "E_CONFIG")]
pub enum ConfigError {
    Missing,
    Invalid {
        #[diagnosis(description)]
        path: String,
        #[diagnosis(source)]
        source: MischiefError,
    },
}
```

Use one `#[diagnosis(source)]` field per variant. Store an owned diagnostic or
`MischiefError`; typed foreign-error sources are not supported by the derive.
Convert foreign failures before storing them in the diagnostic field.

Runnable example: `cargo run -p mischief --example library_boundary`.

## Avoid

- Returning `Report` from a library that promises `Error`.
- Re-exporting renderer choices and application help text as stable library API.
- Encoding programs, arbitrary paths, or mutable presentation decisions into
  diagnostic codes.
- Enabling `macros` merely for `bail!` or `ensure!`.

## Quick Reference

| Requirement | Choice |
| --- | --- |
| Caller can match failures | Public diagnostic enum |
| Dynamic internal context | `MischiefError` |
| Opaque error boundary | `Report` |
| Standard error trait | Implement `Error` manually |
| One source per variant | `#[diagnosis(source)]` |
