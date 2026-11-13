use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};

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
        Ok(input) => expand_error(&input, &facade_path()),
        Err(error) => error.to_compile_error(),
    }
}

fn expand_bail(input: TokenStream2) -> TokenStream2 {
    let parsed_input = match syn::parse2::<MischiefErrorInput>(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    let error = expand_error(&parsed_input, &facade_path());
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
    let parsed_input = match syn::parse2::<EnsureInput>(input) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error(),
    };
    let condition = parsed_input.condition;
    let error = expand_error(&parsed_input.error, &facade_path());
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

fn facade_path() -> TokenStream2 {
    match crate_name("mischief") {
        Ok(FoundCrate::Itself) => quote!(::mischief),
        Ok(FoundCrate::Name(name)) => {
            let name = Ident::new(&name, Span::call_site());
            quote!(::#name)
        }
        Err(_) => quote!(::mischief),
    }
}

/// Derives the `IDiagnosis` trait for named-field structs and enums.
///
/// Structure metadata is configured with `#[diagnosis(...)]`. Descriptions may
/// be static literals or delegated to a field implementing `AsRef<str>`. Mark
/// one field with `#[diagnosis(source)]` to expose it as the diagnosis source.
///
/// Enum-level attributes provide defaults for every variant. Variant-level
/// attributes override those defaults. Use `#[diagnosis(forward)]` on a
/// single-field variant to delegate all methods to that field.
///
/// Add `display` to generate `core::fmt::Display` from static descriptions. Add
/// `error` to generate `core::error::Error`; its `source()` delegates to the
/// field marked `#[diagnosis(source)]`. The type must implement `Debug` and
/// `Display`, either through the `display` flag or manually.
///
/// Formatted descriptions, typed foreign-error sources, transparent
/// derivation, `into_report()` generation, and error mapping between enums are
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
///     #[diagnosis(description_from = 0)]
///     Invalid(String),
/// }
///
/// assert_eq!(InputError::Missing.code(), Some("E_INPUT"));
/// assert_eq!(InputError::Invalid("name".into()).description(), "name");
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
                #[diagnosis(description_from = 0)]
                Tuple(String, u32),
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
    fn idiagnosis_enum_description_from_expansion() {
        let input: TokenStream = r#"
            enum DescriptionFromEnum {
                #[diagnosis(description_from = message)]
                Named { message: String },
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
}
