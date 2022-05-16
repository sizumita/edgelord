use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct CommandMeta {
    pub name: Option<String>,
    pub i18n_names: Option<syn::Ident>,
    pub description: String,
    pub i18n_descriptions: Option<syn::Ident>,
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
    let i18n_names = match args.i18n_names {
        Some(x) => quote! {Some(#x())},
        None => quote! {None}
    };
    let i18n_descriptions = match args.i18n_descriptions {
        Some(x) => quote! {Some(#x())},
        None => quote! {None}
    };
    let function_name = func.sig.ident.clone();
    let visibility = func.vis;

    Ok(TokenStream::from(quote::quote! {
        #visibility fn #function_name() -> ::edgelord_discord::Command {
            ::edgelord_discord::Command {
                name: #command_name.to_string(),
                description: #description.to_string(),
                i18n_names: #i18n_names,
                i18n_descriptions: #i18n_descriptions,
            }
        }
    }))
}
