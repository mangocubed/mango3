use crate::models::InvitationCode;
use crate::utils::*;
use crate::CoreContext;

#[cfg(feature = "delete-invitation-code")]
pub async fn delete_invitation_code(core_context: &CoreContext, invitation_code: &InvitationCode) -> MutResult {
    sqlx::query!("DELETE FROM invitation_codes WHERE id = $1", invitation_code.id)
        .execute(&core_context.db_pool)
        .await?;

    crate::mut_success!()
}

#[cfg(feature = "get-invitation-code")]
pub async fn get_invitation_code(core_context: &CoreContext, code: &str) -> sqlx::Result<InvitationCode> {
    sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitation_codes WHERE code = $1 LIMIT 1",
        code
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-invitation-code-by-id")]
pub async fn get_invitation_code_by_id(core_context: &CoreContext, id: uuid::Uuid) -> sqlx::Result<InvitationCode> {
    sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitation_codes WHERE id = $1 LIMIT 1",
        id
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "insert-invitation-code")]
pub async fn insert_invitation_code(core_context: &CoreContext, email: &str) -> MutResult<InvitationCode> {
    use crate::enums::{Input, InputError};
    use crate::utils::ValidatorTrait;

    let mut validator = crate::validator!();

    let email = email.trim().to_lowercase();

    if validator.validate_presence(Input::Email, &email)
        && validator.validate_length(Input::Email, &email, Some(5), Some(256))
        && validator.validate_format(Input::Email, &email, &crate::constants::REGEX_EMAIL)
    {
        let email_exists = sqlx::query!(
            "SELECT id FROM users WHERE LOWER(email) = $1 LIMIT 1",
            email // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok();
        validator.custom_validation(Input::Email, InputError::AlreadyInUse, &|| !email_exists);
    }

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let code = crate::utils::generate_random_string(crate::config::MISC_CONFIG.invitation_code_length);

    let result = sqlx::query_as!(
        InvitationCode,
        "INSERT INTO invitation_codes (email, code) VALUES ($1, $2) RETURNING *",
        email, // $1
        code,  // $2
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(invitation_code) => {
            core_context
                .jobs
                .guest_mailer(&email, crate::enums::GuestMailerJobCommand::InvitationCode(code))
                .await;

            crate::mut_success!(invitation_code)
        }
        Err(_) => crate::mut_error!(),
    }
}
