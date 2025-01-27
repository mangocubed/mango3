use std::sync::LazyLock;

use regex::Regex;

pub(crate) const ALLOWED_FILE_TYPES: [&str; 5] = ["image/bmp", "image/gif", "image/jpeg", "image/png", "image/webp"];

pub const BLACKLISTED_HASHTAGS: [&str; 6] = ["each", "if", "log", "lookup", "unless", "with"];

pub(crate) static BLACKLISTED_SLUGS: LazyLock<[&str; 80]> = LazyLock::new(|| {
    [
        "_dmarc",
        "account",
        "accounts",
        "admin",
        "administrator",
        "api",
        "asset",
        "assets",
        "app",
        "apps",
        "auth",
        "authentication",
        "authenticator",
        "blog",
        "cdn",
        "cloud",
        "dash",
        "dashboard",
        "dmarc",
        "dns",
        "editor",
        "email",
        "hosting",
        "http",
        "https",
        "forum",
        "graphql",
        "groups",
        "hashtag",
        "hashtags",
        "imap",
        "inbound",
        "legal",
        "login",
        "mail",
        "mango",
        "mango3",
        "monitor",
        "mta",
        "my-account",
        "new-website",
        "ns",
        "pkg",
        "pop3",
        "pop3s",
        "post",
        "posts",
        "profile",
        "profiles",
        "register",
        "reset-password",
        "root",
        "search",
        "shop",
        "sign-in",
        "sign-out",
        "sign-up",
        "signin",
        "signout",
        "signup",
        "smtp",
        "smtps",
        "stat",
        "stats",
        "status",
        "store",
        "studio",
        "upload",
        "uploads",
        "user",
        "users",
        "web",
        "webapi",
        "webapp",
        "webapps",
        "webmail",
        "website",
        "websites",
        "wiki",
        "www",
    ]
});

pub(crate) static DARK_THEMES: LazyLock<[&str; 13]> = LazyLock::new(|| {
    [
        "dark",
        "aqua",
        "black",
        "business",
        "coffee",
        "dim",
        "dracula",
        "forest",
        "halloween",
        "luxury",
        "night",
        "sunset",
        "synthwave",
    ]
});

pub(crate) static LIGHT_THEMES: LazyLock<[&str; 19]> = LazyLock::new(|| {
    [
        "light",
        "acid",
        "autumn",
        "bumblebee",
        "cmyk",
        "corporate",
        "cupcake",
        "cyberpunk",
        "fantasy",
        "emerald",
        "garden",
        "lemonade",
        "lofi",
        "nord",
        "pastel",
        "retro",
        "valentine",
        "winter",
        "wireframe",
    ]
});

pub static REACTION_EMOJIS: LazyLock<[&str; 16]> = LazyLock::new(|| {
    [
        "😀", "😂", "🥹", "🙂", "🙃", "🙁", "😢", "😡", "🤯", "🤔", "😦", "🤡", "💩", "🖕", "👍", "👎",
    ]
});

pub(crate) static REGEX_EMAIL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[^@\s]+@[^@\s]+\z").unwrap());
pub static REGEX_FIND_HASHTAGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#(?<name>[a-zA-Z0-9]+(?:[-_][a-zA-Z0-9]+)*)").unwrap());
pub static REGEX_HANDLEBARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[^\\])\{\{(?s:.)*?\}\}").unwrap());
pub(crate) static REGEX_HASHTAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:[-_][[:alnum:]]+)*\z").unwrap());
pub(crate) static REGEX_SLUG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:-[[:alnum:]]+)*\z").unwrap());
pub(crate) static REGEX_SUBDOMAIN: LazyLock<Regex> = LazyLock::new(|| REGEX_SLUG.clone());
pub(crate) static REGEX_USERNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[-_.]?([[:alnum:]]+[-_.]?)+\z").unwrap());

pub(crate) const HASHTAG_LOOKAROUND: [Option<&str>; 3] = [Some(" "), Some("\n"), None];

pub(crate) const KEY_TEXT_CONFIRM_YOUR_EMAIL: &str = "confirm-your-email";
pub(crate) const KEY_TEXT_RESET_YOUR_PASSWORD: &str = "reset-your-password";
