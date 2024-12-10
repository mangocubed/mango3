use serde::{Deserialize, Serialize};

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
            sender_address: "Mango³ Dev <no-reply@mango3.local>".to_owned(),
            smtp_address: "mango3.local".to_owned(),
            smtp_password: "".to_owned(),
            smtp_security: "none".to_owned(),
            smtp_username: "".to_owned(),
        }
    }
}

impl MailerConfig {
    #[cfg(not(test))]
    pub(crate) fn load() -> Self {
        super::extract_from_env("MAILER_")
    }

    #[cfg(test)]
    pub(crate) fn load() -> Self {
        Self::default()
    }
}
