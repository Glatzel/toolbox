use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Type};

pub(crate) struct DiagnosisMethods {
    pub(crate) description: TokenStream2,
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
    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::core::fmt::Display for #name #type_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(#description)
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

fn is_option(field_type: &Type) -> bool {
    let Type::Path(path) = field_type else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Option")
}
