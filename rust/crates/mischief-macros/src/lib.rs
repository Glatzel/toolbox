use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};
use toml_edit::{DocumentMut, Item, TableLike};

mod derive;

/// Internal structure used to parse the arguments of `mischief!`.
///
/// Supports:
/// - Positional format arguments for the error description.
/// - Optional named fields: `source`, `code`, `severity`, `help`, `url`.
struct MischiefErrorInput {
    description_lit: LitStr,
    description_args: Punctuated<Expr, Token![,]>,
    source: Option<Expr>,
    code: Option<Expr>,
    severity: Option<Expr>,
    help: Option<Expr>,
    url: Option<Expr>,
}

impl Parse for MischiefErrorInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // The first argument must always be a string literal for the
        // description.
        let description_lit = input.parse()?;
        let mut description_args = Punctuated::new();

        let mut source = None;
        let mut code = None;
        let mut severity = None;
        let mut help = None;
        let mut url = None;
        let mut seen_named_arg = false;

        while !input.is_empty() {
            input.parse::<Token![,]>()?; // consume comma
            if input.is_empty() {
                break; // allow trailing comma
            }

            // Named fields: key = value
            if input.peek(Ident) && input.peek2(Token![=]) {
                seen_named_arg = true;
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let value: Expr = input.parse()?;

                match key.to_string().as_str() {
                    "source" if source.is_none() => source = Some(value),
                    "code" if code.is_none() => code = Some(value),
                    "severity" if severity.is_none() => severity = Some(value),
                    "help" if help.is_none() => help = Some(value),
                    "url" if url.is_none() => url = Some(value),
                    "source" | "code" | "severity" | "help" | "url" => {
                        return Err(syn::Error::new(
                            key.span(),
                            format!("duplicate field `{key}`"),
                        ));
                    }
                    _ => {
                        return Err(syn::Error::new(
                            key.span(),
                            "unexpected field, expected one of: `source`, `code`, `severity`, `help`, `url`",
                        ));
                    }
                }
            } else {
                // Positional format arguments
                if seen_named_arg {
                    return Err(syn::Error::new(
                        input.span(),
                        "positional arguments cannot follow named arguments",
                    ));
                }
                description_args.push(input.parse()?);
            }
        }

        Ok(Self {
            description_lit,
            description_args,
            source,
            code,
            severity,
            help,
            url,
        })
    }
}

/// Procedural macro to create a `MischiefError` wrapped in a `Report`.
///
/// # Syntax
///
/// ```text
/// mischief!("format string", positional_args..., source = ..., code = ..., severity = ..., help = ..., url = ...)
/// ```
#[proc_macro]
pub fn mischief(input: TokenStream) -> TokenStream { expand_mischief(input.into()).into() }

/// Early-return diagnostic macro.
///
/// Accepts the same diagnostic fields as [`mischief!`] and returns
/// `Err(Report)` from the current function.
#[proc_macro]
pub fn bail(input: TokenStream) -> TokenStream { expand_bail(input.into()).into() }

/// Conditional early-return diagnostic macro.
///
/// The first argument is a boolean condition. If it is false, the remaining
/// arguments are expanded as a diagnostic and returned as `Err(Report)`.
#[proc_macro]
pub fn ensure(input: TokenStream) -> TokenStream { expand_ensure(input.into()).into() }

fn expand_mischief(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<MischiefErrorInput>(input) {
        Ok(input) => {
            let facade = facade_path();
            expand_error(&input, &facade.path)
        }
        Err(error) => error.to_compile_error(),
    }
}

fn expand_bail(input: TokenStream2) -> TokenStream2 {
    let facade = facade_path();
    let parsed_input = match syn::parse2::<MischiefErrorInput>(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    let error = expand_error(&parsed_input, &facade.path);
    quote! {
        return ::core::result::Result::Err(#error)
    }
}

struct EnsureInput {
    condition: Expr,
    error: MischiefErrorInput,
}

impl Parse for EnsureInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let condition = input.parse()?;
        input.parse::<Token![,]>()?;
        let error = input.parse()?;
        Ok(Self { condition, error })
    }
}

