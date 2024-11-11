use sqlx::query_as;

use crate::models::{Page, PageParams, User};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn paginate_by_name_asc(
        core_context: &CoreContext,
        page_params: &PageParams,
        user: Option<&User>,
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
            "SELECT * FROM websites WHERE ($1::uuid IS NULL OR user_id = $1) AND ($2::text IS NULL OR name > $2)
            ORDER BY name ASC LIMIT $3",
            user_id,
            cursor_name,
            first as i64
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
                AND ($3::timestamptz IS NULL OR created_at < $3 OR (created_at = $3 AND id < $2))
            ORDER BY created_at DESC, id DESC LIMIT $4",
            user_id,           // $1
            cursor_id,         // $2
            cursor_created_at, // $3
            first as i64,      // $4
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
