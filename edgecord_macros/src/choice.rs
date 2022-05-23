use crate::utils::parse_i18n;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::Data;

#[derive(Debug, darling::FromMeta)]
pub(crate) struct ChoicesMeta {
    #[darling(rename = "type")]
    value_type: Option<ChoiceType>,
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct ChoicesArgs {
    #[darling(default)]
    pub choice: Option<ChoicesMeta>,
}

#[derive(Clone, Debug, darling::FromMeta, PartialEq)]
#[darling(rename_all = "snake_case")]
pub(crate) enum ChoiceType {
    String,
    Integer,
    Float,
}

impl ChoiceType {
    pub fn to_option_type(&self) -> proc_macro2::TokenStream {
        match self {
            ChoiceType::String => quote::quote! {String},
            ChoiceType::Integer => quote::quote! {Integer},
            ChoiceType::Float => quote::quote! {Number},
        }
    }
}

#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct ChoiceArgs<T> {
    pub choice: Option<T>,
}

#[derive(Default, Clone, Debug, darling::FromMeta)]
pub(crate) struct StringChoiceMeta {
    pub rename: Option<String>,
    pub i18n_names: Option<syn::Path>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct IntegerChoiceMeta {
    pub rename: Option<String>,
    pub i18n_names: Option<syn::Path>,
    pub value: i64,
}

#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct FloatChoiceMeta {
    pub rename: Option<String>,
    pub i18n_names: Option<syn::Path>,
    pub value: f64,
}

#[derive(Clone)]
pub(crate) enum Choice {
    String {
        meta: StringChoiceMeta,
        ident: Ident,
    },
    Integer {
        meta: IntegerChoiceMeta,
        ident: Ident,
    },
    Float {
        meta: FloatChoiceMeta,
        ident: Ident,
    },
}

impl Choice {
    pub fn get_name(&self) -> String {
        match self {
            Choice::String { meta, ident } => meta.clone().rename.unwrap_or_else(|| ident.to_string()),
            Choice::Integer { meta, ident } => meta.clone().rename.unwrap_or_else(|| ident.to_string()),
            Choice::Float { meta, ident } => meta.clone().rename.unwrap_or_else(|| ident.to_string()),
        }
    }
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
    let enum_meta = ChoicesArgs::from_list(&enum_attrs)?
        .choice
        .unwrap_or(ChoicesMeta {
            value_type: Some(ChoiceType::String),
        });
    let choice_type = enum_meta.value_type.unwrap();

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

        choices.push(match choice_type {
            ChoiceType::String => {
                let meta = <ChoiceArgs<StringChoiceMeta> as darling::FromMeta>::from_list(&attrs)?
                    .choice
                    .unwrap_or_default();
                Choice::String {
                    meta,
                    ident: variant.ident,
                }
            }
            ChoiceType::Integer => {
                let meta = <ChoiceArgs<IntegerChoiceMeta> as darling::FromMeta>::from_list(&attrs)?
                    .choice
                    .expect("integer choice must have value field");
                Choice::Integer {
                    meta,
                    ident: variant.ident,
                }
            }
            ChoiceType::Float => {
                let meta = <ChoiceArgs<FloatChoiceMeta> as darling::FromMeta>::from_list(&attrs)?
                    .choice
                    .expect("float choice must have value field");
                Choice::Float {
                    meta,
                    ident: variant.ident,
                }
            }
        })
    }

    let enum_name = &input.ident;
    let parsed = choices.iter().map(parse_choice).collect::<Vec<_>>();
    let matchs = choices
        .clone()
        .iter()
        .map(|x| parse_choice_matches(enum_name, x))
        .collect::<Vec<_>>();

    let inject = {
        match choice_type {
            ChoiceType::String => quote::quote! {
                let value = {
                    if let CommandOptionValue::String(value) = value {value} else {return Err(::edgecord::Error::WrongOptionType)}
                };
                let value = &*value;
            },
            ChoiceType::Integer => quote::quote! {
                let value = {
                    if let CommandOptionValue::Integer(value) = value {value} else {return Err(::edgecord::Error::WrongOptionType)}
                };
            },
            ChoiceType::Float => quote::quote! {
                let value = {
                    if let CommandOptionValue::Number(value) = value {value} else {return Err(::edgecord::Error::WrongOptionType)}
                };
            },
        }
    };
    let option_type = choice_type.to_option_type();

    Ok(TokenStream::from(quote::quote! {
        impl ::edgecord::application_command::ChoiceTrait for #enum_name {
            fn choices() -> Vec<::edgecord::application_command::Choice> {
                vec![
                    #( #parsed, )*
                ]
            }
        }

        use ::edgecord::model::application::interaction::application_command::{CommandOptionValue};
        use ::edgecord::model::application::command::CommandOptionType;

        impl ::edgecord::application_command::option::FromCommandOptionValue for #enum_name {
            fn from_option(value: CommandOptionValue) -> ::std::result::Result<Self, ::edgecord::Error> where Self: Sized {
                #inject
                match value {
                    #( #matchs, )*
                    _ => Err(::edgecord::Error::WrongOptionType),
                }
            }

            fn get_option_type() -> CommandOptionType {
                CommandOptionType::#option_type
            }

            fn has_choices() -> bool {
                true
            }

        }
    }))
}

fn parse_choice(choice: &Choice) -> proc_macro2::TokenStream {
    let name = choice.get_name();
    match choice {
        Choice::String { meta, ident } => {
            let value = meta.clone().value.unwrap_or_else(|| ident.to_string());
            let i18n_names = parse_i18n(meta.i18n_names.clone());
            quote::quote! {
                ::edgecord::application_command::choice::Choice {
                    name: #name.to_string(),
                    i18n_names: #i18n_names,
                    value: ::edgecord::application_command::choice::ChoiceValue::String(#value.to_string()),
                }
            }
        }
        Choice::Integer { meta, .. } => {
            let value = meta.value;
            let i18n_names = parse_i18n(meta.i18n_names.clone());
            quote::quote! {
                ::edgecord::application_command::choice::Choice {
                    name: #name.to_string(),
                    i18n_names: #i18n_names,
                    value: ::edgecord::application_command::choice::ChoiceValue::Integer(#value),
                }
            }
        }
        Choice::Float { meta, .. } => {
            let value = meta.value;
            let i18n_names = parse_i18n(meta.i18n_names.clone());
            quote::quote! {
                ::edgecord::application_command::choice::Choice {
                    name: #name.to_string(),
                    i18n_names: #i18n_names,
                    value: ::edgecord::application_command::choice::ChoiceValue::Float(#value),
                }
            }
        }
    }
}

fn parse_choice_matches(enum_name: &Ident, choice: &Choice) -> proc_macro2::TokenStream {
    match choice {
        Choice::String { meta, ident } => {
            let value = meta.clone().value.unwrap_or_else(|| ident.to_string());
            quote::quote! {
                #value => Ok(#enum_name::#ident)
            }
        }
        Choice::Integer { meta, ident } => {
            let value = meta.value;
            quote::quote! {
                #value => Ok(#enum_name::#ident)
            }
        }
        Choice::Float { meta, ident } => {
            let value = meta.value;
            quote::quote! {
                #value => Ok(#enum_name::#ident)
            }
        }
    }
}
