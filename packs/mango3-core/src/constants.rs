use std::sync::LazyLock;

use regex::Regex;

#[cfg(feature = "insert-blob")]
pub(crate) const ALLOWED_FILE_TYPES: [&str; 5] = ["image/bmp", "image/gif", "image/jpeg", "image/png", "image/webp"];

pub const BLACKLISTED_HASHTAGS: [&str; 6] = ["each", "if", "log", "lookup", "unless", "with"];

#[cfg(any(
    feature = "insert-post",
    feature = "insert-user",
    feature = "insert-website",
    feature = "update-post"
))]
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

#[cfg(feature = "update-website")]
pub(crate) static DARK_THEMES: LazyLock<[&str; 14]> = LazyLock::new(|| {
    [
        "dark",
        "abyss",
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

#[cfg(feature = "update-website")]
pub(crate) static LIGHT_THEMES: LazyLock<[&str; 21]> = LazyLock::new(|| {
    [
        "light",
        "acid",
        "autumn",
        "bumblebee",
        "caramellatte",
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
        "silk",
        "valentine",
        "winter",
        "wireframe",
    ]
});

pub static REACTION_EMOJIS: LazyLock<[&str; 24]> = LazyLock::new(|| {
    [
        "😀", "😂", "🥹", "🙂", "🙃", "🧐", "😏", "😒", "🙁", "😢", "😡", "🤯", "🤔", "🫠", "😶", "😑", "😦", "😴",
        "🤤", "🤡", "💩", "🖕", "👍", "👎",
    ]
});

pub static REGEX_FIND_HASHTAGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#(?<name>[a-zA-Z0-9]+(?:[-_][a-zA-Z0-9]+)*)").unwrap());

#[cfg(feature = "regex-email")]
pub(crate) static REGEX_EMAIL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[^@\s]+@[^@\s]+\z").unwrap());
#[cfg(feature = "regex-handlebars")]
pub(crate) static REGEX_HANDLEBARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|[^\\])\{\{(?s:.)*?\}\}").unwrap());
#[cfg(feature = "regex-hashtag")]
pub(crate) static REGEX_HASHTAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:[-_][[:alnum:]]+)*\z").unwrap());
#[cfg(feature = "regex-slug")]
pub(crate) static REGEX_SLUG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:-[[:alnum:]]+)*\z").unwrap());
#[cfg(feature = "regex-subdomain")]
pub(crate) static REGEX_SUBDOMAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[[:alnum:]]+(?:-[[:alnum:]]+)*\z").unwrap());
#[cfg(feature = "regex-username")]
pub(crate) static REGEX_USERNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[-_.]?([[:alnum:]]+[-_.]?)+\z").unwrap());

pub(crate) const HASHTAG_LOOKAROUND: [Option<&str>; 3] = [Some(" "), Some("\n"), None];

#[cfg(feature = "all-hashtags-by-ids")]
pub(crate) const PREFIX_ALL_HASHTAGS_BY_IDS: &str = "all_hashtags_by_ids";
#[cfg(feature = "all-navigation-items-by-website")]
pub(crate) const PREFIX_ALL_NAVIGATION_ITEMS_BY_WEBSITE: &str = "all_navigation_items_by_website";
#[cfg(feature = "get-blob-by-id")]
pub(crate) const PREFIX_GET_BLOB_BY_ID: &str = "get_blob_by_id";
#[cfg(feature = "get-hashtag-by-id")]
pub(crate) const PREFIX_GET_HASHTAG_BY_ID: &str = "get_hashtag_by_id";
#[cfg(feature = "get-hashtag-by-name")]
pub(crate) const PREFIX_GET_HASHTAG_BY_NAME: &str = "get_hashtag_by_name";
#[cfg(feature = "get-post-by-id")]
pub(crate) const PREFIX_GET_POST_BY_ID: &str = "get_post_by_id";
#[cfg(feature = "get-post-by-slug")]
pub(crate) const PREFIX_GET_POST_BY_SLUG: &str = "get_post_by_slug";
#[cfg(feature = "get-user-by-id")]
pub(crate) const PREFIX_GET_USER_BY_ID: &str = "get_user_by_id";
#[cfg(feature = "get-user-by-username")]
pub(crate) const PREFIX_GET_USER_BY_USERNAME: &str = "get_user_by_username";
#[cfg(feature = "get-user-by-username-or-email")]
pub(crate) const PREFIX_GET_USER_BY_USERNAME_OR_EMAIL: &str = "get_user_by_username_or_email";
#[cfg(feature = "get-user-session-by-id")]
pub(crate) const PREFIX_GET_USER_SESSION_BY_ID: &str = "get_user_session_by_id";
#[cfg(feature = "get-website-by-id")]
pub(crate) const PREFIX_GET_WEBSITE_BY_ID: &str = "get_website_by_id";
#[cfg(feature = "get-website-by-subdomain")]
pub(crate) const PREFIX_GET_WEBSITE_BY_SUBDOMAIN: &str = "get_website_by_subdomain";
#[cfg(feature = "post-comment")]
pub(crate) const PREFIX_POST_COMMENT_CONTENT_HTML: &str = "post_comment_content_html";
#[cfg(feature = "post-content-html")]
pub(crate) const PREFIX_POST_CONTENT_HTML: &str = "post_content_html";
#[cfg(feature = "post")]
pub(crate) const PREFIX_POST_CONTENT_PREVIEW_HTML: &str = "post_content_preview_html";
#[cfg(feature = "user-bio-html")]
pub(crate) const PREFIX_USER_BIO_HTML: &str = "user_bio_html";
#[cfg(feature = "user")]
pub(crate) const PREFIX_USER_BIO_PREVIEW_HTML: &str = "user_bio_preview_html";
#[cfg(feature = "website-description-html")]
pub(crate) const PREFIX_WEBSITE_DESCRIPTION_HTML: &str = "website_description_html";
#[cfg(feature = "website")]
pub(crate) const PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML: &str = "website_description_preview_html";
