use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum GuestMailerJobCommand {
    InvitationCode(String),
}

#[derive(strum::Display, Clone, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Input {
    Bio,
    Birthdate,
    Code,
    Content,
    CountryAlpha2,
    CurrentPassword,
    Description,
    DisplayName,
    Email,
    FullName,
    Name,
    NewPassword,
    Password,
    Slug,
    Subdomain,
    Title,
    Username,
    UsernameOrEmail,
    Url,
}

#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum InputError {
    AlreadyInUse,
    CantBeBlank,
    CantBePresent,
    InvalidFormat,
    InvalidLength(Option<u32>, Option<u32>),
    IsInvalid,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MailerJobCommand {
    ConfirmationCode { action: String, code: String },
    Welcome,
}
