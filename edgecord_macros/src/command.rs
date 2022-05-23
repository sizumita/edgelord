use crate::channel_type::ChannelTypes;
use crate::permission::PermissionFlagBits;
use crate::utils::parse_i18n;
use crate::validate::validate_option;
#[allow(unused_imports)]
use darling::FromMeta as _;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Comma;
use syn::FnArg;

#[derive(Debug, darling::FromMeta)]
pub(crate) struct CommandMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
    pub default_permissions: Option<PermissionFlagBits>,
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct OptionMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
    #[allow(dead_code)]
    pub autocomplete: Option<syn::Path>,
    pub min_value: Option<syn::Lit>,
    pub max_value: Option<syn::Lit>,
    pub channel_types: Option<ChannelTypes>,
}

#[derive(Debug, darling::FromMeta)]
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
    let command_name = args.name.unwrap_or_else(|| func.sig.ident.to_string());
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
        #visibility fn #function_name() -> ::edgecord::application_command::Command {
            use ::edgecord::application_command::option::FromCommandOptionValue;
            #func

            ::edgecord::application_command::Command {
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
    let args = options
        .iter()
        .map(|option| {
            let name = match &option.meta.name {
                None => option.name.to_string(),
                Some(x) => x.clone(),
            };
            let (required, t) = parse_option_type(&option.t);
            if required {
                quote::quote! {
                    #t::from_option(options.iter().find(|x| x.name == #name).cloned().unwrap().value).unwrap()
                }
            } else {
                quote::quote! {
                    options.iter().find(|x| x.name == #name).cloned().map(|value| #t::from_option(value.value).unwrap())
                }
            }
        })
        .collect::<Vec<_>>();
    quote::quote! {
        ::std::rc::Rc::new(move |ctx, interaction, options| Box::pin(inner(ctx, #( #args, )*)))
    }
}

fn parse_option_meta(option: &CommandOption) -> proc_macro2::TokenStream {
    let i18n_names = parse_i18n(option.meta.i18n_names.clone());
    let i18n_descriptions = parse_i18n(option.meta.i18n_descriptions.clone());
    let name = option
        .meta
        .name
        .clone()
        .unwrap_or_else(|| option.name.to_string());
    let description = option.meta.description.clone();
    let (required, ty) = parse_option_type(&option.t);
    let min_value = parse_range_value(&option.meta.min_value);
    let max_value = parse_range_value(&option.meta.max_value);
    let channel_types = {
        if let Some(x) = option.meta.channel_types.clone().map(|x| x.to_vec_token()) {
            quote::quote!(Some(#x))
        } else {
            quote::quote!(None)
        }
    };

    quote::quote! {
        ::edgecord::application_command::CommandOption {
            option_type: #ty::get_option_type(),
            name: #name.to_string(),
            description: #description.to_string(),
            i18n_names: #i18n_names,
            i18n_descriptions: #i18n_descriptions,
            choices: #ty::choices(),
            required: #required,
            min_value: #min_value,
            max_value: #max_value,
            channel_types: #channel_types
        }
    }
}

fn parse_range_value(value: &Option<syn::Lit>) -> proc_macro2::TokenStream {
    match value {
        None => quote::quote! {None},
        Some(x) => {
            let v = x.to_token_stream();
            quote::quote! {Some(#v.into())}
        }
    }
}

fn parse_option_type(ty: &syn::Type) -> (bool, syn::Type) {
    fn path_is_option(path: &syn::Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.first().unwrap().ident == "Option"
    }

    match ty {
        syn::Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            let type_params = typepath.path.segments.first().unwrap().clone().arguments;
            let generic_arg = match type_params {
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap().clone(),
                _ => return (true, ty.clone()),
            };
            match generic_arg {
                syn::GenericArgument::Type(ty) => (false, ty),
                _ => (true, ty.clone()),
            }
        }
        _ => (true, ty.clone()),
    }
}
