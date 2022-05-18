use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::command::I18nMap;

/**
Command Choice Parameter.
**/
#[derive(Clone, Serialize, Deserialize)]
pub struct Choice {
    pub name: String,
    pub i18n_names: I18nMap,
    pub value: serde_json::Value,
}

pub trait ChoiceTrait {
    fn choices() -> Vec<Choice>;
    fn from_value(value: serde_json::Value) -> Result<Self, crate::Error> where Self: Sized;
}
