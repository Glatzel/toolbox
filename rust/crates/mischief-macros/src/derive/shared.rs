use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

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
