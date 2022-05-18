use crate::utils::parse_i18n;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Data, Expr, Lit};

#[derive(Clone, Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct ChoiceMeta {
    pub rename: Option<String>,
    pub i18n_names: Option<syn::Path>,
    pub value: Option<String>,
}

#[derive(Clone, Default, Debug, darling::FromMeta)]
#[darling(default)]
pub(crate) struct ChoiceArgs {
    pub choice: ChoiceMeta,
}

#[derive(Clone, Debug, darling::FromMeta, PartialEq)]
#[darling(rename_all = "snake_case")]
pub(crate) enum ChoiceType {
    String,
    Integer,
    Float,
}

impl ChoiceType {
    pub fn to_ident(&self) -> syn::Ident {
        match self {
            ChoiceType::String => syn::Ident::from_string("String").unwrap(),
            ChoiceType::Integer => syn::Ident::from_string("i64").unwrap(),
            ChoiceType::Float => syn::Ident::from_string("f64").unwrap(),
        }
    }
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct ChoicesMeta {
    value_type: Option<ChoiceType>,
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct ChoicesArgs {
    #[darling(default)]
    pub choice: Option<ChoicesMeta>,
}

#[derive(Clone)]
pub(crate) struct Choice {
    pub meta: ChoiceMeta,
    pub ident: Ident,
    pub value: String,
}

pub fn expand_derive_choice(mut input: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let enum_ = match input.clone().data {
        Data::Enum(e) => e,
        _ => return Err(syn::Error::new(input.span(), "choice must be enum")),
    };
    let enum_attrs = input
        .attrs
        .drain(..)
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;
    let enum_meta = ChoicesArgs::from_list(&enum_attrs)?.choice;

    let mut choices = Vec::<Choice>::new();

    for mut variant in enum_.variants {
        if !matches!(&variant.fields, syn::Fields::Unit) {
            return Err(syn::Error::new(
                variant.fields.span(),
                "choice params can't have fields",
            ));
        }

        let attrs = variant
            .attrs
            .drain(..)
            .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
            .collect::<Result<Vec<_>, _>>()?;

        let value = {
            if let Some((_, expr)) = variant.discriminant.clone() {
                if let Expr::Lit(lit) = expr.clone() {
                    match lit.lit {
                        Lit::Str(v) => v.value(),
                        Lit::Int(v) => v.to_string(),
                        Lit::Float(v) => v.to_string(),
                        _ => {
                            return Err(syn::Error::new(
                                expr.span(),
                                "choice value must be string, int or float",
                            ))
                        }
                    }
                } else {
                    return Err(syn::Error::new(
                        expr.span(),
                        "choice value must be string, int or float",
                    ));
                }
            } else {
                variant.ident.to_string()
            }
        };

        let meta = <ChoiceArgs as darling::FromMeta>::from_list(&attrs)?.choice;
        choices.push(Choice {
            meta: meta.clone(),
            ident: variant.ident,
            value: meta.value.unwrap_or(value),
        })
    }

    let enum_name = &input.ident;
    let ty = enum_meta
        .unwrap_or(ChoicesMeta { value_type: None })
        .value_type
        .clone()
        .unwrap_or(ChoiceType::String);
    let parsed = choices
        .iter()
        .map(|x| parse_choice(&ty, x))
        .collect::<Vec<_>>();
    let matchs = choices
        .clone()
        .iter()
        .map(|x| parse_choice_matches(&enum_name, &ty, &x))
        .collect::<Vec<_>>();
    let t = ty.to_ident();
    let inject = {
        match ty {
            ChoiceType::String => quote::quote! {
                let value = {
                    if let CommandOptionValue::String(value) = value {value} else {return Err(::edgelord::discord::Error::WrongOptionType)}
                };
                let value = &*value;
            },
            ChoiceType::Integer => quote::quote! {
                let value = {
                    if let CommandOptionValue::Integer(value) = value {value} else {return Err(::edgelord::discord::Error::WrongOptionType)}
                };
            },
            ChoiceType::Float => quote::quote! {
                let value = {
                    if let CommandOptionValue::Number(value) = value {value} else {return Err(::edgelord::discord::Error::WrongOptionType)}
                };
            },
        }
    };

    Ok(TokenStream::from(quote::quote! {
        impl ::edgelord::discord::ChoiceTrait for #enum_name {
            fn choices() -> Vec<::edgelord::discord::Choice> {
                vec![
                    #( #parsed, )*
                ]
            }
        }

        use ::edgelord::discord::model::application::interaction::application_command::CommandOptionValue;

        impl ::edgelord::discord::option::FromCommandOptionValue for #enum_name {
            fn from_option(value: CommandOptionValue) -> ::std::result::Result<Self, ::edgelord::discord::Error> where Self: Sized {
                #inject
                match value {
                    #( #matchs, )*
                    _ => Err(::edgelord::discord::Error::WrongOptionType),
                }
            }
        }
    }))
}

fn parse_choice(ty: &ChoiceType, choice: &Choice) -> proc_macro2::TokenStream {
    let name = &choice.ident;
    let renamed = parse_choice_value(ty, choice);
    let str_name = choice.meta.rename.clone().unwrap_or(name.to_string());
    let i18n_names = parse_i18n(choice.meta.i18n_names.clone());
    quote::quote! {
        ::edgelord::discord::Choice {
            name: #str_name.to_string(),
            i18n_names: #i18n_names,
            value: #renamed,
        }
    }
}

fn parse_choice_matches(
    enum_name: &Ident,
    ty: &ChoiceType,
    choice: &Choice,
) -> proc_macro2::TokenStream {
    let value = parse_choice_raw_value(ty, choice);
    let name = &choice.ident;
    quote::quote! {
        #value => Ok(#enum_name::#name)
    }
}

fn parse_choice_value(ty: &ChoiceType, choice: &Choice) -> proc_macro2::TokenStream {
    let v = choice.value.clone();
    match ty {
        ChoiceType::String => {
            quote::quote! { ::edgelord::discord::ChoiceValue::from(#v.to_string()) }
        }
        ChoiceType::Float => {
            let v = v.parse::<f32>().unwrap();
            quote::quote! { ::edgelord::discord::ChoiceValue::from(#v) }
        }
        ChoiceType::Integer => {
            let v = v.parse::<i32>().unwrap();
            quote::quote! { ::edgelord::discord::ChoiceValue::from(#v) }
        }
    }
}

fn parse_choice_raw_value(ty: &ChoiceType, choice: &Choice) -> proc_macro2::TokenStream {
    let v = choice.value.clone();
    match ty {
        ChoiceType::String => quote::quote! { #v },
        ChoiceType::Float => {
            let v = v.parse::<f64>().unwrap();
            quote::quote! { #v }
        }
        ChoiceType::Integer => {
            let v = v.parse::<i64>().unwrap();
            quote::quote! { #v }
        }
    }
}
