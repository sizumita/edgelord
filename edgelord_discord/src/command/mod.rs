use std::collections::HashMap;

pub struct Command {
    pub name: String,
    pub i18n_names: Option<HashMap<&'static str, String>>,
    pub description: String,
    pub i18n_descriptions: Option<HashMap<&'static str, String>>,
}
