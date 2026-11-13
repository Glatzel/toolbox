use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Result;
use syn::{DeriveInput, Fields, Member};

use super::attrs::{
    Description, DiagnosisAttributes, SourceKind, option_literal, parse_field_roles,
};
use super::shared::{
    DiagnosisMethods, diagnosis_source_builder, error_source_expression,
    foreign_error_source_expression, foreign_source_builder, generate_display_impl,
    generate_error_impl, generate_from_impl, generate_idiagnosis_impl, generate_into_report_impl,
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
    let roles = parse_field_roles(&field_refs)?;
    let source = roles
        .source
        .map(|source| field_refs[source.index].ident.clone().expect("named field"));

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
    let source = match (roles.source, source) {
        (Some(source), _) if source.kind == SourceKind::Foreign => quote! { None },
        (Some(_), Some(source)) => {
            quote! {
                {
                    use #facade::IAsDiagnosis as _;
                    self.#source.as_diagnosis()
                }
            }
        }
        (None, _) => quote! { None },
        (Some(_), None) => unreachable!("source field was resolved above"),
    };
    let error_source = roles.source.map_or_else(
        || quote! { None },
        |source| {
            let field = field_refs[source.index];
            let name = field.ident.as_ref().expect("named field");
            let value = quote! { &self.#name };
            match source.kind {
                SourceKind::Diagnosis => error_source_expression(&facade, &field.ty, value),
                SourceKind::Foreign => foreign_error_source_expression(&facade, &field.ty, value),
            }
        },
    );
    let from = roles
        .from
        .map(|from| {
            if field_refs.len() != 1 {
                return Err(syn::Error::new(
                    from.span,
                    "`from` requires a struct with exactly one field",
                ));
            }
            let field = field_refs[from.index];
            let field_name = field.ident.as_ref().expect("named field");
            Ok(generate_from_impl(
                input,
                &field.ty,
                quote! { Self { #field_name: value } },
            ))
        })
        .transpose()?;

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
    if let Some(from) = from {
        expanded.extend(from);
    }
    let report_expression = roles.source.map_or_else(
        || quote! { #facade::Report::new(builder.build()) },
        |source| {
            let field = field_refs[source.index];
            let field_name = field.ident.as_ref().expect("named field");
            let builder_expression = match source.kind {
                SourceKind::Diagnosis => diagnosis_source_builder(&field.ty, quote! { source }),
                SourceKind::Foreign => foreign_source_builder(&field.ty, quote! { source }),
            };
            quote! {
                match self {
                    Self { #field_name: source, .. } => {
                        #facade::Report::new((#builder_expression).build())
                    }
                }
            }
        },
    );
    expanded.extend(generate_into_report_impl(input, &facade, report_expression));
    Ok(expanded)
}
