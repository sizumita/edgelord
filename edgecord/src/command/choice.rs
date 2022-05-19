use crate::command::I18nMap;
use serde::{Deserialize, Serialize};

pub type ChoiceValue = serde_json::Value;

/**
Command Choice Parameter.
**/
#[derive(Clone, Serialize, Deserialize)]
pub struct Choice {
    pub name: String,
    pub i18n_names: I18nMap,
    pub value: ChoiceValue,
}

pub trait ChoiceTrait {
    fn choices() -> Vec<Choice>;
}
