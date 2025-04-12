use std::collections::HashSet;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use chrono::{DateTime, Utc};
use fake::faker::address::en::CountryCode;
use fake::faker::chrono::en::DateTimeBefore;
use fake::faker::internet::en::{FreeEmail, IPv4, Password, Username};
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use rand::rng;
use url::Url;
use uuid::Uuid;

use crate::models::{NavigationItem, User, Website};
use crate::CoreContext;

mod test_blob;
mod test_post;
mod test_post_comment;

pub use test_blob::insert_test_blob;
pub use test_post::insert_test_post;
pub use test_post_comment::insert_test_post_comment;

fn unique_fake<T, F>(prefix: &str, fake_fn: F) -> T
where
    F: Fn() -> T,
    T: Display,
{
    let file_path = std::env::temp_dir().join("unique_fake");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(&file_path)
        .expect("Could not open file");

    let mut file_content = String::new();

    let _ = file.read_to_string(&mut file_content);

    let mut lines = file_content
        .lines()
        .map(|line| line.to_owned())
        .collect::<HashSet<String>>();

    if lines.len() > 200 {
        for line in lines.clone().iter().take(lines.len() - 200) {
            lines.remove(line);
        }
    }

    let _ = file.set_len(0);

    for line in &lines {
        let _ = file.write_all(format!("{line}\n").as_bytes());
    }

    let mut fake = fake_fn();

    while !lines.insert(format!("{prefix}_{fake}")) {
        fake = fake_fn();
    }

    let _ = file.write_all(format!("{prefix}_{fake}\n").as_bytes());

    fake
}

fn fake_birthdate() -> String {
    DateTimeBefore(Utc::now())
        .fake::<DateTime<Utc>>()
        .date_naive()
        .to_string()
}

fn fake_country_alpha2() -> String {
    CountryCode().fake()
}

fn fake_email() -> String {
    unique_fake("email", || FreeEmail().fake_with_rng(&mut rng()))
}

pub fn fake_ipv4() -> String {
    IPv4().fake()
}

pub fn fake_name() -> String {
    unique_fake("name", || {
        let mut name: String = Name().fake_with_rng(&mut rng());
        name.truncate(256);
        name
    })
}

pub fn fake_paragraph() -> String {
    Paragraph(1..3).fake()
}

fn fake_password() -> String {
    Password(6..128).fake()
}

pub fn fake_sentence() -> String {
    Sentence(2..5).fake()
}

pub fn fake_slug() -> String {
    unique_fake("slug", || {
        let mut slug: String = Username().fake_with_rng(&mut rng());
        slug.truncate(256);
        slug.replace("_", "-").replace(".", "-")
    })
}

pub fn fake_username() -> String {
    unique_fake("username", || {
        let mut username: String = Username().fake_with_rng(&mut rng());
        username.truncate(16);
        username
    })
}

pub fn fake_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn fake_url() -> String {
    Faker.fake::<Url>().to_string()
}

pub async fn insert_test_navigation_item<'a>(
    core_context: &CoreContext,
    website: Option<&Website>,
) -> NavigationItem<'a> {
    let website = if let Some(website) = website {
        website
    } else {
        &insert_test_website(core_context, None).await
    };
    let label = fake_name();
    let url = fake_url();

    crate::commands::insert_navigation_item(&website, 0, &label, &url)
        .await
        .ok()
        .expect("Could not insert navigation item")
        .data
}

pub async fn insert_test_user(core_context: &CoreContext) -> User {
    let username = fake_username();
    let email = fake_email();
    let password = fake_password();
    let full_name = fake_name();
    let birthdate = fake_birthdate();
    let country_alpha2 = fake_country_alpha2();

    crate::commands::insert_user(
        core_context,
        &username,
        &email,
        &password,
        &full_name,
        &birthdate,
        "en",
        &country_alpha2,
    )
    .await
    .expect("Could not insert user")
    .data
}

pub async fn insert_test_website(core_context: &CoreContext, user: Option<&User>) -> Website {
    let user = if let Some(user) = user {
        user
    } else {
        &insert_test_user(core_context).await
    };
    let name = fake_name();
    let subdomain = fake_slug();
    let description = fake_sentence();

    crate::commands::insert_website(core_context, &user, &name, &subdomain, &description)
        .await
        .ok()
        .expect("Could not insert website")
        .data
}

pub async fn setup_core_context() -> CoreContext {
    CoreContext::setup().await
}
