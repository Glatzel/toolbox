use syn::parse::Result;
use syn::{Field, LitStr};

#[derive(Clone)]
pub(crate) struct DescriptionTemplate {
    raw: LitStr,
    formatted: LitStr,
    fields: Vec<usize>,
    needs_formatting: bool,
}

impl DescriptionTemplate {
    pub(crate) fn raw(&self) -> &LitStr { &self.raw }

    pub(crate) fn formatted(&self) -> &LitStr { &self.formatted }

    pub(crate) fn fields(&self) -> &[usize] { &self.fields }

    pub(crate) fn needs_formatting(&self) -> bool { self.needs_formatting }
}

pub(crate) fn parse_description_template(
    template: &LitStr,
    fields: &[&Field],
) -> Result<DescriptionTemplate> {
    parse_description_template_inner(template, fields, true)
}

pub(crate) fn parse_default_description_template(template: &LitStr) -> Result<DescriptionTemplate> {
    parse_description_template_inner(template, &[], false)
}

fn parse_description_template_inner(
    template: &LitStr,
    fields: &[&Field],
    allow_placeholders: bool,
) -> Result<DescriptionTemplate> {
    let source = template.value();
    let chars: Vec<char> = source.chars().collect();
    let mut formatted = String::with_capacity(source.len());
    let mut used_fields = Vec::new();
    let mut needs_formatting = false;
    let mut index = 0;

    while index < chars.len() {
        match chars[index] {
            '{' if chars.get(index + 1) == Some(&'{') => {
                formatted.push_str("{{");
                needs_formatting = true;
                index += 2;
            }
            '{' => {
                if !allow_placeholders {
                    return Err(syn::Error::new(
                        template.span(),
                        "enum-level default description cannot contain placeholders",
                    ));
                }
                let start = index;
                index += 1;
                let mut placeholder = String::new();
                loop {
                    match chars.get(index) {
                        Some('}') => break,
                        Some('{') => {
                            return Err(syn::Error::new(
                                template.span(),
                                "malformed description template: nested `{` in placeholder",
                            ));
                        }
                        Some(ch) => {
                            placeholder.push(*ch);
                            index += 1;
                        }
                        None => {
                            return Err(syn::Error::new(
                                template.span(),
                                "malformed description template: unclosed `{`",
                            ));
                        }
                    }
                }
                if index == start + 1 {
                    return Err(syn::Error::new(
                        template.span(),
                        "description template placeholder must name a field",
                    ));
                }

                let (field, spec) = placeholder
                    .split_once(':')
                    .map_or((placeholder.as_str(), None), |(field, spec)| {
                        (field, Some(spec))
                    });
                if field.is_empty() {
                    return Err(syn::Error::new(
                        template.span(),
                        "description template placeholder must name a field",
                    ));
                }
                if spec == Some("") {
                    return Err(syn::Error::new(
                        template.span(),
                        "description template format spec cannot be empty",
                    ));
                }

                let field_index = resolve_field(template, fields, field)?;
                let argument_index = if let Some(position) = used_fields
                    .iter()
                    .position(|candidate| *candidate == field_index)
                {
                    position
                } else {
                    used_fields.push(field_index);
                    used_fields.len() - 1
                };

                formatted.push('{');
                formatted.push_str(&argument_index.to_string());
                if let Some(spec) = spec {
                    formatted.push(':');
                    formatted.push_str(spec);
                }
                formatted.push('}');
                needs_formatting = true;
                index += 1;
            }
            '}' if chars.get(index + 1) == Some(&'}') => {
                formatted.push_str("}}");
                needs_formatting = true;
                index += 2;
            }
            '}' => {
                return Err(syn::Error::new(
                    template.span(),
                    "malformed description template: unmatched `}`",
                ));
            }
            ch => {
                formatted.push(ch);
                index += 1;
            }
        }
    }

    Ok(DescriptionTemplate {
        raw: template.clone(),
        formatted: LitStr::new(&formatted, template.span()),
        fields: used_fields,
        needs_formatting,
    })
}

fn resolve_field(template: &LitStr, fields: &[&Field], field: &str) -> Result<usize> {
    if field.chars().all(|ch| ch.is_ascii_digit()) {
        let index = field.parse::<usize>().map_err(|_| {
            syn::Error::new(
                template.span(),
                format!("description template tuple index `{field}` is too large"),
            )
        })?;
        if index < fields.len() && fields[index].ident.is_none() {
            return Ok(index);
        }
        return Err(syn::Error::new(
            template.span(),
            format!("description template tuple index `{field}` is out of range"),
        ));
    }

    if syn::parse_str::<syn::Ident>(field).is_ok() {
        if let Some(index) = fields
            .iter()
            .position(|candidate| candidate.ident.as_ref().is_some_and(|name| name == field))
        {
            return Ok(index);
        }
    }

    Err(syn::Error::new(
        template.span(),
        format!("description template field `{field}` does not exist"),
    ))
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    fn named_fields() -> Vec<Field> {
        let fields: syn::FieldsNamed = parse_quote! {{
            name: String,
            count: usize,
        }};
        fields.named.into_iter().collect()
    }

    fn tuple_fields() -> Vec<Field> {
        let fields: syn::FieldsUnnamed = parse_quote! {(
            String,
            u8,
        )};
        fields.unnamed.into_iter().collect()
    }

    #[test]
    fn rewrites_named_fields_and_keeps_escaped_braces() {
        let fields = named_fields();
        let fields: Vec<&Field> = fields.iter().collect();
        let template: LitStr = parse_quote! {"{{name}} {name:>8} {count}"};

        let parsed = parse_description_template(&template, &fields).expect("valid template");

        assert_eq!(parsed.raw().value(), "{{name}} {name:>8} {count}");
        assert_eq!(parsed.formatted().value(), "{{name}} {0:>8} {1}");
        assert_eq!(parsed.fields(), &[0, 1]);
        assert!(parsed.needs_formatting());
    }

    #[test]
    fn rewrites_debug_and_tuple_fields() {
        let fields = tuple_fields();
        let fields: Vec<&Field> = fields.iter().collect();
        let template: LitStr = parse_quote! {"{1:?} {0}"};

        let parsed = parse_description_template(&template, &fields).expect("valid template");

        assert_eq!(parsed.formatted().value(), "{0:?} {1}");
        assert_eq!(parsed.fields(), &[1, 0]);
    }

    #[test]
    fn plain_description_does_not_require_formatting() {
        let fields = named_fields();
        let fields: Vec<&Field> = fields.iter().collect();
        let template: LitStr = parse_quote! {"plain description"};

        let parsed = parse_description_template(&template, &fields).expect("valid description");

        assert_eq!(parsed.raw().value(), "plain description");
        assert_eq!(parsed.formatted().value(), "plain description");
        assert!(parsed.fields().is_empty());
        assert!(!parsed.needs_formatting());
    }

    #[test]
    fn rejects_unknown_fields() {
        let fields = named_fields();
        let fields: Vec<&Field> = fields.iter().collect();
        let template: LitStr = parse_quote! {"{missing}"};

        let error = parse_description_template(&template, &fields)
            .err()
            .expect("unknown field must fail");

        assert_eq!(
            error.to_string(),
            "description template field `missing` does not exist"
        );
    }
}
