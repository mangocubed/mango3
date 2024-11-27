use std::env::set_var;

use chrono::{DateTime, Utc};
use fake::faker::address::en::CountryCode;
use fake::faker::chrono::en::DateTimeBefore;
use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use regex::Regex;
use url::Url;
use uuid::Uuid;

use crate::models::{NavigationItem, User, Website};
use crate::CoreContext;

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
    SafeEmail().fake()
}

pub fn fake_name() -> String {
    Name().fake()
}

fn fake_password() -> String {
    Password(6..128).fake()
}

fn fake_sentence() -> String {
    Sentence(2..5).fake()
}

fn fake_slug() -> String {
    Regex::new(r"[._]")
        .unwrap()
        .replace_all(&fake_username(), "-")
        .to_string()
}

fn fake_username() -> String {
    let mut username = Username().fake::<String>();
    username.truncate(16);
    username
}

pub fn fake_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn fake_url() -> String {
    Faker.fake::<Url>().to_string()
}

pub async fn insert_test_navigation_item(core_context: &CoreContext, website: Option<&Website>) -> NavigationItem {
    let website = if let Some(website) = website {
        website
    } else {
        &insert_test_website(core_context).await
    };
    let label = fake_name();
    let url = fake_url();

    NavigationItem::insert(core_context, &website, 0, &label, &url)
        .await
        .ok()
        .unwrap()
}

pub async fn insert_test_user(core_context: &CoreContext) -> User {
    let username = fake_username();
    let email = fake_email();
    let password = fake_password();
    let full_name = fake_name();
    let birthdate = fake_birthdate();
    let country_alpha2 = fake_country_alpha2();

    User::insert(
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
    .ok()
    .unwrap()
}

pub async fn insert_test_website(core_context: &CoreContext) -> Website {
    let user = insert_test_user(core_context).await;
    let name = fake_name();
    let subdomain = fake_slug();
    let description = fake_sentence();

    Website::insert(core_context, &user, &name, &subdomain, &description)
        .await
        .ok()
        .unwrap()
}

pub async fn setup_core_context() -> CoreContext {
    set_var("DATABASE_URL", "postgres://mango3:mango3@127.0.0.1:5432/mango3_test");
    set_var("JOBS_REDIS_URL", "redis://127.0.0.1:6379/6");
    set_var("SESSIONS_REDIS_URL", "redis://127.0.0.1:6379/7");

    CoreContext::setup().await
}
