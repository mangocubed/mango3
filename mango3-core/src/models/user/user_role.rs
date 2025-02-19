use sqlx::{query, query_as};

use crate::enums::UserRole;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn update_role(&self, core_context: &CoreContext, role: UserRole) -> Result<Self, ValidationErrors> {
        if role == UserRole::Superuser {
            let _ = query!(
                r#"UPDATE users SET role = 'admin' WHERE role = 'superuser' AND id != $1"#,
                self.id
            )
            .execute(&core_context.db_pool)
            .await;
        }

        let result = query_as!(
            Self,
            r#"UPDATE users SET role = $2 WHERE id = $1 RETURNING
                id,
                username,
                email,
                email_confirmation_code_id,
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
                updated_at"#,
            self.id,          // $1
            role as UserRole, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                self.cache_remove().await;

                Ok(user)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_user, setup_core_context};

    use super::UserRole;

    #[tokio::test]
    async fn should_update_user_role() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = user.update_role(&core_context, UserRole::Admin).await;

        assert!(result.is_ok());

        let user = result.unwrap();

        assert_eq!(user.role, UserRole::Admin);
    }
}
