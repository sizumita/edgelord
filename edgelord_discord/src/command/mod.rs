mod context;

use std::collections::HashMap;
use futures::future::BoxFuture;

pub use context::*;

type I18nMap = Option<HashMap<&'static str, String>>;

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,

    pub options: Vec<CommandOption>,

    pub action: for<'a> fn(
        Context,
        name: &'a str,
    ) -> BoxFuture<'a, worker::Result<worker::Response>>,
}

#[derive(Clone)]
pub struct CommandOption {
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,
}

