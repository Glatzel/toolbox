use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Expr, Fields, Ident, LitStr, Token};

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
        return ::core::result::Result::Err(#error);
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

/// Derives the `IDiagnosis` trait for a named-field struct.
///
/// Container metadata is configured with `#[diagnosis(...)]`. Descriptions
/// may be static literals or delegated to a field implementing `AsRef<str>`.
/// Mark one field with `#[diagnosis(source)]` to expose it as the diagnosis
/// source.
#[proc_macro_derive(IDiagnosis, attributes(diagnosis))]
#[allow(non_snake_case)]
pub fn IDiagnosis(input: TokenStream) -> TokenStream { expand_idiagnosis(input.into()).into() }

#[derive(Default)]
struct DiagnosisAttributes {
    description: Option<Description>,
    code: Option<LitStr>,
    severity: Option<Expr>,
    help: Option<LitStr>,
    url: Option<LitStr>,
}

enum Description {
    Static(LitStr),
    Field(Ident),
}

fn expand_idiagnosis(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<DeriveInput>(input).and_then(|input| derive_idiagnosis(&input)) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error(),
    }
}

fn derive_idiagnosis(input: &DeriveInput) -> Result<TokenStream2> {
    let config = parse_diagnosis_attributes(&input.attrs)?;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "IDiagnosis can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "IDiagnosis can only be derived for structs with named fields",
            ));
        }
    };

    let mut source = None;
    for field in fields {
        let field_name = field
            .ident
            .clone()
            .expect("named fields always have identifiers");
        for attr in &field.attrs {
            if !attr.path().is_ident("diagnosis") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("source") {
                    if source.is_some() {
                        return Err(meta.error("duplicate `source` field marker"));
                    }
                    source = Some(field_name.clone());
                    Ok(())
                } else {
                    Err(meta.error("expected `source`"))
                }
            })?;
        }
    }

    let description = match config.description {
        Some(Description::Static(description)) => quote! { #description },
        Some(Description::Field(field)) => {
            if !fields
                .iter()
                .any(|candidate| candidate.ident.as_ref() == Some(&field))
            {
                return Err(syn::Error::new_spanned(
                    field,
                    "delegated description field does not exist",
                ));
            }
            quote! { ::core::convert::AsRef::<str>::as_ref(&self.#field) }
        }
        None => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "missing `description` in `#[diagnosis(...)]`",
            ));
        }
    };

    let facade = facade_path();
    let code = option_literal(config.code);
    let help = option_literal(config.help);
    let url = option_literal(config.url);
    let severity = config
        .severity
        .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) });
    let source = source.map_or_else(
        || quote! { None },
        |source| {
            quote! {
                {
                    use #facade::IAsDiagnosis as _;
                    self.#source.as_diagnosis()
                }
            }
        },
    );

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #facade::IDiagnosis for #name #type_generics #where_clause {
            fn description(&self) -> &str {
                #description
            }

            fn source(&self) -> Option<&dyn #facade::IDiagnosis> {
                #source
            }

            fn code(&self) -> Option<&str> {
                #code
            }

            fn severity(&self) -> Option<#facade::Severity> {
                #severity
            }

            fn help(&self) -> Option<&str> {
                #help
            }

            fn url(&self) -> Option<&str> {
                #url
            }
        }
    })
}

fn parse_diagnosis_attributes(attrs: &[syn::Attribute]) -> Result<DiagnosisAttributes> {
    let mut config = DiagnosisAttributes::default();
    for attr in attrs {
        if !attr.path().is_ident("diagnosis") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("description") {
                if config.description.is_some() {
                    return Err(meta.error("duplicate `description` attribute"));
                }
                let value = meta.value()?;
                config.description = Some(if value.peek(LitStr) {
                    Description::Static(value.parse()?)
                } else {
                    Description::Field(value.parse()?)
                });
            } else if meta.path.is_ident("code") {
                if config.code.is_some() {
                    return Err(meta.error("duplicate `code` attribute"));
                }
                config.code = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("severity") {
                if config.severity.is_some() {
                    return Err(meta.error("duplicate `severity` attribute"));
                }
                config.severity = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("help") {
                if config.help.is_some() {
                    return Err(meta.error("duplicate `help` attribute"));
                }
                config.help = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("url") {
                if config.url.is_some() {
                    return Err(meta.error("duplicate `url` attribute"));
                }
                config.url = Some(meta.value()?.parse()?);
            } else {
                return Err(
                    meta.error("expected one of: `description`, `code`, `severity`, `help`, `url`")
                );
            }
            Ok(())
        })?;
    }
    Ok(config)
}

fn option_literal(value: Option<LitStr>) -> TokenStream2 {
    value.map_or_else(|| quote! { None }, |value| quote! { Some(#value) })
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use proc_macro2::TokenStream;

    use super::{expand_bail, expand_ensure, expand_idiagnosis, expand_mischief};

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
}
