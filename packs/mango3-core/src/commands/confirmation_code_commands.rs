#[allow(unused_imports)]
use crate::enums::ConfirmationCodeAction;
#[allow(unused_imports)]
use crate::models::*;

#[cfg(feature = "confirm-confirmation-code")]
pub async fn confirm_confirmation_code<F, IF, T>(
    confirmation_code: &ConfirmationCode<'_>,
    action: ConfirmationCodeAction,
    code: &str,
    on_success: F,
) -> crate::utils::MutResult<T>
where
    F: Fn() -> IF,
    IF: std::future::IntoFuture<Output = crate::utils::MutResult<T>>,
{
    use crate::enums::{Input, InputError};
    use crate::utils::ValidatorTrait;

    let db_pool = crate::db_pool().await;
    let mut validator = crate::validator!();

    if action != confirmation_code.action {
        return crate::mut_error!();
    }

    if validator.validate_presence(Input::Code, code) {
        let code_is_valid = async {
            if confirmation_code.failed_attempts < 3
                && crate::utils::verify_password(code, &confirmation_code.encrypted_code)
            {
                return true;
            }

            let _ = sqlx::query!(
                "UPDATE confirmation_codes SET failed_attempts = failed_attempts + 1 WHERE id = $1",
                confirmation_code.id
            )
            .execute(db_pool)
            .await;

            false
        }
        .await;

        validator.custom_validation(Input::Code, InputError::IsInvalid, || code_is_valid);
    }

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let result = on_success().await;

    match result {
        Ok(success) => {
            let _ = delete_confirmation_code(confirmation_code).await;

            Ok(success)
        }
        Err(error) => Err(error),
    }
}

#[cfg(feature = "delete-confirmation-code")]
pub async fn delete_confirmation_code(confirmation_code: &ConfirmationCode<'_>) -> crate::utils::MutResult {
    let db_pool = crate::db_pool().await;

    sqlx::query!("DELETE FROM confirmation_codes WHERE id = $1", confirmation_code.id)
        .execute(db_pool)
        .await?;

    crate::mut_success!()
}

#[cfg(feature = "delete-all-expired-confirmation-codes")]
pub async fn delete_all_expired_confirmation_codes() -> crate::utils::MutResult {
    let db_pool = crate::db_pool().await;

    sqlx::query!("DELETE FROM confirmation_codes WHERE created_at < current_timestamp - INTERVAL '1 hour'")
        .execute(db_pool)
        .await?;

    crate::mut_success!()
}

#[cfg(feature = "get-confirmation-code-by-id")]
pub async fn get_confirmation_code_by_id<'a>(id: uuid::Uuid) -> sqlx::Result<ConfirmationCode<'a>> {
    let db_pool = crate::db_pool().await;

    sqlx::query_as!(
        ConfirmationCode,
        r#"SELECT
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at
            FROM confirmation_codes WHERE id = $1 LIMIT 1"#,
        id, // $1
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "get-confirmation-code-by-user")]
pub async fn get_confirmation_code_by_user(
    user: &User,
    action: ConfirmationCodeAction,
) -> sqlx::Result<ConfirmationCode<'_>> {
    let db_pool = crate::db_pool().await;

    sqlx::query_as!(
        ConfirmationCode,
        r#"SELECT
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at
            FROM confirmation_codes WHERE user_id = $1 AND action = $2 LIMIT 1"#,
        user.id,                          // $1
        action as ConfirmationCodeAction, // $2
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "insert-confirmation-code")]
pub async fn insert_confirmation_code(
    user: &User,
    action: ConfirmationCodeAction,
) -> crate::utils::MutResult<ConfirmationCode<'_>> {
    let db_pool = crate::db_pool().await;
    let jobs = crate::jobs().await;

    if let Ok(confirmation_code) = get_confirmation_code_by_user(user, action.clone()).await {
        return crate::mut_success!(confirmation_code);
    }

    let code = crate::utils::generate_random_string(crate::config::MISC_CONFIG.confirmation_code_length);

    let encrypted_code = crate::utils::encrypt_password(&code);

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
    .fetch_one(db_pool)
    .await;

    match result {
        Ok(confirmation_code) => {
            jobs.mailer(user, crate::enums::MailerJobCommand::ConfirmationCode { action, code })
                .await;

            crate::mut_success!(confirmation_code)
        }
        Err(_) => crate::mut_error!(),
    }
}
