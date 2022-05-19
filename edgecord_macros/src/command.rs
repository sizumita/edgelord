use crate::utils::parse_i18n;
use crate::validate::validate_option;
#[allow(unused_imports)]
use darling::FromMeta as _;
use proc_macro::TokenStream;
use darling::util::Flag;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Comma;
use syn::FnArg;
use crate::permission::PermissionFlagBits;

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct CommandMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
    pub default_permissions: Option<PermissionFlagBits>,
}

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct OptionMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
    pub autocomplete: Option<syn::Path>,
    pub required: Flag,
}

#[derive(Default, Debug, darling::FromMeta)]
pub(crate) struct OptionMetaWrapped {
    pub option: OptionMeta,
}

pub(crate) struct CommandOption {
    pub name: syn::Ident,
    pub t: syn::Type,
    pub meta: OptionMeta,
}

pub(crate) fn parse_command(
    args: CommandMeta,
    mut func: syn::ItemFn,
) -> Result<TokenStream, darling::Error> {
    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new(func.sig.span(), "command function must be async").into());
    }
    let command_name = args.name.unwrap_or(func.sig.ident.to_string());
    let description = args.description;
    let i18n_names = parse_i18n(args.i18n_names);
    let i18n_descriptions = parse_i18n(args.i18n_descriptions);
    let function_name = std::mem::replace(&mut func.sig.ident, syn::parse_quote! { inner });
    let visibility = &func.vis;

    let options = parse_options(&mut func.sig.inputs)?;
    let parsed_options = options.iter().map(parse_option_meta).collect::<Vec<_>>();
    let action = parse_action(options);
    let default_permissions = {
        match args.default_permissions {
            None => quote::quote! {None},
            Some(x) => {
                let y = x.bits().bits();
                quote::quote! {Some(#y)}
            }
        }
    };

    Ok(TokenStream::from(quote::quote! {
        #visibility fn #function_name<'a>() -> ::edgecord::Command<'a> {
            use ::edgecord::option::FromCommandOptionValue;
            #func

            ::edgecord::Command {
                command_type: ::edgecord::model::application::command::CommandType::ChatInput,
                name: #command_name.to_string(),
                description: #description.to_string(),
                i18n_names: #i18n_names,
                i18n_descriptions: #i18n_descriptions,
                default_permissions: #default_permissions,
                options: vec! [#( #parsed_options, )*],
                action: #action,
            }
        }
    }))
}

pub(crate) fn parse_options(
    options: &mut Punctuated<FnArg, Comma>,
) -> Result<Vec<CommandOption>, darling::Error> {
    let mut parsed_options = Vec::new();

    for option in options.iter_mut().skip(1) {
        let pattern = match option {
            syn::FnArg::Typed(x) => &mut *x,
            syn::FnArg::Receiver(r) => {
                return Err(syn::Error::new(r.span(), "invalid argument").into());
            }
        };

        let name = match &*pattern.pat {
            syn::Pat::Ident(pat_ident) => &pat_ident.ident,
            x => return Err(syn::Error::new(x.span(), "name must be identifier").into()),
        };

        let attrs = pattern
            .attrs
            .drain(..)
            .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
            .collect::<Result<Vec<_>, _>>()?;

        let meta = <OptionMetaWrapped as darling::FromMeta>::from_list(&attrs)?.option;

        validate_option(&meta, pattern.span())?;

        parsed_options.push(CommandOption {
            name: name.clone(),
            t: (*pattern.ty).clone(),
            meta,
        })
    }
    Ok(parsed_options)
}

fn parse_action(options: Vec<CommandOption>) -> proc_macro2::TokenStream {
    let args =
        options.iter().map(
            |option| {
                let name = match &option.meta.name {
                    None => option.name.to_string(),
                    Some(x) => x.clone(),
                };
                let t = option.t.clone();
                quote::quote! {
                    ::edgecord::ChatInputCommandContext::get_option::<#t>(interaction.clone(), #name)
                }
            }
        ).collect::<Vec<_>>();
    quote::quote! {
        ::std::rc::Rc::new(move |ctx, interaction| Box::pin(inner(ctx, #( #args, )*)))
    }
}

fn parse_option_meta(option: &CommandOption) -> proc_macro2::TokenStream {
    let i18n_names = parse_i18n(option.meta.i18n_names.clone());
    let i18n_descriptions = parse_i18n(option.meta.i18n_descriptions.clone());
    let name = option.meta.name.clone().unwrap_or(option.name.to_string());
    let description = option.meta.description.clone();
    let t = &option.t;
    let required = &option.meta.required.is_present();

    quote::quote! {
        ::edgecord::CommandOption {
            option_type: #t::get_option_type(),
            name: #name.to_string(),
            description: #description.to_string(),
            i18n_names: #i18n_names,
            i18n_descriptions: #i18n_descriptions,
            choices: {
                if #t::has_choices() {
                    <#t as ::edgecord::ChoiceTrait>::choices()
                } else {
                    vec![]
                }
            },
            required: #required,
        }
    }
}
