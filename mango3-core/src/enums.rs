use serde::{Deserialize, Serialize};

#[derive(strum::Display, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Input {
    Birthdate,
    CountryAlpha2,
    Description,
    Email,
    FullName,
    Name,
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
