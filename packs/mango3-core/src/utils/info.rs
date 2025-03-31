use std::process::Command;
use std::sync::LazyLock;

use crate::constants::REACTION_EMOJIS;

pub static INFO: LazyLock<Info> = LazyLock::new(Info::load);

#[derive(Clone)]
pub struct Info {
    pub git_commit_hash: String,
    pub git_commit_short_hash: String,
    pub reaction_emojis: Vec<String>,
    pub version: String,
}

impl Info {
    fn load() -> Self {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("could not get git commit hash");

        let git_commit_hash = String::from_utf8(output.stdout).expect("could not parse git commit hash");

        Self {
            git_commit_hash: git_commit_hash.clone(),
            git_commit_short_hash: git_commit_hash[0..7].to_owned(),
            reaction_emojis: REACTION_EMOJIS.clone().iter().map(|s| (*s).to_owned()).collect(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
