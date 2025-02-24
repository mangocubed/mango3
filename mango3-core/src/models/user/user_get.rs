use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::Uuid;

use crate::constants::{PREFIX_GET_USER_BY_ID, PREFIX_GET_USER_BY_USERNAME, PREFIX_GET_USER_BY_USERNAME_OR_EMAIL};
use crate::enums::UserRole;
use crate::models::async_redis_cache;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        get_user_by_id(core_context, id).await
    }

    pub async fn get_by_username(core_context: &CoreContext, username: &str) -> sqlx::Result<Self> {
        get_user_by_username(core_context, username).await
    }

    pub async fn get_by_username_or_email(core_context: &CoreContext, username_or_email: &str) -> sqlx::Result<Self> {
        get_user_by_username_or_email(core_context, username_or_email).await
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, User>",
    create = r##" { async_redis_cache(PREFIX_GET_USER_BY_ID).await } "##
)]
pub(crate) async fn get_user_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<User> {
    query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, User>",
    create = r##" { async_redis_cache(PREFIX_GET_USER_BY_USERNAME).await } "##
)]
pub(crate) async fn get_user_by_username(core_context: &CoreContext, username: &str) -> sqlx::Result<User> {
    if username.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users WHERE LOWER(username) = $1 LIMIT 1"#,
        username.to_lowercase()
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username_or_email.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, User>",
    create = r##" { async_redis_cache(PREFIX_GET_USER_BY_USERNAME_OR_EMAIL).await } "##
)]
pub(crate) async fn get_user_by_username_or_email(
    core_context: &CoreContext,
    username_or_email: &str,
) -> sqlx::Result<User> {
    if username_or_email.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users
        WHERE
            disabled_at IS NULL
            AND (LOWER(username) = $1 OR (email_confirmed_at IS NOT NULL AND LOWER(email) = $1))
        LIMIT 1"#,
        username_or_email.to_lowercase()
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_username, fake_uuid, insert_test_user, setup_core_context};

    use super::User;

    #[tokio::test]
    async fn should_get_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = User::get_by_id(&core_context, user.id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_id_when_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = User::get_by_id(&core_context, id).await;

        assert!(result.is_err());
    }

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

    #[tokio::test]
    async fn should_get_by_username_or_email() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = User::get_by_username_or_email(&core_context, &user.username).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_email_when_is_unverified() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = User::get_by_username_or_email(&core_context, &user.email).await;

        assert!(result.is_err());
    }
}
