use sqlx::query_as;

use mango3_utils::models::{CursorPage, CursorPageParams};

use crate::create_cursor_page;
use crate::models::{User, Website};
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn search<'a>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        website: Option<&'a Website>,
        user: Option<&'a User>,
        is_published: Option<bool>,
        query: &'a str,
    ) -> CursorPage<Self> {
        create_cursor_page!(
            core_context,
            cursor_page_params,
            |node: Self| node.id,
            move |core_context, after| async move {
                Self::get_by_id_with_search_rank(core_context, after, website, user, is_published, query)
                    .await
                    .ok()
            },
            move |core_context, cursor_resource, limit| async move {
                let website_id = website.map(|w| w.id);
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_search_rank, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), c.search_rank, Some(c.created_at)))
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
                        variables,
                        hashtag_ids,
                        cover_image_blob_id,
                        blob_ids,
                        published_at,
                        modified_at,
                        ts_rank(search, websearch_to_tsquery($4)) AS search_rank,
                        created_at,
                        updated_at
                    FROM posts
                    WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND (
                            $3::bool IS NULL OR ($3 IS TRUE AND published_at IS NOT NULL)
                            OR ($3 IS FALSE AND published_at IS NULL)
                        ) AND (
                            search @@ websearch_to_tsquery($4)
                            OR title ILIKE '%' || $4 || '%'
                            OR slug ILIKE '%' || $4 || '%'
                            OR content ILIKE '%' || $4 || '%'
                        ) AND (
                            ($5::uuid IS NULL OR $6::real IS NULL OR $7::timestamptz IS NULL)
                            OR ts_rank(search, websearch_to_tsquery($4)) < $6 OR (
                                ts_rank(search, websearch_to_tsquery($4)) = $6 AND (
                                    created_at < $7 OR (created_at = $7 AND id < $5)
                                )
                            )
                        )
                    ORDER BY search_rank DESC, created_at DESC, id DESC LIMIT $8"#,
                    website_id,         // $1
                    user_id,            // $2
                    is_published,       // $3
                    query,              // $4
                    cursor_id,          // $5
                    cursor_search_rank, // $6
                    cursor_created_at,  // $7
                    limit,              // $8
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
    use crate::test_utils::{insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    use super::{CursorPageParams, Post};

    #[tokio::test]
    async fn should_get_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = Post::search(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            "",
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page = Post::search(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            &post.title,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
