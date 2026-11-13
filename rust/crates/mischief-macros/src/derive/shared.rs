use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Type};

pub(crate) struct DiagnosisMethods {
    pub(crate) description: TokenStream2,
    pub(crate) fmt_description: Option<TokenStream2>,
    pub(crate) source: TokenStream2,
    pub(crate) code: TokenStream2,
    pub(crate) severity: TokenStream2,
    pub(crate) help: TokenStream2,
    pub(crate) url: TokenStream2,
}

pub(crate) fn generate_idiagnosis_impl(
    input: &DeriveInput,
    facade: &TokenStream2,
    methods: DiagnosisMethods,
) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let description = methods.description;
    let fmt_description = methods.fmt_description.map(|body| {
        quote! {
            fn fmt_description(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                #body
            }
        }
    });
    let source = methods.source;
    let code = methods.code;
    let severity = methods.severity;
    let help = methods.help;
    let url = methods.url;
    quote! {
        impl #impl_generics #facade::IDiagnosis for #name #type_generics #where_clause {
            fn description(&self) -> &str {
                #description
            }

            #fmt_description

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
    }
}

pub(crate) fn generate_display_impl(
    input: &DeriveInput,
    description: TokenStream2,
) -> TokenStream2 {
    generate_display_body_impl(input, quote! { f.write_str(#description) })
}

pub(crate) fn generate_display_body_impl(input: &DeriveInput, body: TokenStream2) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::core::fmt::Display for #name #type_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #body
            }
        }
    }
}

pub(crate) fn generate_error_impl(input: &DeriveInput, source: TokenStream2) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::core::error::Error for #name #type_generics #where_clause {
            fn source(&self) -> Option<&(dyn ::core::error::Error + 'static)> {
                #source
            }
        }
    }
}

pub(crate) fn error_source_expression(
    facade: &TokenStream2,
    field_type: &Type,
    value: TokenStream2,
) -> TokenStream2 {
    if is_option(field_type) {
        quote! {
            match #value {
                Some(source) => {
                    #facade::IErrorSource::as_error_source(source)
                }
                None => None,
            }
        }
    } else {
        quote! { #facade::IErrorSource::as_error_source(#value) }
    }
}

pub(crate) fn foreign_error_source_expression(
    facade: &TokenStream2,
    field_type: &Type,
    value: TokenStream2,
) -> TokenStream2 {
    if is_option(field_type) {
        quote! {
            match #value {
                Some(source) => {
                    #facade::IForeignErrorSource::as_foreign_error_source(source)
                }
                None => None,
            }
        }
    } else {
        quote! { #facade::IForeignErrorSource::as_foreign_error_source(#value) }
    }
}

pub(crate) fn description_field_expression(value: TokenStream2) -> TokenStream2 {
    quote! { ::core::convert::AsRef::<str>::as_ref(#value) }
}

pub(crate) fn code_field_expression(field_type: &Type, value: TokenStream2) -> TokenStream2 {
    if is_option(field_type) {
        quote! {
            match #value {
                Some(code) => Some(::core::convert::AsRef::<str>::as_ref(code)),
                None => None,
            }
        }
    } else {
        quote! { Some(::core::convert::AsRef::<str>::as_ref(#value)) }
    }
}

pub(crate) fn generate_from_impl(
    input: &DeriveInput,
    field_type: &Type,
    body: TokenStream2,
) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::core::convert::From<#field_type> for #name #type_generics #where_clause {
            fn from(value: #field_type) -> Self {
                #body
            }
        }
    }
}

pub(crate) fn generate_into_report_impl(
    input: &DeriveInput,
    facade: &TokenStream2,
    report_expression: TokenStream2,
) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics #name #type_generics #where_clause {
            /// Converts this value into a mischief report.
            pub fn into_report(self) -> #facade::Report {
                extern crate alloc;

                fn copy_diagnosis(source: &dyn #facade::IDiagnosis) -> #facade::MischiefError {
                    use alloc::string::ToString as _;

                    let mut builder = #facade::MischiefError::builder(
                        #facade::DescriptionDisplay(source).to_string()
                    );
                    if let Some(code) = #facade::IDiagnosis::code(source) {
                        builder = builder.code(code.to_string());
                    }
                    if let Some(severity) = #facade::IDiagnosis::severity(source) {
                        builder = builder.severity(severity);
                    }
                    if let Some(help) = #facade::IDiagnosis::help(source) {
                        builder = builder.help(help.to_string());
                    }
                    if let Some(url) = #facade::IDiagnosis::url(source) {
                        builder = builder.url(url.to_string());
                    }
                    if let Some(source) = #facade::IDiagnosis::source(source) {
                        builder = builder.source(copy_diagnosis(source));
                    }
                    builder.build()
                }

                use alloc::string::ToString as _;

                let mut builder = #facade::MischiefError::builder(
                    #facade::IDiagnosis::description(&self).to_string()
                );
                if let Some(code) = #facade::IDiagnosis::code(&self) {
                    builder = builder.code(code.to_string());
                }
                if let Some(severity) = #facade::IDiagnosis::severity(&self) {
                    builder = builder.severity(severity);
                }
                if let Some(help) = #facade::IDiagnosis::help(&self) {
                    builder = builder.help(help.to_string());
                }
                if let Some(url) = #facade::IDiagnosis::url(&self) {
                    builder = builder.url(url.to_string());
                }

                #report_expression
            }
        }
    }
}

pub(crate) fn diagnosis_source_builder(field_type: &Type, value: TokenStream2) -> TokenStream2 {
    if is_option(field_type) {
        quote! {
            match #value {
                Some(source) => builder.source(copy_diagnosis(&source)),
                None => builder,
            }
        }
    } else {
        quote! { builder.source(copy_diagnosis(&#value)) }
    }
}

pub(crate) fn foreign_source_builder(field_type: &Type, value: TokenStream2) -> TokenStream2 {
    if is_option(field_type) {
        quote! {
            match #value {
                Some(source) => builder.source_error(source),
                None => builder,
            }
        }
    } else {
        quote! { builder.source_error(#value) }
    }
}

pub(crate) fn is_option(field_type: &Type) -> bool {
    let Type::Path(path) = field_type else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Option")
}
