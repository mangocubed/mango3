use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use mango3_core::config::MAILER_CONFIG;

mod admin_mailer_worker;
mod guest_mailer_worker;
mod mailer_worker;

pub use admin_mailer_worker::admin_mailer_worker;
pub use guest_mailer_worker::guest_mailer_worker;
pub use mailer_worker::mailer_worker;

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
