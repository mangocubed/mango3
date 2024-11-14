use std::collections::HashMap;

use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use mango3_core::config::{BASIC_CONFIG, MAILER_CONFIG};
use mango3_core::enums::MailerJobCommand;
use mango3_core::jobs::MailerJob;
use mango3_core::locales::I18n;
use mango3_core::models::User;

use crate::constants::*;

pub async fn mailer_worker(job: MailerJob) {
    let i18n = job.user.i18n();

    match job.command {
        MailerJobCommand::ConfirmationCode { action, code } => {
            send_confirmation_code_email(&i18n, &job.user, &action, &code).await
        }
        MailerJobCommand::Welcome => send_welcome_email(&i18n, &job.user).await,
    }
}

pub async fn send_confirmation_code_email(i18n: &I18n, user: &User, action: &str, code: &str) {
    let title = i18n.text(KEY_TEXT_CONFIRMATION_CODE);
    let mut text_args = HashMap::new();
    text_args.insert(KEY_TEXT_ARG_ACTION.to_owned(), action.into());
    let message = format!(
        "{} {},\n\n{}:\n\n{}",
        i18n.text(KEY_TEXT_HELLO),
        user.username,
        i18n.text_with_args(KEY_TEXT_USE_THIS_CODE_TO_ACTION, &text_args),
        code,
    );

    let _ = send_email(&user.email, &title, &message).await;
}

async fn send_welcome_email(i18n: &I18n, user: &User) {
    let mut text_args = HashMap::new();
    text_args.insert(KEY_TEXT_ARG_TITLE.to_owned(), BASIC_CONFIG.title.clone().into());
    let title = i18n.text_with_args(KEY_TEXT_WELCOME_TO_TITLE, &text_args);
    let message = format!("{} @{},\n\n{}", i18n.text(KEY_TEXT_HELLO), user.username, title);

    let _ = send_email(&user.email, &title, &message).await;
}

async fn send_email(to: &str, subject: &str, body: &str) {
    if !MAILER_CONFIG.enable {
        return;
    }

    let message = Message::builder()
        .from(
            MAILER_CONFIG
                .sender_address
                .parse()
                .expect("Could not parse mailer sender address"),
        )
        .to(to.parse().expect("Could not parse recipient address"))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .expect("Could not build message");

    let credentials = Credentials::new(
        MAILER_CONFIG.smtp_username.to_owned(),
        MAILER_CONFIG.smtp_password.to_owned(),
    );

    match MAILER_CONFIG.smtp_security.as_str() {
        "tls" => AsyncSmtpTransport::<Tokio1Executor>::relay(&MAILER_CONFIG.smtp_address),
        "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&MAILER_CONFIG.smtp_address),
        _ => Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
            MAILER_CONFIG.smtp_address.clone(),
        )),
    }
    .expect("Could not get SMTP transport builder")
    .credentials(credentials)
    .build()
    .send(message)
    .await
    .expect("Could not send email");
}
