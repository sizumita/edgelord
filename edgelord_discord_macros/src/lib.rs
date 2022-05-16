#[macro_use]
mod command;
mod validate;

use proc_macro::TokenStream;
use darling::FromMeta;
use proc_macro2::Span;
use crate::command::{CommandMeta, parse_command};

/**

# Command Metadata Macro

## Command Arguments

- `name`: command default name
- `description`: command default description
- `i18n_names`: command i18n names HashMap generate function name. () -> HashMap<&'static str, String>. See https://discord.com/developers/docs/reference#locales
- `i18n_descriptions`: command i18n descriptions HashMap generate function name. () -> HashMap<&'static str, String>.


**/


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

