use serde::{Deserialize, Serialize};

/**
Discord i18n Locales.

https://discord.com/developers/docs/reference#locales
**/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Locales {
    Da,
    De,
    #[serde(rename = "en-GB")]
    EnGB,
    #[serde(rename = "en-US")]
    EnUS,
    #[serde(rename = "es-ES")]
    EsES,
    Fr,
    Hr,
    It,
    Lt,
    Hu,
    Nl,
    No,
    Pl,
    #[serde(rename = "pt-BR")]
    PtBR,
    Ro,
    Fi,
    #[serde(rename = "sv-SE")]
    SvSE,
    Vi,
    Tr,
    Cs,
    El,
    Bg,
    Ru,
    Uk,
    Hi,
    Th,
    #[serde(rename = "zh-CN")]
    ZhCN,
    Ja,
    #[serde(rename = "zh-TW")]
    ZhTW,
    Ko,
}

#[cfg(test)]
mod tests {
    use super::Locales;

    #[test]
    fn test_locale_1() {
        assert_eq!(serde_json::to_string(&Locales::EnGB).unwrap(), "\"en-GB\"")
    }

    #[test]
    fn test_locale_2() {
        assert_eq!(serde_json::to_string(&Locales::Ja).unwrap(), "\"ja\"")
    }
}

impl From<Locales> for String {
    fn from(locale: Locales) -> Self {
        serde_json::to_string(&locale).unwrap()
    }
}
