use apalis::prelude::Error;

use mango3_core::enums::AdminMailerJobCommand;
use mango3_core::models::User;
use mango3_core::utils::*;
use mango3_core::CoreContext;

use crate::constants::{
    KEY_TEXT_HELLO, KEY_TEXT_NEW_USER_ACCOUNT_CREATED,
    KEY_TEXT_SOMEONE_HAS_CREATED_A_NEW_USER_ACCOUNT_WITH_THE_FOLLOWING_USERNAME,
    KEY_TEXT_THIS_USER_ACCOUNT_IS_DISABLED_BY_DEFAULT,
};

use super::send_email;

pub async fn admin_mailer_worker(job: AdminMailerJob) -> Result<(), Error> {
    match job.command {
        AdminMailerJobCommand::NewUser(new_user) => send_new_user_emails(&new_user).await,
    }

    Ok(())
}

async fn send_new_user_emails(new_user: &User) {
    let core_context = CoreContext::setup().await;

    let users = User::all_admins(&core_context).await;

    for user in users {
        let i18n = user.i18n();
        let title = i18n.text(KEY_TEXT_NEW_USER_ACCOUNT_CREATED);
        let mut message = format!(
            "{} @{},\n\n{}: @{}",
            i18n.text(KEY_TEXT_HELLO),
            user.username,
            i18n.text(KEY_TEXT_SOMEONE_HAS_CREATED_A_NEW_USER_ACCOUNT_WITH_THE_FOLLOWING_USERNAME),
            new_user.username
        );

        if user.is_disabled() {
            message += &format!("\n\n{}.", i18n.text(KEY_TEXT_THIS_USER_ACCOUNT_IS_DISABLED_BY_DEFAULT));
        }

        let _ = send_email(&user.email, &title, &message).await;
    }
}
