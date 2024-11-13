use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub(crate) static ref BLACKLISTED_SUBDOMAINS: Vec<String> = vec![
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
        "ns".to_owned(),
        "pkg".to_owned(),
        "pop3".to_owned(),
        "pop3s".to_owned(),
        "profile".to_owned(),
        "profiles".to_owned(),
        "register".to_owned(),
        "reset-password".to_owned(),
        "root".to_owned(),
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
    pub(crate) static ref BLACKLISTED_USERNAMES: Vec<String> = BLACKLISTED_SUBDOMAINS.to_vec();
    pub(crate) static ref REGEX_EMAIL: Regex = Regex::new(r"\A[^@\s]+@[^@\s]+\z").unwrap();
    pub(crate) static ref REGEX_SUBDOMAIN: regex::Regex = regex::Regex::new(r"\A[a-z0-9]+(?:-[a-z0-9]+)*\z").unwrap();
    pub(crate) static ref REGEX_USERNAME: Regex = Regex::new(r"\A[-_.]?([a-zA-Z0-9]+[-_.]?)+\z").unwrap();
}
