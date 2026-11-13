use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Expr, Field, Fields, Ident, Index, LitStr, Member, Token};

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
/// Formatted descriptions, `Display` or `Error` generation, typed foreign-error
/// sources, and error mapping between enums are not supported.
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
pub fn IDiagnosis(input: TokenStream) -> TokenStream { expand_idiagnosis(input.into()).into() }

#[derive(Clone, Default)]
struct DiagnosisAttributes {
    description: Option<Description>,
    code: Option<LitStr>,
    severity: Option<Expr>,
    help: Option<LitStr>,
    url: Option<LitStr>,
}

#[derive(Clone)]
enum Description {
    Static(LitStr),
    Field(Member),
}

fn expand_idiagnosis(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<DeriveInput>(input).and_then(|input| derive_idiagnosis(&input)) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error(),
    }
}

fn derive_idiagnosis(input: &DeriveInput) -> Result<TokenStream2> {
    let (config, forward) = parse_diagnosis_attributes(&input.attrs, false)?;
    if let Some(forward) = forward {
        return Err(syn::Error::new(
            forward,
            "`forward` is only supported on enum variants",
        ));
    }
    match &input.data {
        Data::Struct(data) => derive_struct_idiagnosis(input, config, &data.fields),
        Data::Enum(data) => derive_enum_idiagnosis(input, config, data),
        Data::Union(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "IDiagnosis can only be derived for structs and enums",
        )),
    }
}

fn derive_struct_idiagnosis(
    input: &DeriveInput,
    config: DiagnosisAttributes,
    input_fields: &Fields,
) -> Result<TokenStream2> {
    let fields = match input_fields {
        Fields::Named(fields) => &fields.named,
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
            let field_name = match &field {
                Member::Named(field_name) => field_name,
                Member::Unnamed(_) => {
                    return Err(syn::Error::new_spanned(
                        &field,
                        "delegated description field does not exist",
                    ));
                }
            };
            if !fields
                .iter()
                .any(|candidate| candidate.ident.as_ref() == Some(field_name))
            {
                return Err(syn::Error::new_spanned(
                    field_name,
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

#[derive(Clone)]
struct FieldBinding {
    member: Member,
    binding: Ident,
}

struct EnumArm {
    pattern: TokenStream2,
    description: TokenStream2,
    source: TokenStream2,
    code: TokenStream2,
    severity: TokenStream2,
    help: TokenStream2,
    url: TokenStream2,
}

fn derive_enum_idiagnosis(
    input: &DeriveInput,
    defaults: DiagnosisAttributes,
    data: &syn::DataEnum,
) -> Result<TokenStream2> {
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "IDiagnosis cannot be derived for an empty enum",
        ));
    }

    let facade = facade_path();
    let mut arms = Vec::with_capacity(data.variants.len());
    for variant in &data.variants {
        let (config, forward) = parse_diagnosis_attributes(&variant.attrs, true)?;
        let fields: Vec<&Field> = variant.fields.iter().collect();
        let bindings: Vec<FieldBinding> = fields
            .iter()
            .enumerate()
            .map(|(index, field)| FieldBinding {
                member: field
                    .ident
                    .clone()
                    .map_or_else(|| Member::Unnamed(Index::from(index)), Member::Named),
                binding: format_ident!("__field_{index}", span = field.span()),
            })
            .collect();
        let pattern = enum_variant_pattern(variant, &fields, &bindings);

        if let Some(forward_span) = forward {
            if fields.len() != 1 {
                return Err(syn::Error::new(
                    forward_span,
                    "`forward` requires a variant with exactly one field",
                ));
            }
            let binding = &bindings[0].binding;
            arms.push(EnumArm {
                pattern,
                description: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.description()
                    }
                },
                source: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.source()
                    }
                },
                code: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.code()
                    }
                },
                severity: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.severity()
                    }
                },
                help: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.help()
                    }
                },
                url: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.url()
                    }
                },
            });
            continue;
        }

        let description = enum_description(&config, &defaults, &bindings, variant)?;
        let source_index = source_field_index(&fields)?;
        let source = source_index.map_or_else(
            || quote! { None },
            |index| {
                let binding = &bindings[index].binding;
                quote! {
                    {
                        use #facade::IAsDiagnosis as _;
                        #binding.as_diagnosis()
                    }
                }
            },
        );
        let code = option_literal(config.code.as_ref().or(defaults.code.as_ref()).cloned());
        let help = option_literal(config.help.as_ref().or(defaults.help.as_ref()).cloned());
        let url = option_literal(config.url.as_ref().or(defaults.url.as_ref()).cloned());
        let severity = config
            .severity
            .as_ref()
            .or(defaults.severity.as_ref())
            .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) });

        arms.push(EnumArm {
            pattern,
            description,
            source,
            code,
            severity,
            help,
            url,
        });
    }

    let description_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.description;
            quote! { #pattern => #expression }
        })
        .collect();
    let source_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.source;
            quote! { #pattern => #expression }
        })
        .collect();
    let code_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.code;
            quote! { #pattern => #expression }
        })
        .collect();
    let severity_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.severity;
            quote! { #pattern => #expression }
        })
        .collect();
    let help_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.help;
            quote! { #pattern => #expression }
        })
        .collect();
    let url_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.url;
            quote! { #pattern => #expression }
        })
        .collect();

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #facade::IDiagnosis for #name #type_generics #where_clause {
            fn description(&self) -> &str {
                match self {
                    #(#description_arms),*
                }
            }

            fn source(&self) -> Option<&dyn #facade::IDiagnosis> {
                match self {
                    #(#source_arms),*
                }
            }

            fn code(&self) -> Option<&str> {
                match self {
                    #(#code_arms),*
                }
            }

            fn severity(&self) -> Option<#facade::Severity> {
                match self {
                    #(#severity_arms),*
                }
            }

            fn help(&self) -> Option<&str> {
                match self {
                    #(#help_arms),*
                }
            }

            fn url(&self) -> Option<&str> {
                match self {
                    #(#url_arms),*
                }
            }
        }
    })
}