fn expand_ensure(input: TokenStream2) -> TokenStream2 {
    let facade = facade_path();
    let parsed_input = match syn::parse2::<EnsureInput>(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    let condition = parsed_input.condition;
    let error = expand_error(&parsed_input.error, &facade.path);
    quote! {
        if !(#condition) {
            return ::core::result::Result::Err(#error);
        }
    }
}

fn expand_error(input: &MischiefErrorInput, facade: &TokenStream2) -> TokenStream2 {
    let description_lit = &input.description_lit;
    let description_args = &input.description_args;

    let code = input
        .code
        .as_ref()
        .map_or_else(|| quote! {}, |value| quote! { .code(#value) });
    let severity = input
        .severity
        .as_ref()
        .map_or_else(|| quote! {}, |value| quote! { .severity(#value) });
    let help = input
        .help
        .as_ref()
        .map_or_else(|| quote! {}, |value| quote! { .help(#value) });
    let url = input
        .url
        .as_ref()
        .map_or_else(|| quote! {}, |value| quote! { .url(#value) });
    let source = input.source.as_ref().map_or_else(
        || quote! {},
        |value| {
            quote! {
                .source({
                    use #facade::IIntoDiagnosis as _;
                    (#value).into_diagnosis()
                })
            }
        },
    );

    quote! {
        {
            extern crate alloc;
            use core::fmt::Write as _;

            let mut description = alloc::string::String::new();
            write!(description, #description_lit, #description_args).unwrap();

            #facade::Report::new(
                #facade::MischiefError::builder(description)
                    #code
                    #severity
                    #help
                    #url
                    #source
                    .build()
            )
        }
    }
}

struct FacadePath {
    path: TokenStream2,
    supports_report: bool,
}

fn facade_path() -> FacadePath {
    // Integration tests can use dev-dependencies, while library builds only
    // have normal dependencies. Prefer the facade in the former case.
    if std::env::var("CARGO_PKG_NAME").as_deref() == Ok("mischief") {
        return match crate_name("mischief") {
            Ok(FoundCrate::Itself) => FacadePath {
                path: quote!(::mischief),
                supports_report: true,
            },
            Ok(FoundCrate::Name(name)) => {
                let name = Ident::new(&name, Span::call_site());
                FacadePath {
                    path: quote!(::#name),
                    supports_report: true,
                }
            }
            Err(_) => FacadePath {
                path: quote!(::mischief),
                supports_report: true,
            },
        };
    }

    let full_facade = match crate_name("mischief") {
        Ok(FoundCrate::Itself) => {
            return FacadePath {
                path: quote!(::mischief),
                supports_report: true,
            };
        }
        Ok(FoundCrate::Name(name)) => Some(Ident::new(&name, Span::call_site())),
        Err(_) => None,
    };

    // `proc-macro-crate` also reports dev-dependencies. A normal facade
    // dependency must win so generated report methods keep working.
    let integration_test = std::env::var_os("CARGO_TARGET_TMPDIR").is_some();
    if integration_test || has_normal_dependency("mischief") {
        if let Some(name) = &full_facade {
            let name = name.clone();
            return FacadePath {
                path: quote!(::#name),
                supports_report: true,
            };
        }
    }

    match crate_name("mischief-api") {
        Ok(FoundCrate::Itself) => FacadePath {
            path: quote!(::mischief_api),
            supports_report: false,
        },
        Ok(FoundCrate::Name(name)) => {
            let name = Ident::new(&name, Span::call_site());
            FacadePath {
                path: quote!(::#name),
                supports_report: false,
            }
        }
        Err(_) => match full_facade {
            Some(name) => FacadePath {
                path: quote!(::#name),
                supports_report: true,
            },
            None => FacadePath {
                path: quote!(::mischief),
                supports_report: true,
            },
        },
    }
}

fn has_normal_dependency(package_name: &str) -> bool {
    let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return false;
    };
    let Ok(manifest) =
        std::fs::read_to_string(std::path::Path::new(&manifest_dir).join("Cargo.toml"))
    else {
        return false;
    };
    let Ok(manifest) = manifest.parse::<DocumentMut>() else {
        return false;
    };

    let direct = manifest
        .get("dependencies")
        .and_then(Item::as_table_like)
        .is_some_and(|dependencies| table_has_package(dependencies, package_name));
    if direct {
        return true;
    }

    manifest
        .get("target")
        .and_then(Item::as_table_like)
        .is_some_and(|targets| {
            targets.iter().any(|(_, target)| {
                target
                    .as_table_like()
                    .and_then(|target| target.get("dependencies"))
                    .and_then(Item::as_table_like)
                    .is_some_and(|dependencies| table_has_package(dependencies, package_name))
            })
        })
}

fn table_has_package(table: &dyn TableLike, package_name: &str) -> bool {
    table.iter().any(|(dependency_name, dependency)| {
        dependency
            .get("package")
            .and_then(Item::as_str)
            .unwrap_or(dependency_name)
            == package_name
    })
}

/// Derives the `IDiagnosis` trait for named-field structs, tuple structs, and
/// enums.
///
/// Structure metadata is configured with `#[diagnosis(...)]`. Mark one field
/// with `#[diagnosis(description)]` to return its `AsRef<str>` value as the
/// description, and mark one field with `#[diagnosis(code)]` to return its
/// string value as the error code. A code field may be a string-like value or
/// an `Option` containing one.
///
/// Mark one field with `#[diagnosis(source)]` to expose it as the diagnosis
/// source. Use `#[diagnosis(source(foreign))]` for a `Send + Sync + 'static`
/// core error that should appear only through `core::error::Error::source()`.
///
/// Enum-level attributes provide defaults for every variant. Variant-level
/// attributes override those defaults. Use `#[diagnosis(forward)]` on a
/// single-field variant to delegate all methods to that field.
///
/// Static descriptions may contain thiserror-style placeholders. Tuple fields
/// use `{0}` and `{1}`; named fields use `{name}`. Format specifications such
/// as `{0:?}`, `{name:>8}`, and `{name:#x}` are supported, and `{{` / `}}`
/// escape literal braces. Only field references are accepted; arbitrary
/// expressions and additional format arguments are not supported.
///
/// `IDiagnosis::description` returns the unformatted template literal for
/// static diagnostics with placeholders, or the marked field's string value for
/// field-backed diagnostics. Renderers and the `DescriptionDisplay` adapter
/// call `IDiagnosis::fmt_description` to produce the formatted text.
/// Enum-level default descriptions cannot contain placeholders.
///
/// Add `display` to generate `core::fmt::Display` from the same description
/// source. Add `error` to generate `core::error::Error`; its `source()`
/// delegates to the field marked `#[diagnosis(source)]`. The type must
/// implement `Debug` and `Display`, either through the `display` flag or
/// manually.
///
/// Mark one field per variant with `#[diagnosis(from)]` to generate
/// `From<FieldType>`. `from` is independent of source tracking. Two `from`
/// fields with the same type in one enum are rejected.
///
/// When the caller depends on the `mischief` facade, every derived type also
/// receives an inherent `into_report(self) -> Report`. Diagnostic sources are
/// copied into the report, while owned foreign sources are retained as
/// downcastable roots. Callers that depend only on `mischief-api` omit this
/// method because the API deliberately has no reporter. The blanket
/// `From<Error> for Report` conversion is unchanged.
///
/// Use `transparent` on a single-field struct or variant to delegate
/// `IDiagnosis`, `Display`, and `Error::source()` to an inner diagnosis and
/// error. Use `transparent(foreign)` for an inner foreign error. Transparent
/// forms require the `display` and `error` flags. For `transparent(foreign)`,
/// `fmt_description` forwards to the inner error's `Display`; `description`
/// returns the enum-level default when present or the variant name otherwise.
/// Transparent wrappers do not add a report node.
///
/// Arbitrary expressions or extra arguments in descriptions,
/// `#[backtrace]`, custom `fmt` expressions, error mapping between external
/// enums, and changes to the blanket `From<Error> for Report` conversion are
/// not supported.
///
/// # Examples
///
/// ```ignore
/// // Compiled from the facade crate, not from the proc-macro crate.
/// use mischief::IDiagnosis;
///
/// #[derive(IDiagnosis)]
/// #[diagnosis(description = "invalid input", code = "E_INPUT")]
/// enum InputError {
///     Missing,
///     Invalid(#[diagnosis(description)] String),
/// }
///
/// assert_eq!(InputError::Missing.code(), Some("E_INPUT"));
/// assert_eq!(InputError::Invalid("name".into()).description(), "name");
/// ```
///
/// Tuple struct fields use positional placeholders:
///
/// ```ignore
/// #[derive(Debug, IDiagnosis)]
/// #[diagnosis(display, error, description = "Filter Error: {0}")]
/// struct FilterError(String);
///
/// assert_eq!(
///     FilterError("invalid UTF-8".into()).to_string(),
///     "Filter Error: invalid UTF-8",
/// );
/// ```
///
/// Descriptions can format fields for both diagnostics and `Display`:
///
/// ```ignore
/// #[derive(Debug, IDiagnosis)]
/// #[diagnosis(display)]
/// enum PitchError {
///     #[diagnosis(description = "Unknown pitch: {0}")]
///     Unknown(char),
/// }
///
/// let error = PitchError::Unknown('H');
/// assert_eq!(error.description(), "Unknown pitch: {0}");
/// assert_eq!(error.to_string(), "Unknown pitch: H");
/// ```
///
/// Transparent foreign errors forward their display and source chain:
///
/// ```ignore
/// #[derive(IDiagnosis)]
/// #[diagnosis(display, error, description = "I/O error")]
/// enum FileError {
///     #[diagnosis(transparent(foreign))]
///     Io(#[diagnosis(from)] std::io::Error),
/// }
/// ```
#[proc_macro_derive(IDiagnosis, attributes(diagnosis))]
#[allow(non_snake_case)]
pub fn IDiagnosis(input: TokenStream) -> TokenStream {
    crate::derive::expand_idiagnosis(input.into()).into()
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use proc_macro2::TokenStream;

    use super::{expand_bail, expand_ensure, expand_mischief};
    use crate::derive::expand_idiagnosis;

    #[test]
    fn mischief_expansion() {
        let input: TokenStream = r#"
            "wrapped {}",
            context,
            code = "E404",
            severity = Severity::Warning,
            help = "Try again.",
            url = "https://example.com/errors/E404",
            source = source_error,
        "#
        .parse()
        .expect("macro input should parse");

        assert_snapshot!(expand_mischief(input).to_string());
    }

    #[test]
    fn bail_expansion() {
        let input: TokenStream = r#""failed {}", context, code = "E500""#
            .parse()
            .expect("macro input should parse");

        assert_snapshot!(expand_bail(input).to_string());
    }

    #[test]
    fn ensure_expansion() {
        let input: TokenStream = r#"ready, "failed {}", context, help = "retry""#
            .parse()
            .expect("ensure input should parse");

        assert_snapshot!(expand_ensure(input).to_string());
    }

    #[test]
    fn idiagnosis_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(
                description = "static description",
                code = "E_DERIVE",
                severity = Severity::Warning,
                help = "try again",
                url = "https://example.com/errors/E_DERIVE",
            )]
            struct StaticDiagnosis {
                #[diagnosis(source)]
                source: Option<MischiefError>,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_tuple_struct_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(error, display, description = "Filter Error: {0}")]
            struct FilterError(String);
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_unit_variant_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(description = "default", code = "E_DEFAULT")]
            enum UnitEnum {
                Unit,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_tuple_variant_expansion() {
        let input: TokenStream = r#"
            enum TupleEnum {
                Tuple(#[diagnosis(description)] String, u32),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_named_variant_expansion() {
        let input: TokenStream = r#"
            enum NamedEnum {
                #[diagnosis(description = "named")]
                Named { value: u32 },
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_default_override_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(
                description = "default",
                code = "E_DEFAULT",
                severity = Severity::Advice,
                help = "default help",
                url = "https://example.com/default",
            )]
            enum OverrideEnum {
                Default,
                #[diagnosis(
                    description = "override",
                    code = "E_OVERRIDE",
                    severity = Severity::Warning,
                    help = "override help",
                    url = "https://example.com/override",
                )]
                Override,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_description_field_expansion() {
        let input: TokenStream = r#"
            enum DescriptionFieldEnum {
                Named {
                    #[diagnosis(description)]
                    message: String,
                },
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_forward_expansion() {
        let input: TokenStream = r#"
            enum ForwardEnum {
                #[diagnosis(forward)]
                Forward(MischiefError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_enum_source_expansion() {
        let input: TokenStream = r#"
            enum SourceEnum {
                #[diagnosis(description = "source")]
                Source {
                    #[diagnosis(source)]
                    source: Option<MischiefError>,
                },
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_display_error_struct_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(
                display,
                error,
                description = "outer",
            )]
            struct Outer {
                #[diagnosis(source)]
                source: Option<Inner>,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_display_error_enum_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(
                display,
                error,
                description = "default",
            )]
            enum Flagged {
                Default,
                #[diagnosis(description = "variant")]
                Variant(#[diagnosis(source)] Inner),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_template_tuple_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display)]
            enum Template {
                #[diagnosis(description = "pair {0}: {1:?}")]
                Pair(&'static str, u8),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_template_named_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display)]
            enum Template {
                #[diagnosis(description = "record {name:>8}: {count:#x}")]
                Record { name: &'static str, count: u32 },
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_template_debug_expansion() {
        let input: TokenStream = r#"
            enum Template {
                #[diagnosis(description = "{value:?}")]
                Debug { value: &'static str },
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_template_escaped_brace_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display)]
            enum Template {
                #[diagnosis(description = "{{literal}} {0}")]
                Literal(&'static str),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_field_expansion() {
        let input: TokenStream = r#"
            enum Template {
                Message(#[diagnosis(description)] String),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_description_template_mixed_transparent_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error)]
            enum Template {
                #[diagnosis(transparent)]
                Inner(Inner),
                #[diagnosis(description = "number {0:?}")]
                Number(u8),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_source_foreign_from_struct_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "foreign source")]
            struct ForeignSource {
                #[diagnosis(source(foreign), from)]
                source: IoError,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_source_foreign_from_enum_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "foreign source")]
            enum ForeignSource {
                #[diagnosis(description = "io")]
                Io(#[diagnosis(source(foreign), from)] IoError),
                #[diagnosis(description = "parse")]
                Parse(#[diagnosis(from)] ParseError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_into_report_struct_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(
                description = "outer",
                code = "E_OUTER",
                severity = Severity::Warning,
                help = "try again",
                url = "https://example.com/E_OUTER",
            )]
            struct Outer {
                #[diagnosis(source)]
                source: Inner,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_into_report_enum_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(description = "default")]
            enum Reportable {
                #[diagnosis(description = "diagnosis")]
                Diagnosis(#[diagnosis(source)] Inner),
                #[diagnosis(description = "foreign")]
                Foreign(#[diagnosis(source(foreign))] IoError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "default")]
            enum Transparent {
                #[diagnosis(transparent)]
                Inner(Inner),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_foreign_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "foreign default", code = "E_FOREIGN")]
            enum Transparent {
                #[diagnosis(transparent(foreign))]
                Io(IoError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_foreign_fallback_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error)]
            enum Transparent {
                #[diagnosis(transparent(foreign))]
                Io(IoError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_named_struct_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, transparent)]
            struct Transparent {
                inner: Inner,
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_from_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "foreign")]
            enum Transparent {
                #[diagnosis(transparent(foreign))]
                Io(#[diagnosis(from)] IoError),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }

    #[test]
    fn idiagnosis_transparent_mixed_enum_expansion() {
        let input: TokenStream = r#"
            #[diagnosis(display, error, description = "default")]
            enum Mixed {
                #[diagnosis(transparent)]
                Inner(Inner),
                #[diagnosis(description = "normal")]
                Normal(#[diagnosis(source)] Source),
            }
        "#
        .parse()
        .expect("derive input should parse");

        assert_snapshot!(expand_idiagnosis(input).to_string());
    }
}
