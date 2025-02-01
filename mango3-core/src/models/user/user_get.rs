use sqlx::query_as;
use sqlx::types::Uuid;

use crate::enums::UserRole;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        query_as!(
            Self,
            r#"SELECT
                    id,
                    username,
                    email,
                    email_confirmation_code_id,
                    email_confirmed_at,
                    encrypted_password,
                    password_reset_confirmation_code_id,
                    display_name,
                    full_name,
                    birthdate,
                    language_code,
                    country_alpha2,
                    bio,
                    hashtag_ids,
                    avatar_image_blob_id,
                    role as "role!: UserRole",
                    created_at,
                    updated_at
                FROM users WHERE locked_at IS NULL AND id = $1 LIMIT 1"#,
            id
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_username(core_context: &CoreContext, username: &str) -> sqlx::Result<Self> {
        if username.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        query_as!(
            Self,
            r#"SELECT
                    id,
                    username,
                    email,
                    email_confirmation_code_id,
                    email_confirmed_at,
                    encrypted_password,
                    password_reset_confirmation_code_id,
                    display_name,
                    full_name,
                    birthdate,
                    language_code,
                    country_alpha2,
                    bio,
                    hashtag_ids,
                    avatar_image_blob_id,
                    role as "role!: UserRole",
                    created_at,
                    updated_at
                FROM users WHERE locked_at IS NULL AND LOWER(username) = $1 LIMIT 1"#,
            username.to_lowercase()
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_username_or_email(core_context: &CoreContext, username_or_email: &str) -> sqlx::Result<Self> {
        if username_or_email.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        query_as!(
            Self,
            r#"SELECT
                    id,
                    username,
                    email,
                    email_confirmation_code_id,
                    email_confirmed_at,
                    encrypted_password,
                    password_reset_confirmation_code_id,
                    display_name,
                    full_name,
                    birthdate,
                    language_code,
                    country_alpha2,
                    bio,
                    hashtag_ids,
                    avatar_image_blob_id,
                    role as "role!: UserRole",
                    created_at,
                    updated_at
                FROM users
                WHERE
                    locked_at IS NULL
                    AND (LOWER(username) = $1 OR (email_confirmed_at IS NOT NULL AND LOWER(email) = $1))
                LIMIT 1"#,
            username_or_email.to_lowercase()
        )
        .fetch_one(&core_context.db_pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_username, insert_test_user, setup_core_context};

    use super::User;

    #[tokio::test]
    async fn should_get_by_username() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = User::get_by_username(&core_context, &user.username).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_username_when_is_invalid() {
        let core_context = setup_core_context().await;
        let username = fake_username();

        let result = User::get_by_username(&core_context, &username).await;

        assert!(result.is_err());
    }
}
