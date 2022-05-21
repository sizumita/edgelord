use proc_macro::TokenStream;
use syn::spanned::Spanned;
use crate::utils::parse_i18n;

#[derive(Debug, darling::FromMeta)]
pub(crate) struct CommandGroupMeta {
    pub name: Option<String>,
    pub description: String,
    pub i18n_names: Option<syn::Path>,
    pub i18n_descriptions: Option<syn::Path>,
}

pub(crate) fn parse_command_group(
    args: CommandGroupMeta,
    mut func: syn::ItemFn,
) -> Result<TokenStream, darling::Error> {
    if func.sig.asyncness.is_some() {
        return Err(syn::Error::new(func.sig.span(), "command group function must not be async").into());
    }

    let command_group_name = args.name.unwrap_or_else(|| func.sig.ident.to_string());

    let description = args.description;
    let i18n_names = parse_i18n(args.i18n_names);
    let i18n_descriptions = parse_i18n(args.i18n_descriptions);
    let function_name = std::mem::replace(&mut func.sig.ident, syn::parse_quote! { inner });
    let visibility = &func.vis;

    Ok(TokenStream::from(quote::quote! {
        #visibility fn #function_name() -> ::edgecord::CommandGroup {
            #func

            ::edgecord::CommandGroup {
                name: #command_group_name.to_string(),
                description: #description.to_string(),
                i18n_names: #i18n_names,
                i18n_descriptions: #i18n_descriptions,
                commands: inner()
            }
        }
    }))
}
