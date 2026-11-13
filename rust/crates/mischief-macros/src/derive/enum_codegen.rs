use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{DeriveInput, Field, Fields, Ident, LitStr};

use super::attrs::{
    DiagnosisAttributes, SourceKind, Transparent, option_literal, parse_diagnosis_attributes,
    parse_field_roles,
};
use super::description_template::{
    DescriptionTemplate, parse_default_description_template, parse_description_template,
};
use super::shared::{
    DiagnosisMethods, code_field_expression, description_field_expression,
    diagnosis_source_builder, error_source_expression, foreign_error_source_expression,
    foreign_source_builder, generate_display_body_impl, generate_error_impl, generate_from_impl,
    generate_idiagnosis_impl, generate_into_report_impl, is_option,
};
use crate::facade_path;

struct FieldBinding {
    binding: Ident,
}

struct EnumArm {
    pattern: TokenStream2,
    description: TokenStream2,
    fmt_description: Option<TokenStream2>,
    display: Option<TokenStream2>,
    source: TokenStream2,
    error_source: TokenStream2,
    report: TokenStream2,
    code: TokenStream2,
    severity: TokenStream2,
    help: TokenStream2,
    url: TokenStream2,
}

pub(crate) fn derive_enum_idiagnosis(
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

    let default_description = parse_enum_default_description(&defaults)?;
    let facade = facade_path();
    let mut arms = Vec::with_capacity(data.variants.len());
    let mut from_impls = Vec::new();
    let mut from_types = HashMap::new();
    for variant in &data.variants {
        let (config, forward) = parse_diagnosis_attributes(&variant.attrs, true, false, true)?;
        let fields: Vec<&Field> = variant.fields.iter().collect();
        let bindings: Vec<FieldBinding> = fields
            .iter()
            .enumerate()
            .map(|(index, field)| FieldBinding {
                binding: format_ident!("__field_{index}", span = field.span()),
            })
            .collect();
        let pattern = enum_variant_pattern(variant, &fields, &bindings);
        let roles = parse_field_roles(&fields)?;
        let from_impl = roles
            .from
            .map(|from| {
                if fields.len() != 1 {
                    return Err(syn::Error::new(
                        from.span,
                        "`from` requires a variant with exactly one field",
                    ));
                }
                let field = fields[0];
                let field_type = field.ty.to_token_stream().to_string();
                if from_types.insert(field_type, from.span).is_some() {
                    return Err(syn::Error::new(from.span, "duplicate `from` field type"));
                }
                let variant_name = &variant.ident;
                let body = match &variant.fields {
                    Fields::Named(_) => {
                        let name = field.ident.as_ref().expect("named field");
                        quote! { Self::#variant_name { #name: value } }
                    }
                    Fields::Unnamed(_) => quote! { Self::#variant_name(value) },
                    Fields::Unit => unreachable!("unit variants have no from field"),
                };
                Ok(generate_from_impl(input, &field.ty, body))
            })
            .transpose()?;
        if let Some(from_impl) = from_impl {
            from_impls.push(from_impl);
        }

        if let Some(transparent) = config.transparent {
            if fields.len() != 1 {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    "`transparent` requires a variant with exactly one field",
                ));
            }
            if !defaults.display || !defaults.error {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    "`transparent` requires the `display` and `error` flags",
                ));
            }
            if config.has_metadata() {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    "`transparent` cannot be combined with diagnostic metadata",
                ));
            }
            if let Some(source) = roles.source {
                return Err(syn::Error::new(
                    source._span,
                    "`transparent` cannot be combined with `source`",
                ));
            }
            if let Some(description) = roles.description {
                return Err(syn::Error::new(
                    description.span,
                    "`transparent` cannot be combined with a `description` field marker",
                ));
            }
            if let Some(code) = roles.code {
                return Err(syn::Error::new(
                    code.span,
                    "`transparent` cannot be combined with a `code` field marker",
                ));
            }

            let binding = &bindings[0].binding;
            let methods = match transparent {
                Transparent::Diagnosis => DiagnosisMethods {
                    description: quote! {
                        {
                            use #facade::IDiagnosis as _;
                            #binding.description()
                        }
                    },
                    fmt_description: Some(quote! {
                        {
                            use #facade::IDiagnosis as _;
                            #binding.fmt_description(f)
                        }
                    }),
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
                },
                Transparent::Foreign => {
                    let description = default_description.as_ref().map_or_else(
                        || LitStr::new(&variant.ident.to_string(), variant.ident.span()),
                        |description| description.raw().clone(),
                    );
                    DiagnosisMethods {
                        description: quote! { #description },
                        fmt_description: Some(quote! {
                            ::core::fmt::Display::fmt(#binding, f)
                        }),
                        source: quote! { None },
                        code: option_literal(defaults.code.clone()),
                        severity: defaults
                            .severity
                            .clone()
                            .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) }),
                        help: option_literal(defaults.help.clone()),
                        url: option_literal(defaults.url.clone()),
                    }
                }
            };
            arms.push(EnumArm {
                pattern,
                description: methods.description,
                fmt_description: methods.fmt_description,
                display: Some(quote! {
                    ::core::fmt::Display::fmt(#binding, f)
                }),
                source: methods.source,
                error_source: quote! {
                    ::core::error::Error::source(#binding)
                },
                report: match transparent {
                    Transparent::Diagnosis => quote! {
                        #facade::Report::new(copy_diagnosis(&#binding))
                    },
                    Transparent::Foreign => quote! {
                        #facade::Report::new(#facade::MischiefError::from_error(#binding))
                    },
                },
                code: methods.code,
                severity: methods.severity,
                help: methods.help,
                url: methods.url,
            });
            continue;
        }

        if let Some(forward_span) = forward {
            if fields.len() != 1 {
                return Err(syn::Error::new(
                    forward_span,
                    "`forward` requires a variant with exactly one field",
                ));
            }
            if defaults.display {
                return Err(syn::Error::new(
                    forward_span,
                    "`display` requires a static description for every variant",
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
                fmt_description: Some(quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.fmt_description(f)
                    }
                }),
                display: None,
                source: quote! {
                    {
                        use #facade::IDiagnosis as _;
                        #binding.source()
                    }
                },
                error_source: quote! { None },
                report: quote! {
                    #facade::Report::new(copy_diagnosis(&#binding))
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

        if let Some(description) = roles.description {
            if config.description.is_some() {
                return Err(syn::Error::new(
                    description.span,
                    "`description` field marker cannot be combined with a `description` attribute",
                ));
            }
            if is_option(&fields[description.index].ty) {
                return Err(syn::Error::new(
                    description.span,
                    "`description` field must not be an `Option`",
                ));
            }
        }
        if let Some(code) = roles.code {
            if config.code.is_some() {
                return Err(syn::Error::new(
                    code.span,
                    "`code` field marker cannot be combined with a `code` attribute",
                ));
            }
        }

        let template = roles
            .description
            .is_none()
            .then(|| {
                variant_description_template(
                    &config,
                    default_description.as_ref(),
                    &fields,
                    variant,
                )
            })
            .transpose()?;
        let description = roles.description.map_or_else(
            || {
                let description = template.as_ref().expect("description template").raw();
                quote! { #description }
            },
            |field| {
                let binding = &bindings[field.index].binding;
                description_field_expression(quote! { #binding })
            },
        );
        let display = if defaults.display {
            if roles.description.is_none()
                && template
                    .as_ref()
                    .expect("description template")
                    .needs_formatting()
            {
                Some(write_description_expression(
                    template.as_ref().expect("description template"),
                    &bindings,
                ))
            } else {
                Some(quote! { f.write_str(#description) })
            }
        } else {
            None
        };
        let fmt_description = template.as_ref().and_then(|template| {
            template
                .needs_formatting()
                .then(|| write_description_expression(template, &bindings))
        });
        let source = roles.source.map_or_else(
            || quote! { None },
            |source| {
                if source.kind == SourceKind::Foreign {
                    return quote! { None };
                }
                let binding = &bindings[source.index].binding;
                quote! {
                    {
                        use #facade::IAsDiagnosis as _;
                        #binding.as_diagnosis()
                    }
                }
            },
        );
        let error_source = roles.source.map_or_else(
            || quote! { None },
            |source| {
                let field = fields[source.index];
                let binding = &bindings[source.index].binding;
                match source.kind {
                    SourceKind::Diagnosis => {
                        error_source_expression(&facade, &field.ty, quote! { #binding })
                    }
                    SourceKind::Foreign => {
                        foreign_error_source_expression(&facade, &field.ty, quote! { #binding })
                    }
                }
            },
        );
        let code = roles.code.map_or_else(
            || option_literal(config.code.as_ref().or(defaults.code.as_ref()).cloned()),
            |code| {
                let field = fields[code.index];
                let binding = &bindings[code.index].binding;
                code_field_expression(&field.ty, quote! { #binding })
            },
        );
        let help = option_literal(config.help.as_ref().or(defaults.help.as_ref()).cloned());
        let url = option_literal(config.url.as_ref().or(defaults.url.as_ref()).cloned());
        let severity = config
            .severity
            .as_ref()
            .or(defaults.severity.as_ref())
            .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) });

        let report = if template
            .as_ref()
            .is_some_and(DescriptionTemplate::needs_formatting)
        {
            let template = template.as_ref().expect("description template");
            let formatted = format_description_expression(&template, &bindings);
            let builder = quote! {
                let builder = builder.description(#formatted);
            };
            roles.source.map_or_else(
                || {
                    quote! {{
                        #builder
                        #facade::Report::new(builder.build())
                    }}
                },
                |source| {
                    let field = fields[source.index];
                    let binding = &bindings[source.index].binding;
                    let builder_expression = match source.kind {
                        SourceKind::Diagnosis => {
                            diagnosis_source_builder(&field.ty, quote! { #binding })
                        }
                        SourceKind::Foreign => {
                            foreign_source_builder(&field.ty, quote! { #binding })
                        }
                    };
                    quote! {{
                        #builder
                        #facade::Report::new((#builder_expression).build())
                    }}
                },
            )
        } else {
            roles.source.map_or_else(
                || quote! { #facade::Report::new(builder.build()) },
                |source| {
                    let field = fields[source.index];
                    let binding = &bindings[source.index].binding;
                    let builder_expression = match source.kind {
                        SourceKind::Diagnosis => {
                            diagnosis_source_builder(&field.ty, quote! { #binding })
                        }
                        SourceKind::Foreign => {
                            foreign_source_builder(&field.ty, quote! { #binding })
                        }
                    };
                    quote! {
                        #facade::Report::new((#builder_expression).build())
                    }
                },
            )
        };

        arms.push(EnumArm {
            pattern,
            description: quote! { #description },
            display,
            source,
            fmt_description,
            error_source,
            report,
            code,
            severity,
            help,
            url,
        });
    }

    fn method_arms<'a>(
        arms: &'a [EnumArm],
        method: impl Fn(&'a EnumArm) -> &'a TokenStream2,
    ) -> Vec<TokenStream2> {
        arms.iter()
            .map(|arm| {
                let pattern = &arm.pattern;
                let expression = method(arm);
                quote! { #pattern => #expression }
            })
            .collect()
    }

    let description_arms = method_arms(&arms, |arm| &arm.description);
    let has_fmt_description = arms.iter().any(|arm| arm.fmt_description.is_some());
    let fmt_description_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = if let Some(expression) = &arm.fmt_description {
                quote! { #expression }
            } else {
                let description = &arm.description;
                quote! { f.write_str(#description) }
            };
            quote! { #pattern => #expression }
        })
        .collect();
    let source_arms = method_arms(&arms, |arm| &arm.source);
    let error_source_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.error_source;
            quote! { #pattern => #expression }
        })
        .collect();
    let code_arms = method_arms(&arms, |arm| &arm.code);
    let severity_arms = method_arms(&arms, |arm| &arm.severity);
    let help_arms = method_arms(&arms, |arm| &arm.help);
    let url_arms = method_arms(&arms, |arm| &arm.url);

    let mut expanded = generate_idiagnosis_impl(
        input,
        &facade,
        DiagnosisMethods {
            description: quote! {
                match self {
                    #(#description_arms),*
                }
            },
            fmt_description: if has_fmt_description {
                Some(quote! {
                    match self {
                        #(#fmt_description_arms),*
                    }
                })
            } else {
                None
            },
            source: quote! {
                match self {
                    #(#source_arms),*
                }
            },
            code: quote! {
                match self {
                    #(#code_arms),*
                }
            },
            severity: quote! {
                match self {
                    #(#severity_arms),*
                }
            },
            help: quote! {
                match self {
                    #(#help_arms),*
                }
            },
            url: quote! {
                match self {
                    #(#url_arms),*
                }
            },
        },
    );
    if defaults.display {
        let display_arms: Vec<_> = arms
            .iter()
            .map(|arm| {
                let pattern = &arm.pattern;
                let expression = arm
                    .display
                    .as_ref()
                    .expect("display arms are generated for every normal variant");
                quote! { #pattern => #expression }
            })
            .collect();
        expanded.extend(generate_display_body_impl(
            input,
            quote! {
                match self {
                    #(#display_arms),*
                }
            },
        ));
    }
    if defaults.error {
        expanded.extend(generate_error_impl(
            input,
            quote! {
                match self {
                    #(#error_source_arms),*
                }
            },
        ));
    }
    let report_arms: Vec<_> = arms
        .iter()
        .map(|arm| {
            let pattern = &arm.pattern;
            let expression = &arm.report;
            quote! { #pattern => #expression }
        })
        .collect();
    expanded.extend(generate_into_report_impl(
        input,
        &facade,
        quote! {
            match self {
                #(#report_arms),*
            }
        },
    ));
    for from_impl in from_impls {
        expanded.extend(from_impl);
    }
    Ok(expanded)
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

fn parse_enum_default_description(
    defaults: &DiagnosisAttributes,
) -> Result<Option<DescriptionTemplate>> {
    defaults
        .description
        .as_ref()
        .map(parse_default_description_template)
        .transpose()
}

fn variant_description_template(
    config: &DiagnosisAttributes,
    default: Option<&DescriptionTemplate>,
    fields: &[&Field],
    variant: &syn::Variant,
) -> Result<DescriptionTemplate> {
    config.description.as_ref().map_or_else(
        || {
            default.cloned().ok_or_else(|| {
                syn::Error::new_spanned(
                    &variant.ident,
                    "missing `description`, `description` field marker, or `forward` on enum variant",
                )
            })
        },
        |description| parse_description_template(description, fields),
    )
}

fn write_description_expression(
    template: &DescriptionTemplate,
    bindings: &[FieldBinding],
) -> TokenStream2 {
    let literal = template.formatted();
    let arguments: Vec<_> = template
        .fields()
        .iter()
        .map(|&index| &bindings[index].binding)
        .collect();
    quote! {
        ::core::write!(f, #literal #(, #arguments)*)
    }
}

fn format_description_expression(
    template: &DescriptionTemplate,
    bindings: &[FieldBinding],
) -> TokenStream2 {
    let literal = template.formatted();
    let arguments: Vec<_> = template
        .fields()
        .iter()
        .map(|&index| &bindings[index].binding)
        .collect();
    quote! {
        alloc::format!(#literal #(, #arguments)*)
    }
}
