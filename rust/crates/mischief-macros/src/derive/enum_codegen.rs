use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{DeriveInput, Field, Fields, Ident, Index, Member};

use super::attrs::{
    Description, DiagnosisAttributes, option_literal, parse_diagnosis_attributes,
    source_field_index,
};
use super::shared::{DiagnosisMethods, generate_idiagnosis_impl};
use crate::facade_path;

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
    let code_arms = method_arms(&arms, |arm| &arm.code);
    let severity_arms = method_arms(&arms, |arm| &arm.severity);
    let help_arms = method_arms(&arms, |arm| &arm.help);
    let url_arms = method_arms(&arms, |arm| &arm.url);

    Ok(generate_idiagnosis_impl(
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
    ))
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
