use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Result;
use syn::{DeriveInput, Fields, Member};

use super::attrs::{Description, DiagnosisAttributes, option_literal, source_field_index};
use super::shared::{
    DiagnosisMethods, error_source_expression, generate_display_impl, generate_error_impl,
    generate_idiagnosis_impl,
};
use crate::facade_path;

pub(crate) fn derive_struct_idiagnosis(
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
    let field_refs: Vec<&syn::Field> = fields.iter().collect();
    let source = source_field_index(&field_refs)?
        .map(|index| field_refs[index].ident.clone().expect("named field"));

    let display_description = if config.display {
        match &config.description {
            Some(Description::Static(description)) => Some(quote! { #description }),
            Some(Description::Field(_)) | None => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "`display` requires a static description",
                ));
            }
        }
    } else {
        None
    };

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
    let error_source = source_field_index(&field_refs)?.map_or_else(
        || quote! { None },
        |index| {
            let field = field_refs[index];
            let source = field.ident.as_ref().expect("named field");
            error_source_expression(&facade, &field.ty, quote! { &self.#source })
        },
    );

    let mut expanded = generate_idiagnosis_impl(
        input,
        &facade,
        DiagnosisMethods {
            description,
            source,
            code,
            severity,
            help,
            url,
        },
    );
    if let Some(description) = display_description {
        expanded.extend(generate_display_impl(input, description));
    }
    if config.error {
        expanded.extend(generate_error_impl(input, error_source));
    }
    Ok(expanded)
}
