use chrono::{DateTime, Utc};
use fake::faker::address::en::CountryCode;
use fake::faker::chrono::en::DateTimeBefore;
use fake::faker::internet::en::{IPv4, Password, SafeEmail, Username};
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use rand::{thread_rng, Rng};
use regex::Regex;
use url::Url;
use uuid::Uuid;

use crate::models::{NavigationItem, User, Website};
use crate::CoreContext;

mod test_blob;
mod test_post;

pub use test_blob::insert_test_blob;
pub use test_post::insert_test_post;

fn random_number() -> i32 {
    thread_rng().gen_range(0..99)
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
    format!("{}_{}", random_number(), SafeEmail().fake::<String>())
}

pub fn fake_ipv4() -> String {
    IPv4().fake()
}

pub fn fake_name() -> String {
    let mut name = Name().fake::<String>();
    name.truncate(253);

    format!("{} {}", name, random_number())
}

fn fake_paragraph() -> String {
    Paragraph(1..3).fake()
}

fn fake_password() -> String {
    Password(6..128).fake()
}

pub fn fake_sentence() -> String {
    Sentence(2..5).fake()
}

pub fn fake_slug() -> String {
    Regex::new(r"[._]")
        .unwrap()
        .replace_all(&fake_username(), "-")
        .to_string()
}

fn fake_username() -> String {
    let mut username = Username().fake::<String>();
    username.truncate(13);
    format!("{}_{}", username, random_number())
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
        &insert_test_website(core_context, None).await
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

pub async fn insert_test_website(core_context: &CoreContext, user: Option<&User>) -> Website {
    let user = if let Some(user) = user {
        user
    } else {
        &insert_test_user(core_context).await
    };
    let name = fake_name();
    let subdomain = fake_slug();
    let description = fake_sentence();

    Website::insert(core_context, &user, &name, &subdomain, &description)
        .await
        .ok()
        .unwrap()
}

pub async fn setup_core_context() -> CoreContext {
    CoreContext::setup().await
}
