use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, Field, LitStr, Member};

#[derive(Clone, Default)]
pub(crate) struct DiagnosisAttributes {
    pub(crate) description: Option<Description>,
    pub(crate) code: Option<LitStr>,
    pub(crate) severity: Option<Expr>,
    pub(crate) help: Option<LitStr>,
    pub(crate) url: Option<LitStr>,
}

#[derive(Clone)]
pub(crate) enum Description {
    Static(LitStr),
    Field(Member),
}

impl DiagnosisAttributes {
    pub(crate) fn is_empty(&self) -> bool {
        self.description.is_none()
            && self.code.is_none()
            && self.severity.is_none()
            && self.help.is_none()
            && self.url.is_none()
    }
}

pub(crate) fn parse_diagnosis_attributes(
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

pub(crate) fn source_field_index(fields: &[&Field]) -> Result<Option<usize>> {
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

pub(crate) fn option_literal(value: Option<LitStr>) -> TokenStream2 {
    value.map_or_else(|| quote! { None }, |value| quote! { Some(#value) })
}
