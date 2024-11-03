use std::collections::HashMap;

use fluent_bundle::FluentValue;
use fluent_templates::{LanguageIdentifier, Loader};
use serde::{Deserialize, Serialize};

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "../locales",
        fallback_language: "en"
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct I18n(pub LanguageIdentifier);

impl Default for I18n {
    fn default() -> Self {
        Self::from("en")
    }
}

impl I18n {
    pub fn text(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.0, text_id)
    }

    pub fn text_with_args<T: AsRef<str>>(&self, text_id: &str, args: &HashMap<T, FluentValue>) -> String {
        LOCALES.lookup_with_args(&self.0, text_id, args)
    }
}

impl From<&str> for I18n {
    fn from(value: &str) -> Self {
        Self(value.parse().expect("Could not parse Lang ID"))
    }
}

impl From<String> for I18n {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
