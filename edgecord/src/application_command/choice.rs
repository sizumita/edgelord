use crate::application_command::I18nMap;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum ChoiceValue {
    String(String),
    Integer(i64),
    Float(f64),
}

impl Serialize for ChoiceValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s.as_str()),
            Self::Integer(i) => serializer.serialize_i64(*i),
            Self::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

/**
Command Choice Parameter.
**/
#[derive(Debug, Clone, Serialize)]
pub struct Choice {
    pub name: String,
    #[serde(rename = "name_localizations")]
    pub i18n_names: I18nMap,
    pub value: ChoiceValue,
}
