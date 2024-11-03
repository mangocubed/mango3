use serde::{Deserialize, Serialize};

#[derive(strum::Display, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Input {}

#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum InputError {
    AlreadyInUse,
    CantBeBlank,
    CantBePresent,
    InvalidFormat,
    InvalidLength(Option<usize>, Option<usize>),
    IsInvalid,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MailerJobCommand {}
