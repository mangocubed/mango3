use mango3_core::enums::GuestMailerJobCommand;
use mango3_core::jobs::GuestMailerJob;
use mango3_core::locales::I18n;

use crate::constants::{KEY_TEXT_HELLO, KEY_TEXT_INVITATION_CODE, KEY_TEXT_USE_THIS_CODE_TO_CREATE_YOUR_ACCOUNT};

use super::send_email;

pub async fn guest_mailer_worker(job: GuestMailerJob) {
    match job.command {
        GuestMailerJobCommand::InvitationCode(code) => send_invitation_code_email(&job.to, &code).await,
    }
}

pub async fn send_invitation_code_email(to: &str, code: &str) {
    let i18n = I18n::default();
    let title = i18n.text(KEY_TEXT_INVITATION_CODE);
    let message = format!(
        "{},\n\n{}:\n\n{}",
        i18n.text(KEY_TEXT_HELLO),
        i18n.text(KEY_TEXT_USE_THIS_CODE_TO_CREATE_YOUR_ACCOUNT),
        code,
    );

    let _ = send_email(to, &title, &message).await;
}
