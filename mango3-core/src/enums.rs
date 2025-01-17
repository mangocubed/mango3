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
    DarkTheme,
    Description,
    DisplayName,
    Email,
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

#[derive(sqlx::Type, strum::Display, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserRole {
    User,
    Creator,
    Admin,
    Superuser,
}

impl From<&String> for UserRole {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "superuser" => Self::Superuser,
            "admin" => Self::Admin,
            "creator" => Self::Creator,
            _ => Self::User,
        }
    }
}
