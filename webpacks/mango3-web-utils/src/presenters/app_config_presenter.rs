use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct AppConfigPresenter {
    pub locale: LanguageIdentifier,
}
