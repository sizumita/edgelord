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
mod command_group;
mod permission;
mod utils;
mod validate;

use crate::command::{parse_command, CommandMeta};
use crate::command_group::{parse_command_group, CommandGroupMeta};
#[allow(unused_imports)]
use darling::FromMeta as _;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/**
This macro transforms a function into edgecord slash command.

The function must return edgecord::InteractionResponse.

# Macro Arguments

- `name`: Set the command name. If you use command as group member, It is used for subcommand name.
- `description`: The description of slash command(or sub command). In the future you will be able to use doc as description.
- `i18n_names`: The function that returns I18nMap for localization command(or subcommand) name.
- `i18n_descriptions`: The function that returns I18nMap for localization command description.

# Function Parameter Attribute Arguments

`ChatInputContext` must be command function first parameter.
You can't add any attributes for context.

After second parameters, they are placed as slashcommand options.

The option type must be in `[String, i64, f64, bool, Id<ChannelMaker>, Id<RoleMaker>, Id<UserMaker>, Id<GenericMaker>, Id<AttachmentMaker> or Choiceable]`

You can add these arguments by using `option(...)` attribute.

- `name`: Set the option name. If you don't set, framework uses argument name.
- `description`: Set the option description. You have to add it for arguments.
- `i18n_names`: The function that returns I18nMap for localization option name.
- `i18n_descriptions`: The function that returns I18nMap for localization option description.
- `required`: This is flag.
TODO: add params

# Examples

```
fn echo_names() -> std::collections::HashMap<Locales, String> {
    std::collections::HashMap::from([(Locales::Ja, "おうむ返し".to_string())])
}

#[command(description = "this is description", i18n_names = "echo_names")]
async fn echo(
    ctx: ChatInputContext,
    #[choice(description = "echo text", required)]
    text: String) -> InteractionResponse {
    ctx.message(&*text)
}
```

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

/**
This macro transforms a function into edgecord slash command group.

The function must return Vec<edgecord::command::SubCommand>.

You can return SubCommand::Group and SubCommand::Command but you can't return SubCommand::Group if the parent group is in other group.

# Macro Arguments

- `name`: Set the group name. If you use command as group member, It is used for group.
- `description`: The description of group. In the future you will be able to use doc as description.
- `i18n_names`: The function that returns I18nMap for localization group name.
- `i18n_descriptions`: The function that returns I18nMap for localization group description.

```ignore
// This group has SubCommand::Group, so it have to be command.
#[group(description = "show animal emojis")]
fn animal_group() -> Vec<SubCommand> {
    vec![SubCommand::Command(dog_command()), SubCommand::Command(cat_command()), SubCommand::Group(rabbit_group())]
}
```

**/
#[proc_macro_attribute]
pub fn group(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Vec<syn::NestedMeta>);
    let args = match <CommandGroupMeta as darling::FromMeta>::from_list(&args) {
        Ok(x) => x,
        Err(e) => return e.write_errors().into(),
    };

    let function = syn::parse_macro_input!(func as syn::ItemFn);

    match parse_command_group(args, function) {
        Ok(stream) => stream,
        Err(e) => e.write_errors().into(),
    }
}

/**
This is a macro that makes enum to command option.

```ignore
#[derive(edgecord::Choiceable)]
enum StringChoices {
    #[choice(value = "dog")]
    Dog,
    #[choice(value = "cat")]
    Cat,
}

#[derive(edgecord::Choiceable)]
#[choice(value_type = "integer")] // string, integer and float is available
enum IntegerChoices {
    #[choice(value = 1)]
    Dog,
    #[choice(value = 2)]
    Cat,
}

```
**/
#[proc_macro_derive(Choiceable, attributes(choice))]
pub fn derive_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    choice::expand_derive_choice(input).unwrap()
}
