use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};

/// Internal structure used to parse the arguments of `mischief!`.
///
/// Supports:
/// - Positional format arguments for the error description.
/// - Optional named fields: `code`, `severity`, `help`, `url`.
struct MischiefErrorInput {
    description_lit: LitStr,
    description_args: Punctuated<Expr, Token![,]>,
    code: Option<Expr>,
    severity: Option<Expr>,
    help: Option<Expr>,
    url: Option<Expr>,
}

impl Parse for MischiefErrorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // The first argument must always be a string literal for the description.
        let description_lit = input.parse()?;
        let mut description_args = Punctuated::new();

        let mut code = None;
        let mut severity = None;
        let mut help = None;
        let mut url = None;
        let mut seen_named_arg = false;

        while !input.is_empty() {
            input.parse::<Token![,]>()?; // consume comma
            if input.is_empty() {
                break; // allow trailing comma
            }

            // Named fields: key = value
            if input.peek(Ident) && input.peek2(Token![=]) {
                seen_named_arg = true;
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let value: Expr = input.parse()?;

                match key.to_string().as_str() {
                    "code" if code.is_none() => code = Some(value),
                    "severity" if severity.is_none() => severity = Some(value),
                    "help" if help.is_none() => help = Some(value),
                    "url" if url.is_none() => url = Some(value),
                    "source" | "code" | "severity" | "help" | "url" => {
                        return Err(syn::Error::new(
                            key.span(),
                            format!("duplicate field `{}`", key),
                        ));
                    }
                    _ => {
                        return Err(syn::Error::new(
                            key.span(),
                            "unexpected field, expected one of: `source`, `code`, `severity`, `help`, `url`",
                        ));
                    }
                }
            } else {
                // Positional format arguments
                if seen_named_arg {
                    return Err(syn::Error::new(
                        input.span(),
                        "positional arguments cannot follow named arguments",
                    ));
                }
                description_args.push(input.parse()?);
            }
        }

        Ok(MischiefErrorInput {
            description_lit,
            description_args,
            code,
            severity,
            help,
            url,
        })
    }
}

/// Procedural macro to create a `MischiefError` wrapped in a `Report`.
///
/// # Syntax
///
/// ```text
/// mischief!("format string", positional_args..., code = ..., severity = ..., help = ..., url = ...)
/// ```
#[proc_macro]
pub fn mischief(input: TokenStream) -> TokenStream {
    let parsed_input = match syn::parse::<MischiefErrorInput>(input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let description_lit = parsed_input.description_lit;
    let description_args = parsed_input.description_args;

    let code = parsed_input
        .code
        .map_or(quote! { None }, |e| quote! { Some(#e.into()) });
    let severity = parsed_input
        .severity
        .map_or(quote! { None }, |e| quote! { Some(#e.into()) });
    let help = parsed_input
        .help
        .map_or(quote! { None }, |e| quote! { Some(#e.into()) });
    let url = parsed_input
        .url
        .map_or(quote! { None }, |e| quote! { Some(#e.into()) });

    let expanded = quote! {
        {
            extern crate mischief;
            use mischief::{MischiefError, Report};

            extern crate alloc;
            use alloc::string::String;
            use core::fmt::Write;

            let mut description = String::new();
            write!(description, #description_lit, #description_args).unwrap();

            Report::new(
                MischiefError::new(
                    description,
                    None,
                    #code,
                    #severity,
                    #help,
                    #url,
                )
            )
        }
    };

    expanded.into()
}
