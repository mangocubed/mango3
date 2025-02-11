use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum GuestMailerJobCommand {
    InvitationCode(String),
}

#[derive(strum::Display, Clone, Debug, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Input {
    Bio,
    Birthdate,
    Code,
    Content,
    CountryAlpha2,
    CurrentPassword,
    DarkTheme,
    Description,
    DisplayName,
    Email,
    Emoji,
    FullName,
    LightTheme,
    Name,
    NewPassword,
    Password,
    Slug,
    Subdomain,
    Title,
    Username,
    UsernameOrEmail,
    Url,
    Variables,
}

#[derive(strum::Display, Debug)]
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
    NewUserSession,
    Locked,
    Welcome,
}

#[derive(sqlx::Type, strum::Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserRole {
    User,
    Creator,
    Admin,
    Superuser,
}

#[derive(Debug)]
pub struct FromStrError;

impl FromStr for UserRole {
    type Err = FromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "superuser" => Ok(Self::Superuser),
            "admin" => Ok(Self::Admin),
            "creator" => Ok(Self::Creator),
            "user" => Ok(Self::User),
            _ => Err(FromStrError),
        }
    }
}

impl From<&String> for UserRole {
    fn from(value: &String) -> Self {
        Self::from_str(value).expect("User role is invalid")
    }
}
