use sqlx::query_as;

use crate::models::User;
use crate::pagination::{CursorPage, CursorPageParams};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn paginate_by_name_asc<'a>(
        core_context: &'a CoreContext,
        page_params: &CursorPageParams,
        user: Option<&'a User>,
        is_published: Option<bool>,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after, user, None).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let user_id = user.map(|u| u.id);
                let cursor_name = cursor_resource.map(|c| c.name.clone());

                query_as!(
                    Self,
                    r#"SELECT
                        id,
                        user_id,
                        name,
                        subdomain,
                        description,
                        icon_image_blob_id,
                        cover_image_blob_id,
                        light_theme,
                        dark_theme,
                        language::varchar AS "language!",
                        published_at,
                        NULL::real AS search_rank,
                        created_at,
                        updated_at
                    FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                        AND (
                            $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                            OR ($2 IS FALSE AND published_at IS NULL)
                        ) AND ($3::text IS NULL OR name > $3)
                    ORDER BY name ASC LIMIT $4"#,
                    user_id,      // $1
                    is_published, // $2
                    cursor_name,  // $3
                    limit,        // $4
                )
                .fetch_all(&core_context.db_pool)
                .await
                .unwrap_or_default()
            },
        )
        .await
    }

    pub async fn paginate_by_created_at_desc<'a>(
        core_context: &'a CoreContext,
        page_params: &CursorPageParams,
        user: Option<&'a User>,
        is_published: Option<bool>,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after, user, None).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), Some(c.created_at)))
                    .unwrap_or_default();

                query_as!(
                    Self,
                    r#"SELECT
                        id,
                        user_id,
                        name,
                        subdomain,
                        description,
                        icon_image_blob_id,
                        cover_image_blob_id,
                        light_theme,
                        dark_theme,
                        language::varchar AS "language!",
                        published_at,
                        NULL::real AS search_rank,
                        created_at,
                        updated_at
                    FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                        AND (
                            $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                            OR ($2 IS FALSE AND published_at IS NULL)
                        ) AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
                    ORDER BY created_at DESC, id DESC LIMIT $5"#,
                    user_id,           // $1
                    is_published,      // $2
                    cursor_id,         // $3
                    cursor_created_at, // $4
                    limit,             // $5
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
    use crate::test_utils::{insert_test_user, insert_test_website, setup_core_context};

    use super::Website;

    #[tokio::test]
    async fn should_get_zero_websites_sorted_by_created_at_desc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page =
            Website::paginate_by_created_at_desc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website_sorted_by_created_at_desc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        insert_test_website(&core_context, Some(&user)).await;

        let cursor_page =
            Website::paginate_by_created_at_desc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }

    #[tokio::test]
    async fn should_get_zero_websites_sorted_by_name_asc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page =
            Website::paginate_by_name_asc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website_sorted_by_name_asc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        insert_test_website(&core_context, Some(&user)).await;

        let cursor_page =
            Website::paginate_by_name_asc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
