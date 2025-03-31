use sqlx::query_as;

use crate::enums::UserRole;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn all_admins(core_context: &CoreContext) -> Vec<Self> {
        query_as!(
            Self,
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
            FROM users WHERE role IN ('admin', 'superuser')"#,
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
    }
}
