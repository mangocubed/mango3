use crate::models::*;
use crate::utils::*;
use crate::CoreContext;

#[cfg(feature = "confirm-confirmation-code")]
pub async fn confirm_confirmation_code<F, IF>(
    core_context: &CoreContext,
    confirmation_code: &ConfirmationCode,
    action: crate::enums::ConfirmationCodeAction,
    code: &str,
    on_success: F,
) -> MutResult
where
    F: Fn() -> IF,
    IF: IntoFuture<Output = Result<(), ValidationErrors>>,
{
    let mut validator = crate::validator!();

    if action != confirmation_code.action {
        return crate::mut_error_result!();
    }

    if validator.validate_presence(Input::Code, code) {
        let code_is_verified = confirmation_code.verify_code(core_context, code).await;

        validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
    }

    if !validator.is_valid {
        return crate::mut_error_result!(validator.errors);
    }

    let result = on_success().await;

    match result {
        Ok(()) => {
            let _ = confirmation_code.delete(core_context).await;

            crate::mut_success_result!()
        }
        errors => errors,
    }
}

#[cfg(feature = "delete-confirmation-code")]
pub async fn delete_confirmation_code(core_context: &CoreContext, confirmation_code: &ConfirmationCode) -> MutResult {
    sqlx::query!("DELETE FROM confirmation_codes WHERE id = $1", self.id)
        .execute(&core_context.db_pool)
        .await
}

#[cfg(feature = "delete-all-expired-confirmation-codes")]
pub async fn delete_all_expired_confirmation_codes(core_context: &CoreContext) -> MutResult {
    sqlx::query!("DELETE FROM confirmation_codes WHERE created_at < current_timestamp - INTERVAL '1 hour'")
        .execute(&core_context.db_pool)
        .await?;

    crate::mut_success_result!()
}

#[cfg(feature = "insert-confirmation-code")]
pub async fn insert_confirmation_code(
    core_context: &CoreContext,
    user: &User,
    action: crate::enums::ConfirmationCodeAction,
) -> MutResult<ConfirmationCode> {
    if let Ok(confirmation_code) = get_confirmation_code_by_user(core_context, user, action.clone()).await {
        return crate::mut_success_result!(confirmation_code);
    }

    let code = generate_random_string(crate::config::MISC_CONFIG.confirmation_code_length);

    let encrypted_code = encrypt_password(&code);

    let result = sqlx::query_as!(
        ConfirmationCode,
        r#"INSERT INTO confirmation_codes (user_id, action, encrypted_code) VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at"#,
        user.id,                                  // $1
        action.clone() as ConfirmationCodeAction, // $2
        encrypted_code                            // $3
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(confirmation_code) => {
            core_context
                .jobs
                .mailer(user, MailerJobCommand::ConfirmationCode { action, code })
                .await;

            crate::mut_success_result!(confirmation_code)
        }
        Err(_) => crate::mut_error_result!(),
    }
}
