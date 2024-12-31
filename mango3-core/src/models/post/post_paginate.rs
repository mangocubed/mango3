use sqlx::query_as;

use crate::models::{User, Website};
use crate::pagination::{CursorPage, CursorPageParams};
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn paginate_by_created_at_desc<'a>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        website: Option<&'a Website>,
        user: Option<&'a User>,
        is_published: Option<bool>,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            cursor_page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after, website, user, None).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let website_id = website.map(|w| w.id);
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), Some(c.created_at)))
                    .unwrap_or_default();

                query_as!(
                    Self,
                    r#"SELECT
                        id,
                        website_id,
                        user_id,
                        language::varchar as "language!",
                        title,
                        slug,
                        content,
                        cover_image_blob_id,
                        published_at,
                        NULL::real AS search_rank,
                        created_at,
                        updated_at
                    FROM posts
                    WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND (
                            $3::bool IS NULL OR ($3 IS TRUE AND published_at IS NOT NULL)
                            OR ($3 IS FALSE AND published_at IS NULL)
                        ) AND ($5::timestamptz IS NULL OR created_at < $5 OR (created_at = $5 AND id < $4))
                    ORDER BY created_at DESC, id DESC LIMIT $6"#,
                    website_id,        // $1
                    user_id,           // $2
                    is_published,      // $3
                    cursor_id,         // $4
                    cursor_created_at, // $5
                    limit,             // $6
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
    use crate::test_utils::{insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    use super::Post;

    #[tokio::test]
    async fn should_get_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = Post::paginate_by_created_at_desc(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page = Post::paginate_by_created_at_desc(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
