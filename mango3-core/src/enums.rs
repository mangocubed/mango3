use serde::{Deserialize, Serialize};

#[derive(strum::Display, Clone, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Input {
    Bio,
    Birthdate,
    CountryAlpha2,
    CurrentPassword,
    Description,
    DisplayName,
    Email,
    FullName,
    Name,
    NewPassword,
    Password,
    Subdomain,
    Username,
    UsernameOrEmail,
}

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
pub enum MailerJobCommand {
    Welcome,
}
