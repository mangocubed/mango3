pub(crate) const COOKIE_NAME_LANGUAGE: &str = "_mango3_language";

#[cfg(feature = "forms")]
pub(crate) const KEY_CODE_ENTER: u32 = 13;
#[cfg(feature = "markdown-editor")]
pub(crate) const KEY_CODE_5: u32 = 53;
#[cfg(feature = "markdown-editor")]
pub(crate) const KEY_CODE_B: u32 = 66;
#[cfg(feature = "markdown-editor")]
pub(crate) const KEY_CODE_I: u32 = 73;
#[cfg(feature = "markdown-editor")]
pub(crate) const KEY_CODE_K: u32 = 75;

#[cfg(feature = "ssr")]
pub const KEY_CONFIRMATION_CODE_ID: &str = "confirmation_code_id";
#[cfg(any(feature = "ssr", feature = "server"))]
pub const KEY_USER_SESSION_ID: &str = "user_session_id";

pub const KEY_PARAM_NAME: &str = "name";
