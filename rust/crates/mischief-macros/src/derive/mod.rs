mod attrs;
mod enum_codegen;
mod shared;
mod struct_codegen;

use attrs::parse_diagnosis_attributes;
use enum_codegen::derive_enum_idiagnosis;
use proc_macro2::TokenStream as TokenStream2;
use struct_codegen::derive_struct_idiagnosis;
use syn::parse::Result;
use syn::{Data, DeriveInput};

pub(crate) fn expand_idiagnosis(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<DeriveInput>(input).and_then(|input| derive_idiagnosis(&input)) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error(),
    }
}

fn derive_idiagnosis(input: &DeriveInput) -> Result<TokenStream2> {
    let (config, forward) = parse_diagnosis_attributes(&input.attrs, false, true)?;
    if let Some(forward) = forward {
        return Err(syn::Error::new(
            forward,
            "`forward` is only supported on enum variants",
        ));
    }
    match &input.data {
        Data::Struct(data) => derive_struct_idiagnosis(input, config, &data.fields),
        Data::Enum(data) => derive_enum_idiagnosis(input, config, data),
        Data::Union(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "IDiagnosis can only be derived for structs and enums",
        )),
    }
}
