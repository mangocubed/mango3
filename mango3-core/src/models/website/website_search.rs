use sqlx::query_as;

use crate::models::User;
use crate::pagination::{CursorPage, CursorPageParams};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn search<'a>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        user: Option<&'a User>,
        is_published: Option<bool>,
        query: &'a str,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            cursor_page_params,
            |node: Self| node.id,
            move |core_context, after| async move {
                Self::get_by_id(core_context, after, user, Some(query))
                    .await
                    .ok()
            },
            move |core_context, cursor_resource, limit| async move {
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_search_rank, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), c.search_rank, Some(c.created_at)))
                    .unwrap_or_default();

                query_as!(
                    Self,
                    r#"SELECT
                        id,
                        user_id,
                        name,
                        subdomain,
                        description,
                        hashtag_ids,
                        icon_image_blob_id,
                        cover_image_blob_id,
                        light_theme,
                        dark_theme,
                        language::varchar as "language!",
                        published_at,
                        ts_rank(search, websearch_to_tsquery($3)) AS search_rank,
                        created_at,
                        updated_at
                    FROM websites
                    WHERE ($1::uuid IS NULL OR user_id = $1)
                        AND (
                            $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                            OR ($2 IS FALSE AND published_at IS NULL)
                        ) AND (
                            search @@ websearch_to_tsquery($3)
                            OR name ILIKE '%' || $3 || '%'
                            OR subdomain ILIKE '%' || $3 || '%'
                            OR description ILIKE '%' || $3 || '%'
                        ) AND (
                            ($4::uuid IS NULL OR $5::real IS NULL OR $6::timestamptz IS NULL)
                            OR ts_rank(search, websearch_to_tsquery($3)) < $5 OR (
                                ts_rank(search, websearch_to_tsquery($3)) = $5 AND (
                                    created_at < $6 OR (created_at = $6 AND id < $4)
                                )
                            )
                        )
                    ORDER BY search_rank DESC, created_at DESC, id DESC LIMIT $7"#,
                    user_id,            // $1
                    is_published,       // $2
                    query,              // $3
                    cursor_id,          // $4
                    cursor_search_rank, // $5
                    cursor_created_at,  // $6
                    limit,              // $7
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
    async fn should_get_zero_websites() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page = Website::search(&core_context, &CursorPageParams::default(), Some(&user), None, "").await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = Website::search(
            &core_context,
            &CursorPageParams::default(),
            Some(&user),
            None,
            &website.name,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
