use std::collections::HashMap;

use apalis::prelude::Error;

use mango3_core::config::{BASIC_CONFIG, MISC_CONFIG};
use mango3_core::enums::MailerJobCommand;
use mango3_core::jobs::MailerJob;
use mango3_core::locales::I18n;
use mango3_core::models::User;

use crate::constants::{
    KEY_TEXT_ARG_ACTION, KEY_TEXT_ARG_TITLE, KEY_TEXT_CONFIRMATION_CODE, KEY_TEXT_HELLO,
    KEY_TEXT_IF_NOT_PLEASE_CONTACT_US_AT_THE_FOLLOWING_EMAIL_ADDRESS,
    KEY_TEXT_IF_YOU_RECOGNIZE_THIS_ACTION_YOU_CAN_IGNORE_THIS_MESSAGE, KEY_TEXT_NEW_USER_SESSION_STARTED,
    KEY_TEXT_SOMEONE_HAS_STARTED_A_USER_SESSION_WITH_YOUR_ACCOUNT, KEY_TEXT_USE_THIS_CODE_TO_ACTION,
    KEY_TEXT_WELCOME_TO_TITLE,
    KEY_TEXT_WE_REGRET_TO_INFORM_YOU_THAT_WE_HAVE_LOCKED_YOUR_USER_ACCOUNT_DUE_TO_NON_COMPLIANCE_OF_OUR_TERMS,
    KEY_TEXT_YOUR_USER_ACCOUNT_HAS_BEEN_LOCKED,
};

use super::send_email;

pub async fn mailer_worker(job: MailerJob) -> Result<(), Error> {
    let i18n = job.user.i18n();

    match job.command {
        MailerJobCommand::ConfirmationCode { action, code } => {
            send_confirmation_code_email(&i18n, &job.user, &action, &code).await
        }
        MailerJobCommand::NewUserSession => send_new_user_session_email(&i18n, &job.user).await,
        MailerJobCommand::Locked => send_lock_email(&i18n, &job.user).await,
        MailerJobCommand::Welcome => send_welcome_email(&i18n, &job.user).await,
    }

    Ok(())
}

pub async fn send_confirmation_code_email(i18n: &I18n, user: &User, action: &str, code: &str) {
    let title = i18n.text(KEY_TEXT_CONFIRMATION_CODE);
    let mut text_args = HashMap::new();
    text_args.insert(KEY_TEXT_ARG_ACTION.into(), action.into());
    let message = format!(
        "{} {},\n\n{}:\n\n{}",
        i18n.text(KEY_TEXT_HELLO),
        user.username,
        i18n.text_with_args(KEY_TEXT_USE_THIS_CODE_TO_ACTION, &text_args),
        code,
    );

    let _ = send_email(&user.email, &title, &message).await;
}

pub async fn send_lock_email(i18n: &I18n, user: &User) {
    let title = i18n.text(KEY_TEXT_YOUR_USER_ACCOUNT_HAS_BEEN_LOCKED);
    let message = format!(
        "{} {},\n\n{}.",
        i18n.text(KEY_TEXT_HELLO),
        user.username,
        i18n.text(
            KEY_TEXT_WE_REGRET_TO_INFORM_YOU_THAT_WE_HAVE_LOCKED_YOUR_USER_ACCOUNT_DUE_TO_NON_COMPLIANCE_OF_OUR_TERMS
        )
    );
    let _ = send_email(&user.email, &title, &message).await;
}

async fn send_welcome_email(i18n: &I18n, user: &User) {
    let mut text_args = HashMap::new();
    text_args.insert(KEY_TEXT_ARG_TITLE.into(), BASIC_CONFIG.title.clone().into());
    let title = i18n.text_with_args(KEY_TEXT_WELCOME_TO_TITLE, &text_args);
    let message = format!("{} @{},\n\n{}", i18n.text(KEY_TEXT_HELLO), user.username, title);

    let _ = send_email(&user.email, &title, &message).await;
}

async fn send_new_user_session_email(i18n: &I18n, user: &User) {
    let title = i18n.text(KEY_TEXT_NEW_USER_SESSION_STARTED);
    let message = format!(
        "{} @{},\n\n{}.\n\n{}.\n\n{}: {}",
        i18n.text(KEY_TEXT_HELLO),
        user.username,
        i18n.text(KEY_TEXT_SOMEONE_HAS_STARTED_A_USER_SESSION_WITH_YOUR_ACCOUNT),
        i18n.text(KEY_TEXT_IF_YOU_RECOGNIZE_THIS_ACTION_YOU_CAN_IGNORE_THIS_MESSAGE),
        i18n.text(KEY_TEXT_IF_NOT_PLEASE_CONTACT_US_AT_THE_FOLLOWING_EMAIL_ADDRESS),
        MISC_CONFIG.support_email_address
    );

    let _ = send_email(&user.email, &title, &message).await;
}
