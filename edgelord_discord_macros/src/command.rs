use proc_macro::TokenStream;
use std::borrow::BorrowMut;
use darling::FromMeta;
use quote::quote;
use syn::FnArg;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::spanned::Spanned as _;
use crate::validate::validate_option;

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct CommandMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
}

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct OptionMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
    pub autocomplete: Option<syn::Path>
}

pub(crate) struct CommandOption {
    pub name: syn::Ident,
    pub t: syn::Type,
    pub meta: OptionMeta,
    pub span: proc_macro2::Span,
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

    Ok(TokenStream::from(quote::quote! {
        #visibility fn #function_name() -> ::edgelord::discord::Command {
            #func

            ::edgelord::discord::Command {
                name: #command_name.to_string(),
                description: #description.to_string(),
                i18n_names: #i18n_names,
                i18n_descriptions: #i18n_descriptions,
                options: vec! [#( #parsed_options, )*],
                action: #action,
            }
        }
    }))
}

fn parse_i18n(path: Option<syn::Path>) -> proc_macro2::TokenStream {
    match path {
        Some(x) => quote! {Some(#x())},
        None => quote! {None}
    }
}

pub(crate) fn parse_options(options: &mut Punctuated<FnArg, Comma>) -> Result<Vec<CommandOption>, darling::Error> {
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
            x => {
                return Err(syn::Error::new(x.span(), "name must be identifier").into())
            }
        };

        let attrs = pattern
            .attrs
            .drain(..)
            .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
            .collect::<Result<Vec<_>, _>>()?;

        let meta = <OptionMeta as darling::FromMeta>::from_list(&attrs)?;

        validate_option(&meta, pattern.span())?;

        parsed_options.push(CommandOption {
            name: name.clone(),
            t: (*pattern.ty).clone(),
            meta,
            span: option.span(),
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
                    ctx.get_option::<#t>(#name)
                }
            }
        ).collect::<Vec<_>>();

    quote::quote! {
        |ctx, _| Box::pin(async move {
            inner(ctx.clone(), #( #args, )*)
                .await
        })
    }
}

fn parse_option_meta(option: &CommandOption) -> proc_macro2::TokenStream {
    let i18n_names = parse_i18n(option.meta.i18n_names.clone());
    let i18n_descriptions = parse_i18n(option.meta.i18n_descriptions.clone());
    let name = option.meta.name.clone().unwrap_or(option.name.to_string());
    let description = option.meta.description.clone();

    quote::quote! {
        ::edgelord::discord::CommandOption {
            name: #name.to_string(),
            description: #description.to_string(),
            i18n_names: #i18n_names,
            i18n_descriptions: #i18n_descriptions,
        }
    }
}
