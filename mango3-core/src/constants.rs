use std::sync::LazyLock;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub(crate) static ref BLACKLISTED_SLUGS: Vec<String> = vec![
        "_dmarc".to_owned(),
        "account".to_owned(),
        "accounts".to_owned(),
        "admin".to_owned(),
        "administrator".to_owned(),
        "api".to_owned(),
        "asset".to_owned(),
        "assets".to_owned(),
        "app".to_owned(),
        "apps".to_owned(),
        "auth".to_owned(),
        "authentication".to_owned(),
        "authenticator".to_owned(),
        "blog".to_owned(),
        "cdn".to_owned(),
        "cloud".to_owned(),
        "dash".to_owned(),
        "dashboard".to_owned(),
        "dmarc".to_owned(),
        "dns".to_owned(),
        "editor".to_owned(),
        "email".to_owned(),
        "hosting".to_owned(),
        "http".to_owned(),
        "https".to_owned(),
        "forum".to_owned(),
        "graphql".to_owned(),
        "groups".to_owned(),
        "hashtag".to_owned(),
        "hashtags".to_owned(),
        "imap".to_owned(),
        "inbound".to_owned(),
        "legal".to_owned(),
        "login".to_owned(),
        "mail".to_owned(),
        "mango".to_owned(),
        "mango3".to_owned(),
        "monitor".to_owned(),
        "mta".to_owned(),
        "my-account".to_owned(),
        "new-website".to_owned(),
        "ns".to_owned(),
        "pkg".to_owned(),
        "pop3".to_owned(),
        "pop3s".to_owned(),
        "post".to_owned(),
        "posts".to_owned(),
        "profile".to_owned(),
        "profiles".to_owned(),
        "register".to_owned(),
        "reset-password".to_owned(),
        "root".to_owned(),
        "search".to_owned(),
        "shop".to_owned(),
        "sign-in".to_owned(),
        "sign-out".to_owned(),
        "sign-up".to_owned(),
        "signin".to_owned(),
        "signout".to_owned(),
        "signup".to_owned(),
        "smtp".to_owned(),
        "smtps".to_owned(),
        "stat".to_owned(),
        "stats".to_owned(),
        "status".to_owned(),
        "store".to_owned(),
        "studio".to_owned(),
        "upload".to_owned(),
        "uploads".to_owned(),
        "user".to_owned(),
        "users".to_owned(),
        "web".to_owned(),
        "webapi".to_owned(),
        "webapp".to_owned(),
        "webapps".to_owned(),
        "webmail".to_owned(),
        "website".to_owned(),
        "websites".to_owned(),
        "wiki".to_owned(),
        "www".to_owned(),
    ];
    pub(crate) static ref BLACKLISTED_SUBDOMAINS: Vec<String> = BLACKLISTED_SLUGS.to_vec();
    pub(crate) static ref BLACKLISTED_USERNAMES: Vec<String> = BLACKLISTED_SLUGS.to_vec();
    pub(crate) static ref DARK_THEMES: Vec<String> = vec![
        "dark".to_owned(),
        "aqua".to_owned(),
        "black".to_owned(),
        "business".to_owned(),
        "coffee".to_owned(),
        "dim".to_owned(),
        "dracula".to_owned(),
        "forest".to_owned(),
        "halloween".to_owned(),
        "luxury".to_owned(),
        "night".to_owned(),
        "sunset".to_owned(),
        "synthwave".to_owned(),
    ];
    pub(crate) static ref LIGHT_THEMES: Vec<String> = vec![
        "light".to_owned(),
        "acid".to_owned(),
        "autumn".to_owned(),
        "bumblebee".to_owned(),
        "cmyk".to_owned(),
        "corporate".to_owned(),
        "cupcake".to_owned(),
        "cyberpunk".to_owned(),
        "fantasy".to_owned(),
        "emerald".to_owned(),
        "garden".to_owned(),
        "lemonade".to_owned(),
        "lofi".to_owned(),
        "nord".to_owned(),
        "pastel".to_owned(),
        "retro".to_owned(),
        "valentine".to_owned(),
        "winter".to_owned(),
        "wireframe".to_owned(),
    ];
    pub(crate) static ref REGEX_EMAIL: Regex = Regex::new(r"\A[^@\s]+@[^@\s]+\z").unwrap();
    pub(crate) static ref REGEX_SLUG: Regex = Regex::new(r"\A[[:alnum:]]+(?:-[[:alnum:]]+)*\z").unwrap();
    pub(crate) static ref REGEX_SUBDOMAIN: Regex = REGEX_SLUG.clone();
    pub(crate) static ref REGEX_USERNAME: Regex = Regex::new(r"\A[-_.]?([[:alnum:]]+[-_.]?)+\z").unwrap();
}

pub static ALLOWED_POST_REACTION_EMOJIS: LazyLock<[&str; 16]> = LazyLock::new(|| {
    [
        "😀", "😂", "🥹", "🙂", "🙃", "🙁", "😢", "😡", "🤯", "🤔", "😦", "🤡", "💩", "🖕", "👍", "👎",
    ]
});

pub static BLACKLISTED_HASHTAGS: LazyLock<[&str; 6]> =
    LazyLock::new(|| ["each", "if", "log", "lookup", "unless", "with"]);

pub(crate) const HASHTAG_LOOKAROUND: [Option<&str>; 3] = [Some(" "), Some("\n"), None];

pub(crate) const KEY_TEXT_CONFIRM_YOUR_EMAIL: &str = "confirm-your-email";
pub(crate) const KEY_TEXT_RESET_YOUR_PASSWORD: &str = "reset-your-password";

pub static REGEX_HANDLEBARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[^\\])\{\{(?s:.)*?\}\}").unwrap());
pub static REGEX_FIND_HASHTAGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#(?<name>[a-zA-Z0-9]+(?:[-_][a-zA-Z0-9]+)*)").unwrap());
pub(crate) static REGEX_HASHTAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:[-_][[:alnum:]]+)*\z").unwrap());
