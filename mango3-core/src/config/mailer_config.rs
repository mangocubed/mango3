use serde::{Deserialize, Serialize};

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct MailerConfig {
    pub enable: bool,
    pub sender_address: String,
    pub smtp_address: String,
    pub smtp_password: String,
    pub smtp_security: String,
    pub smtp_username: String,
}

impl Default for MailerConfig {
    fn default() -> Self {
        Self {
            enable: false,
            sender_address: "Mango³ Dev <no-reply@localhost>".to_owned(),
            smtp_address: "localhost".to_owned(),
            smtp_password: "".to_owned(),
            smtp_security: "none".to_owned(),
            smtp_username: "".to_owned(),
        }
    }
}

impl MailerConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("MAILER_")
    }
}
