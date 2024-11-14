use sqlx::query_as;

use crate::models::{Page, PageParams, User};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn paginate_by_name_asc(
        core_context: &CoreContext,
        page_params: &PageParams,
        user: Option<&User>,
        is_published: Option<bool>,
    ) -> Page<Self> {
        let after = page_params.after;
        let first = page_params.first;
        let user_id = user.map(|u| u.id);

        let cursor = if let Some(id) = after {
            Self::get_by_id(core_context, id, user).await.ok()
        } else {
            None
        };

        let cursor_name = cursor.as_ref().map(|c| c.name.clone());

        let mut nodes = query_as!(
            Self,
            "SELECT * FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                AND (
                    $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                    OR ($2 IS FALSE AND published_at IS NULL)
                ) AND ($3::text IS NULL OR name > $3)
            ORDER BY name ASC LIMIT $4",
            user_id,      // $1
            is_published, // $2
            cursor_name,  // $3
            first as i64, // $4
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default();

        let has_next_page = if nodes.len() > first as usize {
            nodes.remove(nodes.len() - 1);

            true
        } else {
            false
        };

        Page { nodes, has_next_page }
    }

    pub async fn paginate_by_created_at_desc(
        core_context: &CoreContext,
        page_params: &PageParams,
        user: Option<&User>,
        is_published: Option<bool>,
    ) -> Page<Self> {
        let after = page_params.after;
        let first = page_params.first;
        let user_id = user.map(|u| u.id);

        let cursor = if let Some(id) = after {
            Self::get_by_id(core_context, id, user).await.ok()
        } else {
            None
        };

        let (cursor_id, cursor_created_at) = cursor
            .as_ref()
            .map(|c| (Some(c.id), Some(c.created_at)))
            .unwrap_or_default();

        let mut nodes = query_as!(
            Self,
            "SELECT * FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                AND (
                    $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                    OR ($2 IS FALSE AND published_at IS NULL)
                ) AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
            ORDER BY created_at DESC, id DESC LIMIT $5",
            user_id,           // $1
            is_published,      // $2
            cursor_id,         // $3
            cursor_created_at, // $4
            first as i64,      // $5
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default();

        let has_next_page = if nodes.len() > first as usize {
            nodes.remove(nodes.len() - 1);

            true
        } else {
            false
        };

        Page { nodes, has_next_page }
    }
}
