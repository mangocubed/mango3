use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Deserialize, Serialize)]
pub struct AppConfigPresenter {
    pub language: LanguageIdentifier,
}
