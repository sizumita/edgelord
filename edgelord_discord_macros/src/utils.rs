pub fn parse_i18n(path: Option<syn::Path>) -> proc_macro2::TokenStream {
    match path {
        Some(x) => quote::quote! {Some(#x())},
        None => quote::quote! {None},
    }
}
