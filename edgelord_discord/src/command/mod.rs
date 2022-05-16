mod context;

use std::collections::HashMap;
use futures::future::BoxFuture;

pub use context::*;

pub struct Command {
    pub name: String,
    pub i18n_names: Option<HashMap<&'static str, String>>,
    pub description: String,
    pub i18n_descriptions: Option<HashMap<&'static str, String>>,

    pub action: for<'a> fn(
        Context,
        name: &'a str,
    ) -> BoxFuture<'a, worker::Result<worker::Response>>,
}
