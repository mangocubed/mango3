use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::info::Info;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct InfoResp {
    pub git_commit_hash: String,
    pub git_commit_short_hash: String,
    pub reaction_emojis: Vec<String>,
    pub version: String,
}

#[cfg(feature = "ssr")]
impl From<Info> for InfoResp {
    fn from(info: Info) -> Self {
        Self {
            git_commit_hash: info.git_commit_hash.clone(),
            git_commit_short_hash: info.git_commit_short_hash,
            reaction_emojis: info.reaction_emojis,
            version: info.version,
        }
    }
}
