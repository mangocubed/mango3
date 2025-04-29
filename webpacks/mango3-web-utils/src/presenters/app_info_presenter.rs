use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct AppInfoPresenter {
    pub copyright: String,
    pub description: String,
    pub domain: String,
    pub enable_register: bool,
    pub git_commit_hash: String,
    pub git_commit_short_hash: String,
    pub reaction_emojis: Vec<String>,
    pub support_email_address: String,
    pub title: String,
    pub version: String,
}
