use sqlx::query_as;

use crate::enums::UserRole;
use crate::pagination::{CursorPage, CursorPageParams};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn paginate_by_username_asc(
        core_context: &CoreContext,
        cursor_page_params: &CursorPageParams,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            cursor_page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let cursor_username = cursor_resource.map(|c| c.username);

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
                    FROM users WHERE $1::citext IS NULL OR username > $1 ORDER BY username ASC LIMIT $2"#,
                    cursor_username, // $1
                    limit,           // $2
                )
                .fetch_all(&core_context.db_pool)
                .await
                .unwrap_or_default()
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::pagination::CursorPageParams;
    use crate::test_utils::{insert_test_user, setup_core_context};

    use super::User;

    #[tokio::test]
    async fn should_get_some_users() {
        let core_context = setup_core_context().await;

        insert_test_user(&core_context).await;

        let cursor_page = User::paginate_by_username_asc(&core_context, &CursorPageParams::default()).await;

        assert!(!cursor_page.nodes.is_empty());
    }
}
