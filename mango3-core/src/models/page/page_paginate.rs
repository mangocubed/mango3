use sqlx::query_as;

use crate::models::{User, Website};
use crate::pagination::{CursorPage, CursorPageParams};
use crate::CoreContext;

use super::Page;

impl Page {
    pub async fn paginate_by_created_at_desc<'a>(
        core_context: &'a CoreContext,
        page_params: &CursorPageParams,
        website: Option<&'a Website>,
        user: Option<&'a User>,
        is_published: Option<bool>,
    ) -> CursorPage<Self> {
        CursorPage::new(
            core_context,
            page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after, website, user).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let website_id = website.map(|w| w.id);
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), Some(c.created_at)))
                    .unwrap_or_default();

                query_as!(
                    Self,
                    "SELECT * FROM pages
                    WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND (
                            $3::bool IS NULL OR ($3 IS TRUE AND published_at IS NOT NULL)
                            OR ($3 IS FALSE AND published_at IS NULL)
                        ) AND ($5::timestamptz IS NULL OR created_at < $5 OR (created_at = $5 AND id < $4))
                    ORDER BY created_at DESC, id DESC LIMIT $6",
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
