use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, Field, Ident, LitStr, Member, Token, parenthesized};

#[derive(Clone, Default)]
pub(crate) struct DiagnosisAttributes {
    pub(crate) display: bool,
    pub(crate) error: bool,
    pub(crate) description: Option<Description>,
    pub(crate) code: Option<LitStr>,
    pub(crate) severity: Option<Expr>,
    pub(crate) help: Option<LitStr>,
    pub(crate) url: Option<LitStr>,
    pub(crate) transparent: Option<Transparent>,
}

#[derive(Clone)]
pub(crate) enum Description {
    Static(LitStr),
    Field(Member),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Transparent {
    Diagnosis,
    Foreign,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceKind {
    Diagnosis,
    Foreign,
}

#[derive(Clone, Copy)]
pub(crate) struct SourceField {
    pub(crate) index: usize,
    pub(crate) kind: SourceKind,
    pub(crate) _span: Span,
}

#[derive(Clone, Copy)]
pub(crate) struct FromField {
    pub(crate) index: usize,
    pub(crate) span: Span,
}

#[derive(Default)]
pub(crate) struct FieldRoles {
    pub(crate) source: Option<SourceField>,
    pub(crate) from: Option<FromField>,
}

impl DiagnosisAttributes {
    pub(crate) fn is_empty(&self) -> bool {
        self.description.is_none()
            && self.code.is_none()
            && self.severity.is_none()
            && self.help.is_none()
            && self.url.is_none()
            && !self.display
            && !self.error
            && self.transparent.is_none()
    }

    pub(crate) fn has_metadata(&self) -> bool {
        self.description.is_some()
            || self.code.is_some()
            || self.severity.is_some()
            || self.help.is_some()
            || self.url.is_some()
    }
}

pub(crate) fn parse_diagnosis_attributes(
    attrs: &[Attribute],
    allow_forward: bool,
    allow_type_flags: bool,
    allow_transparent: bool,
) -> Result<(DiagnosisAttributes, Option<Span>)> {
    let mut config = DiagnosisAttributes::default();
    let mut forward = None;
    for attr in attrs {
        if !attr.path().is_ident("diagnosis") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if allow_type_flags && meta.path.is_ident("display") {
                if config.display {
                    return Err(meta.error("duplicate `display` attribute"));
                }
                config.display = true;
            } else if allow_type_flags && meta.path.is_ident("error") {
                if config.error {
                    return Err(meta.error("duplicate `error` attribute"));
                }
                config.error = true;
            } else if allow_transparent && meta.path.is_ident("transparent") {
                if config.transparent.is_some() {
                    return Err(meta.error("duplicate `transparent` attribute"));
                }
                config.transparent = Some(if meta.input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in meta.input);
                    let kind: Ident = content.parse()?;
                    if kind != "foreign" {
                        return Err(syn::Error::new_spanned(
                            kind,
                            "unknown transparent argument; expected `foreign`",
                        ));
                    }
                    if !content.is_empty() {
                        return Err(content.error("unexpected transparent argument"));
                    }
                    Transparent::Foreign
                } else {
                    Transparent::Diagnosis
                });
            } else if meta.path.is_ident("description") {
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
                let expected = if allow_forward && allow_transparent {
                    "expected one of: `description`, `description_from`, `code`, `severity`, `help`, `url`, `forward`, `transparent`"
                } else if allow_forward {
                    "expected one of: `description`, `description_from`, `code`, `severity`, `help`, `url`, `forward`"
                } else if allow_type_flags && allow_transparent {
                    "expected one of: `display`, `error`, `description`, `description_from`, `code`, `severity`, `help`, `url`, `transparent`"
                } else if allow_type_flags {
                    "expected one of: `display`, `error`, `description`, `description_from`, `code`, `severity`, `help`, `url`"
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

pub(crate) fn parse_field_roles(fields: &[&Field]) -> Result<FieldRoles> {
    let mut roles = FieldRoles::default();
    for (index, field) in fields.iter().enumerate() {
        for attr in &field.attrs {
            if !attr.path().is_ident("diagnosis") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("source") {
                    if roles.source.is_some() {
                        return Err(meta.error("duplicate `source` field marker"));
                    }
                    let kind = if meta.input.peek(syn::token::Paren) {
                        let content;
                        parenthesized!(content in meta.input);
                        let kind: Ident = content.parse()?;
                        if kind != "foreign" {
                            return Err(syn::Error::new_spanned(
                                kind,
                                "unknown source argument; expected `foreign`",
                            ));
                        }
                        if !content.is_empty() {
                            content.parse::<Token![,]>()?;
                            if !content.is_empty() {
                                return Err(content.error("unexpected source argument"));
                            }
                        }
                        SourceKind::Foreign
                    } else {
                        SourceKind::Diagnosis
                    };
                    roles.source = Some(SourceField {
                        index,
                        kind,
                        _span: meta.path.span(),
                    });
                    Ok(())
                } else if meta.path.is_ident("from") {
                    if roles.from.is_some() {
                        return Err(meta.error("duplicate `from` field marker"));
                    }
                    roles.from = Some(FromField {
                        index,
                        span: meta.path.span(),
                    });
                    Ok(())
                } else {
                    Err(meta.error("expected `source` or `from`"))
                }
            })?;
        }
    }
    Ok(roles)
}

pub(crate) fn option_literal(value: Option<LitStr>) -> TokenStream2 {
    value.map_or_else(|| quote! { None }, |value| quote! { Some(#value) })
}
