//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_derive on crates.io](https://img.shields.io/crates/v/cdumay_error_derive)](https://crates.io/crates/cdumay_error_derive)
//! [![cdumay_error_derive on docs.rs](https://docs.rs/cdumay_error_derive/badge.svg)](https://docs.rs/cdumay_error_derive)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_derive)
//!
//! The `cdumay_error_derive` crate provides procedural macros to simplify the creation of custom error types in Rust. By leveraging these macros,
//! developers can efficiently define error structures that integrate seamlessly with the `cdumay_error` error management ecosystem.
//!
//! # Overview
//!
//! Error handling in Rust often involves creating complex structs to represent various error kinds and implementing traits to provide context and
//! conversions. The `cdumay_error_derive` crate automates this process by offering macros that generate the necessary boilerplate code, allowing for
//! more readable and maintainable error definitions.
//!
//! # Features
//!
//! * **Macros**: Automatically generate implementations for custom error types.
//! * **Integration with cdumay_error**: Designed to work cohesively with the `cdumay_error` crate, ensuring consistent error handling patterns.
//!
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parenthesized, parse_macro_input, Ident, LitInt, LitStr, Token, Type};

struct ErrorKindArgs {
    const_name: Ident,
    _eq: Token![=],
    _parens: syn::token::Paren,
    message: LitStr,
    _comma1: Token![,],
    code: LitInt,
    _comma2: Token![,],
    description: LitStr,
}

impl Parse for ErrorKindArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let const_name: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;

        let content;
        let _parens = parenthesized!(content in input);

        let message: LitStr = content.parse()?;
        let _comma1: Token![,] = content.parse()?;
        let code: LitInt = content.parse()?;
        let _comma2: Token![,] = content.parse()?;
        let description: LitStr = content.parse()?;

        Ok(ErrorKindArgs {
            const_name,
            _eq,
            _parens,
            message,
            _comma1,
            code,
            _comma2,
            description,
        })
    }
}

struct ErrorKindArgsList {
    items: Punctuated<ErrorKindArgs, Comma>,
}

impl Parse for ErrorKindArgsList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ErrorKindArgsList {
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

/// The `define_kinds` macro is a procedural macro that generates constants of type `cdumay_error::ErrorKind`. This macro simplifies the definition
/// of structured error kinds by allowing developers to declare them using a concise syntax. It takes a list of error definitions and expands
/// them into properly structured `cdumay_error::ErrorKind` constants.
#[proc_macro]
pub fn define_kinds(input: TokenStream) -> TokenStream {
    let args_list = parse_macro_input!(input as ErrorKindArgsList);

    let constants = args_list.items.iter().map(|args| {
        let const_name = &args.const_name;
        let message = &args.message;
        let code = &args.code;
        let description = &args.description;

        quote! {
            #[allow(non_upper_case_globals)]
            pub const #const_name: cdumay_error::ErrorKind = cdumay_error::ErrorKind(stringify!(#const_name), #message, #code, #description);
        }
    });

    TokenStream::from(quote! {
        #(#constants)*
    })
}

struct ErrorDefinition {
    name: Ident,
    kind: Type,
}

struct ErrorDefinitions {
    definitions: Vec<ErrorDefinition>,
}

impl Parse for ErrorDefinitions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut definitions = Vec::new();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let kind: Type = input.parse()?;
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            definitions.push(ErrorDefinition { name, kind });
        }

        Ok(ErrorDefinitions { definitions })
    }
}

/// The `define_errors` macro is a procedural macro that generates structured error types implementing `cdumay_error::AsError`. This macro simplifies
/// error handling by defining error structures with relevant metadata, serialization, and error conversion logic.
///
/// Each generated struct:
///
/// * Implements `cdumay_error::AsError` for interoperability with `cdumay_error::ErrorKind`.
/// * Provides methods for setting error messages and details.
/// * Supports conversion from `cdumay_error::Error`.
///
#[proc_macro]
pub fn define_errors(input: TokenStream) -> TokenStream {
    let definitions = parse_macro_input!(input as ErrorDefinitions);

    let generated_structs = definitions.definitions.iter().map(|definition| {
        let name = &definition.name;
        let kind = &definition.kind;

        quote! {
            #[derive(Debug, Clone)]
            pub struct #name {
                class: String,
                message: String,
                details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
            }

            impl #name {
                pub const kind: cdumay_error::ErrorKind = #kind;
                pub fn new() -> Self {
                    Self {
                        class: format!("{}::{}::{}", Self::kind.side(), Self::kind.name(), stringify!(#name)),
                        message: Self::kind.description().into(),
                        details: None,
                    }
                }
                pub fn set_message(mut self, message: String) -> Self {
                    self.message = message;
                    self
                }
                pub fn set_details(mut self, details: std::collections::BTreeMap<String, serde_value::Value>) -> Self {
                    self.details = Some(details);
                    self
                }
                pub fn convert(error: cdumay_error::Error) -> Self {
                    let mut err_clone = error.clone();
                    let mut details = error.details.unwrap_or_default();
                    err_clone.details = None;
                    details.insert("origin".to_string(), serde_value::to_value(err_clone).unwrap());
                    Self {
                        class: format!("{}::{}::{}", Self::kind.side(), Self::kind.name(), stringify!(#name)),
                        message: Self::kind.description().into(),
                        details: Some(details),
                    }
                }
            }
            impl cdumay_error::AsError for #name {
                fn kind()-> cdumay_error::ErrorKind {
                    Self::kind
                }
                fn class(&self) -> String {
                    self.class.clone()
                }
                fn message(&self) -> String {
                    self.message.clone()
                }
                fn details(&self) -> Option<std::collections::BTreeMap<String, serde_value::Value>> {
                    self.details.clone()
                }
            }

            impl std::error::Error for #name {}

            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "[{}] {} ({}): {}", Self::kind.message_id(), stringify!(#name), Self::kind.code(), self.message())
                }
            }
        }
    });

    TokenStream::from(quote! {
        #(#generated_structs)*
    })
}