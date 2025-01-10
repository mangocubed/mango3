#[cfg(feature = "ssr")]
use std::sync::LazyLock;

#[cfg(feature = "ssr")]
use regex::Regex;

pub(crate) const COOKIE_NAME_LANGUAGE: &str = "_mango3_language";

#[cfg(feature = "ssr")]
pub(crate) const KEY_USER_SESSION_ID: &str = "user_session_id";

pub const KEY_PARAM_NAME: &str = "name";

#[cfg(feature = "ssr")]
pub(crate) static REGEX_HANDLEBARS_DECLARE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\{\{\s*declare\s+"(?<key>[[:word:]]+)"\s+((?<bool>|true|false)|(?<number>[0-9,\.-]+)|"(?<string>.*)"|(?<array>\[.*\])|(?<object>\{.*\}))\s*\}\}"#).unwrap()
});
