use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Result;
use syn::{DeriveInput, Fields, Index, Member};

use super::attrs::{
    DiagnosisAttributes, FieldRoles, SourceKind, Transparent, option_literal, parse_field_roles,
};
use super::description_template::{DescriptionTemplate, parse_description_template};
use super::shared::{
    DiagnosisMethods, code_field_expression, description_field_expression,
    diagnosis_source_builder, error_source_expression, foreign_error_source_expression,
    foreign_source_builder, generate_display_body_impl, generate_display_impl, generate_error_impl,
    generate_from_impl, generate_idiagnosis_impl, generate_into_report_impl, is_option,
};
use crate::facade_path;

pub(crate) fn derive_struct_idiagnosis(
    input: &DeriveInput,
    config: DiagnosisAttributes,
    input_fields: &Fields,
) -> Result<TokenStream2> {
    let fields = match input_fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "IDiagnosis can only be derived for structs with named or tuple fields",
            ));
        }
    };
    let field_refs: Vec<&syn::Field> = fields.iter().collect();
    let roles = parse_field_roles(&field_refs)?;
    if let Some(transparent) = config.transparent {
        return derive_transparent_struct(input, config, transparent, &field_refs, roles);
    }
    let description_field = roles.description;
    let code_field = roles.code;
    if let Some(field) = description_field {
        if is_option(&field_refs[field.index].ty) {
            return Err(syn::Error::new(
                field.span,
                "`description` field must not be an `Option`",
            ));
        }
    }
    if description_field.is_some() && config.description.is_some() {
        return Err(syn::Error::new(
            description_field.expect("description field marker").span,
            "`description` field marker cannot be combined with a `description` attribute",
        ));
    }
    if code_field.is_some() && config.code.is_some() {
        return Err(syn::Error::new(
            code_field.expect("code field marker").span,
            "`code` field marker cannot be combined with a `code` attribute",
        ));
    }
    let description_template = description_field
        .is_none()
        .then(|| struct_description_template(&config, &field_refs, input))
        .transpose()?;
    let description = description_field.map_or_else(
        || {
            let description = description_template
                .as_ref()
                .expect("description template")
                .raw();
            quote! { #description }
        },
        |field| {
            let field = field_access(&field_refs, field.index);
            description_field_expression(quote! { &#field })
        },
    );
    let fmt_description = description_template.as_ref().and_then(|template| {
        template
            .needs_formatting()
            .then(|| write_description_expression(template, &field_refs))
    });

    let resolved_facade = facade_path();
    let facade = &resolved_facade.path;
    let code = code_field.map_or_else(
        || option_literal(config.code),
        |field| {
            let field_type = &field_refs[field.index].ty;
            let field = field_access(&field_refs, field.index);
            code_field_expression(field_type, quote! { &#field })
        },
    );
    let help = option_literal(config.help);
    let url = option_literal(config.url);
    let severity = config
        .severity
        .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) });
    let source = match roles.source {
        Some(source) if source.kind == SourceKind::Foreign => quote! { None },
        Some(source) => {
            let field = field_access(&field_refs, source.index);
            quote! {
                {
                    use #facade::IAsDiagnosis as _;
                    #field.as_diagnosis()
                }
            }
        }
        None => quote! { None },
    };
    let error_source = roles.source.map_or_else(
        || quote! { None },
        |source| {
            let field = field_refs[source.index];
            let value = field_access(&field_refs, source.index);
            let value = quote! { &#value };
            match source.kind {
                SourceKind::Diagnosis => error_source_expression(facade, &field.ty, value),
                SourceKind::Foreign => foreign_error_source_expression(facade, &field.ty, value),
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
            Ok(generate_from_impl(
                input,
                &field.ty,
                field_constructor(&field_refs, from.index, quote! { value }),
            ))
        })
        .transpose()?;

    let mut expanded = generate_idiagnosis_impl(
        input,
        facade,
        DiagnosisMethods {
            description: quote! { #description },
            fmt_description,
            source,
            code,
            severity,
            help,
            url,
        },
    );
    if config.display {
        if description_field.is_some() {
            expanded.extend(generate_display_body_impl(
                input,
                quote! { f.write_str(#description) },
            ));
        } else if description_template
            .as_ref()
            .expect("description template")
            .needs_formatting()
        {
            expanded.extend(generate_display_body_impl(
                input,
                write_description_expression(
                    description_template.as_ref().expect("description template"),
                    &field_refs,
                ),
            ));
        } else {
            expanded.extend(generate_display_impl(input, quote! { #description }));
        }
    }
    if config.error {
        expanded.extend(generate_error_impl(input, error_source));
    }
    if let Some(from) = from {
        expanded.extend(from);
    }
    let description_update = description_template.as_ref().and_then(|template| {
        template
            .needs_formatting()
            .then(|| format_description_expression(template, &field_refs))
    });
    let report_expression = if let Some(source) = roles.source {
        let field = field_refs[source.index];
        let source_pattern = field_pattern(&field_refs, source.index);
        let builder_expression = match source.kind {
            SourceKind::Diagnosis => diagnosis_source_builder(&field.ty, quote! { source }),
            SourceKind::Foreign => foreign_source_builder(&field.ty, quote! { source }),
        };
        let description_update = description_update.as_ref().map_or_else(
            || quote! {},
            |formatted| {
                quote! {
                    let builder = builder.description(#formatted);
                }
            },
        );
        quote! {
            #description_update
            match self {
                #source_pattern => {
                    #facade::Report::new((#builder_expression).build())
                }
            }
        }
    } else {
        let description_update = description_update.as_ref().map_or_else(
            || quote! {},
            |formatted| {
                quote! {
                    let builder = builder.description(#formatted);
                }
            },
        );
        quote! {{
            #description_update
            #facade::Report::new(builder.build())
        }}
    };
    if resolved_facade.supports_report {
        expanded.extend(generate_into_report_impl(input, facade, report_expression));
    }
    Ok(expanded)
}

fn field_member(fields: &[&syn::Field], index: usize) -> Member {
    match fields[index].ident.as_ref() {
        Some(name) => Member::Named(name.clone()),
        None => Member::Unnamed(Index::from(index)),
    }
}

fn field_access(fields: &[&syn::Field], index: usize) -> TokenStream2 {
    let member = field_member(fields, index);
    quote! { self.#member }
}

fn field_constructor(fields: &[&syn::Field], index: usize, value: TokenStream2) -> TokenStream2 {
    match fields[index].ident.as_ref() {
        Some(name) => quote! { Self { #name: #value } },
        None => quote! { Self(#value) },
    }
}

fn field_pattern(fields: &[&syn::Field], index: usize) -> TokenStream2 {
    if let Some(name) = fields[index].ident.as_ref() {
        return quote! { Self { #name: source, .. } };
    }

    let fields = fields.iter().enumerate().map(|(field_index, _)| {
        if field_index == index {
            quote! { source }
        } else {
            quote! { _ }
        }
    });
    quote! { Self(#(#fields),*) }
}

fn struct_description_template(
    config: &DiagnosisAttributes,
    fields: &[&syn::Field],
    input: &DeriveInput,
) -> Result<DescriptionTemplate> {
    config.description.as_ref().map_or_else(
        || {
            Err(syn::Error::new_spanned(
                &input.ident,
                "missing `description` in `#[diagnosis(...)]`",
            ))
        },
        |description| parse_description_template(description, fields),
    )
}

fn write_description_expression(
    template: &DescriptionTemplate,
    fields: &[&syn::Field],
) -> TokenStream2 {
    let literal = template.formatted();
    let arguments: Vec<_> = template
        .fields()
        .iter()
        .map(|&index| field_access(fields, index))
        .collect();
    quote! {
        ::core::write!(f, #literal #(, #arguments)*)
    }
}

fn format_description_expression(
    template: &DescriptionTemplate,
    fields: &[&syn::Field],
) -> TokenStream2 {
    let literal = template.formatted();
    let arguments: Vec<_> = template
        .fields()
        .iter()
        .map(|&index| field_access(fields, index))
        .collect();
    quote! {
        alloc::format!(#literal #(, #arguments)*)
    }
}

fn derive_transparent_struct(
    input: &DeriveInput,
    config: DiagnosisAttributes,
    transparent: Transparent,
    fields: &[&syn::Field],
    roles: FieldRoles,
) -> Result<TokenStream2> {
    if fields.len() != 1 {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`transparent` requires a struct with exactly one field",
        ));
    }
    if !config.display || !config.error {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`transparent` requires the `display` and `error` flags",
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
    if transparent == Transparent::Diagnosis && config.has_metadata() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`transparent` cannot be combined with diagnostic metadata",
        ));
    }
    if transparent == Transparent::Foreign && config.description.is_none() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`transparent(foreign)` requires a description",
        ));
    }

    let foreign_description = if transparent == Transparent::Foreign {
        Some(struct_description_template(&config, fields, input)?)
    } else {
        None
    };
    let resolved_facade = facade_path();
    let facade = &resolved_facade.path;
    let field = fields[0];
    let inner = field_access(fields, 0);
    let methods = match transparent {
        Transparent::Diagnosis => DiagnosisMethods {
            description: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.description()
                }
            },
            fmt_description: Some(quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.fmt_description(f)
                }
            }),
            source: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.source()
                }
            },
            code: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.code()
                }
            },
            severity: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.severity()
                }
            },
            help: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.help()
                }
            },
            url: quote! {
                {
                    use #facade::IDiagnosis as _;
                    #inner.url()
                }
            },
        },
        Transparent::Foreign => {
            let description = foreign_description
                .as_ref()
                .expect("foreign transparency requires a description");
            let description_literal = description.raw();
            DiagnosisMethods {
                description: quote! { #description_literal },
                fmt_description: description
                    .needs_formatting()
                    .then(|| write_description_expression(description, fields)),
                source: quote! { None },
                code: option_literal(config.code.clone()),
                severity: config
                    .severity
                    .clone()
                    .map_or_else(|| quote! { None }, |severity| quote! { Some(#severity) }),
                help: option_literal(config.help.clone()),
                url: option_literal(config.url.clone()),
            }
        }
    };

    let mut expanded = generate_idiagnosis_impl(input, facade, methods);
    expanded.extend(generate_display_body_impl(
        input,
        quote! { ::core::fmt::Display::fmt(&#inner, f) },
    ));
    expanded.extend(generate_error_impl(
        input,
        quote! { ::core::error::Error::source(&#inner) },
    ));

    if let Some(from) = roles.from {
        expanded.extend(generate_from_impl(
            input,
            &field.ty,
            field_constructor(fields, 0, quote! { value }),
        ));
        let _ = from;
    }

    let report_expression = match transparent {
        Transparent::Diagnosis => quote! {
            #facade::Report::new(copy_diagnosis(&#inner))
        },
        Transparent::Foreign => quote! {
            #facade::Report::new(#facade::MischiefError::from_error(#inner))
        },
    };
    if resolved_facade.supports_report {
        expanded.extend(generate_into_report_impl(input, facade, report_expression));
    }
    Ok(expanded)
}