fn enum_variant_pattern(
    variant: &syn::Variant,
    fields: &[&Field],
    bindings: &[FieldBinding],
) -> TokenStream2 {
    let name = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! { Self::#name },
        Fields::Named(_) => {
            let fields = fields.iter().zip(bindings).map(|(field, binding)| {
                let name = field
                    .ident
                    .as_ref()
                    .expect("named enum fields have identifiers");
                let binding = &binding.binding;
                quote! { #name: #binding }
            });
            quote! { Self::#name { #(#fields),* } }
        }
        Fields::Unnamed(_) => {
            let bindings = bindings.iter().map(|binding| &binding.binding);
            quote! { Self::#name(#(#bindings),*) }
        }
    }
}

fn enum_description(
    config: &DiagnosisAttributes,
    defaults: &DiagnosisAttributes,
    bindings: &[FieldBinding],
    variant: &syn::Variant,
) -> Result<TokenStream2> {
    let description = config
        .description
        .as_ref()
        .or(defaults.description.as_ref())
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &variant.ident,
                "missing `description`, `description_from`, or `forward` on enum variant",
            )
        })?;

    match description {
        Description::Static(description) => Ok(quote! { #description }),
        Description::Field(member) => {
            let binding = binding_for_member(bindings, member).ok_or_else(|| {
                syn::Error::new_spanned(member, "delegated description field does not exist")
            })?;
            Ok(quote! { ::core::convert::AsRef::<str>::as_ref(#binding) })
        }
    }
}

fn binding_for_member<'a>(bindings: &'a [FieldBinding], member: &Member) -> Option<&'a Ident> {
    match member {
        Member::Named(member) => bindings.iter().find_map(|binding| match &binding.member {
            Member::Named(candidate) if candidate == member => Some(&binding.binding),
            _ => None,
        }),
        Member::Unnamed(member) => bindings
            .get(member.index as usize)
            .map(|binding| &binding.binding),
    }
}

fn source_field_index(fields: &[&Field]) -> Result<Option<usize>> {
    let mut source = None;
    for (index, field) in fields.iter().enumerate() {
        for attr in &field.attrs {
            if !attr.path().is_ident("diagnosis") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("source") {
                    if source.is_some() {
                        return Err(meta.error("duplicate `source` field marker"));
                    }
                    source = Some(index);
                    Ok(())
                } else {
                    Err(meta.error("expected `source`"))
                }
            })?;
        }
    }
    Ok(source)
}

fn parse_diagnosis_attributes(
    attrs: &[Attribute],
    allow_forward: bool,
) -> Result<(DiagnosisAttributes, Option<Span>)> {
    let mut config = DiagnosisAttributes::default();
    let mut forward = None;
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
            } else if meta.path.is_ident("description_from") {
                if config.description.is_some() {
                    return Err(meta.error("duplicate `description` attribute"));
                }
                config.description = Some(Description::Field(meta.value()?.parse()?));
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
            } else if allow_forward && meta.path.is_ident("forward") {
                if forward.is_some() {
                    return Err(meta.error("duplicate `forward` attribute"));
                }
                forward = Some(meta.path.span());
            } else {
                let expected = if allow_forward {
                    "expected one of: `description`, `description_from`, `code`, `severity`, `help`, `url`, `forward`"
                } else {
                    "expected one of: `description`, `description_from`, `code`, `severity`, `help`, `url`"
                };
                return Err(meta.error(expected));
            }
            Ok(())
        })?;
    }
    if let Some(forward) = forward {
        if !config.is_empty() {
            return Err(syn::Error::new(
                forward,
                "`forward` cannot be combined with other diagnosis attributes",
            ));
        }
    }
    Ok((config, forward))
}

impl DiagnosisAttributes {
    fn is_empty(&self) -> bool {
        self.description.is_none()
            && self.code.is_none()
            && self.severity.is_none()
            && self.help.is_none()
            && self.url.is_none()
    }
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
}
