use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{DeriveInput, Field, Fields, Ident, Index, Member};

use super::attrs::{
    Description, DiagnosisAttributes, SourceKind, option_literal, parse_diagnosis_attributes,
    parse_field_roles,
};
use super::shared::{
    DiagnosisMethods, diagnosis_source_builder, error_source_expression,
    foreign_error_source_expression, foreign_source_builder, generate_display_impl,
    generate_error_impl, generate_from_impl, generate_idiagnosis_impl, generate_into_report_impl,
};
use crate::facade_path;

#[derive(Clone)]
struct FieldBinding {
    member: Member,
    binding: Ident,
}

struct EnumArm {
    pattern: TokenStream2,
    description: TokenStream2,
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

    let facade = facade_path();
    let mut arms = Vec::with_capacity(data.variants.len());
    let mut from_impls = Vec::new();
    let mut from_types = HashMap::new();
    for variant in &data.variants {
        let (config, forward) = parse_diagnosis_attributes(&variant.attrs, true, false)?;
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

        let description = enum_description(&config, &defaults, &bindings, variant)?;
        let display = if defaults.display {
            Some(enum_static_description(&config, &defaults, variant)?)
        } else {
            None
        };
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
            display,
            source,
            error_source,
            report: roles.source.map_or_else(
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
            ),
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
        expanded.extend(generate_display_impl(
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

fn enum_static_description(
    config: &DiagnosisAttributes,
    defaults: &DiagnosisAttributes,
    variant: &syn::Variant,
) -> Result<TokenStream2> {
    match config
        .description
        .as_ref()
        .or(defaults.description.as_ref())
    {
        Some(Description::Static(description)) => Ok(quote! { #description }),
        Some(Description::Field(_)) | None => Err(syn::Error::new_spanned(
            &variant.ident,
            "`display` requires a static description for every variant",
        )),
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
