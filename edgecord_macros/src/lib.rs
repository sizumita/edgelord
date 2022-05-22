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

- `name`: The command name. If you use the command as group member, It is used for subcommand name.
- `description`: The description of the slash command(or sub command). In the future you will be able to use doc as description. Required for arguments.
- `i18n_names`: A function that returns HashMap<Locales, String> of localization command(or subcommand) name.
- `i18n_descriptions`: A function that returns HashMap<Locales, String> for localization command description.
- `default_permissions`: The permissions that a member has to have when he uses this command. If this command is used as a subcommand, this field is ignored.

# Function Parameter Attribute Arguments

`ChatInputContext` must be the first parameter of the command function.
You can't add any attributes to context.

From second parameters onwards, they are used as slashcommand options.

The option type must be in `[String, i64, f64, bool, Id<ChannelMaker>, Id<RoleMaker>, Id<UserMaker>, Id<GenericMaker>, Id<AttachmentMaker> or Choiceable]`

You can add these arguments by using `option(...)` attribute.

- `name`: The command name.
- `description`: The description of the option. Required for arguments.
- `i18n_names`: A function that returns HashMap<Locales, String> of localization option name.
- `i18n_descriptions`: A function that returns HashMap<Locales, String> of localization option description.
- `required`: This is a flag.
TODO: add params

# Examples

```ignore
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
This macro transforms a function into edgecord slash command group or slash command that has subcommand.

The function must return Vec<edgecord::command::SubCommand>.

You can return either SubCommand::Group or SubCommand::Command, but you can't return SubCommand::Group if the parent group is in other group.

# Macro Arguments

- `name`: The group name. If you use the command as group member, It is used for group name.
- `description`: The description of the group. In the future you will be able to use doc as description. Required for arguments.
- `i18n_names`: A function that returns HashMap<Locales, String> of localization group name.
- `i18n_descriptions`: A function that returns HashMap<Locales, String> of localization group description.
- `default_permissions`: The permissions that a member has to have when he uses this command. If this command is used as a subcommand group, this field is ignored.

```ignore
// This group has a SubCommand::Group, so it has to be command.
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
This is a macro that turns an enum into a command option.

You can use the `choice(...)` macro in Choiceable.

# Macro Arguments

- `value_type`: Set value type for the enum. You can use `"string"`, `"integer"` and `"float"`.

# Function Parameter Attribute Arguments

- `name`: The choice name.
- `value`: The choice value. Required for arguments.
- `i18n_names`: A function that returns HashMap<Locales, String> of localization choice name.


```ignore
#[derive(edgecord::Choiceable)]
enum StringChoices {
    #[choice(value = "dog")]
    Dog,
    #[choice(value = "cat")]
    Cat,
}

#[derive(edgecord::Choiceable)]
#[choice(value_type = "integer")] // string, integer and float are available
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
