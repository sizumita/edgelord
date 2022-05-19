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
//! ```ignore
//! use edgecord::{ChatInputCommandContext, InteractionResponse, command};
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
//! ```ignore
//! use edgecord::{ChatInputCommandContext, InteractionResponse, command, Choiceable};
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
//! # Command DefaultPermissions
//!
//! You can set default permissions for command with `default_permissions(permission, ...)`.
//!
//! ```ignore
//!
//! #[edgecord::command(name = "remove_role", description = "remove your role", default_permissions(manage_roles))]
//! async fn remove_role(ctx: edgecord::Context) -> edgecord::InteractionResponse {
//!     ctx.message("removed your roles!")
//! }
//! ```
//!
mod choice;
mod command;
mod utils;
mod validate;
mod permission;

use crate::command::{parse_command, CommandMeta};
#[allow(unused_imports)]
use darling::FromMeta as _;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

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

#[proc_macro_derive(Choiceable, attributes(choice))]
pub fn derive_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    choice::expand_derive_choice(input).unwrap()
}
