use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};

/// A helper struct to parse the custom syntax of the `mischief_error!` macro.
struct MischiefErrorInput {
    description_lit: LitStr,
    description_args: Punctuated<Expr, Token![,]>,
    code: Option<Expr>,
    severity: Option<Expr>,
    help: Option<Expr>,
    url: Option<Expr>,
}

/// Implement the `Parse` trait from `syn` to define our parsing logic.
impl Parse for MischiefErrorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. The first argument must be the format string literal.
        let description_lit = input.parse()?;
        let mut description_args = Punctuated::new();

        // Optional fields
        let mut code = None;
        let mut severity = None;
        let mut help = None;
        let mut url = None;

        // Flag to enforce that positional format arguments come before named fields.
        let mut seen_named_arg = false;

        // 2. Loop through the rest of the comma-separated arguments.
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            } // Allow a trailing comma

            // 3. Check if the next part is a named field like `code = "..."`.
            if input.peek(Ident) && input.peek2(Token![=]) {
                seen_named_arg = true;
                let key: Ident = input.parse()?;
                let key_str = key.to_string();
                input.parse::<Token![=]>()?; // Consume the `=`
                let value: Expr = input.parse()?;

                // Match the identifier to the correct struct field.
                match key_str.as_str() {
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
                // 4. If not a named field, it's a positional format argument.
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

#[proc_macro]
pub fn mischief(input: TokenStream) -> TokenStream {
    // Parse the token stream into our custom `MischiefErrorInput` struct.
    let parsed_input = match syn::parse::<MischiefErrorInput>(input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let description_lit = parsed_input.description_lit;
    let description_args = parsed_input.description_args;

    // Use `quote` to generate the code for optional fields.
    // If an expression `e` was provided, this generates `Some(e.into())`.
    // Otherwise, it generates `None`.
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

    // Generate the final `MischiefError` struct instantiation.
    let expanded = quote! {
        {
            extern crate mischief;
            use mischief::{MischiefError, Report};

            extern crate alloc;
            use alloc::string::String;
            use core::fmt::Write;
            let mut description = String::new();
            write!(description, #description_lit, #description_args).unwrap();

            // Note: Your `MischiefError` struct and `Severity` enum must be in scope where this macro is called.
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
