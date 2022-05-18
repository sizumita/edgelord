//! # Command Metadata Macro
//!
//! You can define application command using this macro.
//!
//! ## Command Arguments
//!
//! - `name`: command default name
//! - `description`: command default description
//! - `i18n_names`: command i18n names HashMap generate function name. () -> HashMap<&'static str, String>. See https://discord.com/developers/docs/reference#locales
//! - `i18n_descriptions`: command i18n descriptions HashMap generate function name. () -> HashMap<&'static str, String>.
//!
//! ## Command Option Arguments
//!
//! - `name`: command option default name
//! - `description`: command option default description
//! - `i18n_names`: command option i18n names HashMap generate function name. () -> HashMap<&'static str, String>. See https://discord.com/developers/docs/reference#locales
//! - `i18n_descriptions`: command option i18n descriptions HashMap generate function name. () -> HashMap<&'static str, String>.
//!
//! # Examples
//!
//! ```rust
//! use edgelord::discord::{ChatInputCommandContext, InteractionResponse, command};
//!
//! #[command(name = "help", description = "show help message")]
//! async fn help_message(
//!     ctx: ChatInputCommandContext,
//!     #[description = "command group"]
//!     group: Option<String>,
//!
//! ) -> InteractionResponse {
//!     ctx.message("this is help message!")
//! }
//! ```
//!
//! # Command Option Choices
//!
//! You can use [`Choiceable`] derive for enum and use command option type.
//!
//! # Example
//!
//! ```
//! use edgelord::discord::{ChatInputCommandContext, InteractionResponse, command, Choiceable};
//!
//! #[derive(Choiceable)]
//! enum StringChoices {
//!     Dog,
//!     #[choice(rename = "🐱")]
//!     Cat,
//!     #[choice(i18n_names = "some_i18n_func")]
//!     Bull,
//! }
//!
//! #[derive(Choiceable)]
//! #[repr(u8)]
//! #[choice(value_type = "integer")] // "float" to f64
//! enum IntChoices {
//!     Egg = 12,
//! }
//!
//! ```
//!
//!
mod command;
mod validate;
mod choice;
mod utils;

use proc_macro::TokenStream;
use darling::FromMeta;
use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};
use crate::command::{CommandMeta, parse_command};


#[proc_macro_attribute]
pub fn command(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Vec<syn::NestedMeta>);
    let args = match <CommandMeta as darling::FromMeta>::from_list(&args) {
        Ok(x) => x,
        Err(e) => return e.write_errors().into(),
    };

    let function = syn::parse_macro_input!(func as syn::ItemFn);

    match parse_command(args, function) {
        Ok(stream) => stream,
        Err(e) => e.write_errors().into(),
    }
}


#[proc_macro_derive(Choiceable, attributes(choice, value_type))]
pub fn derive_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    choice::expand_derive_choice(input).unwrap()
}
