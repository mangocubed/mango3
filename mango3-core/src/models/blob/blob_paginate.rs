use sqlx::query_as;

use mango3_utils::models::{CursorPage, CursorPageParams};

use crate::create_cursor_page;
use crate::models::{User, Website};
use crate::CoreContext;

use super::Blob;

impl Blob {
    pub async fn paginate_by_created_at_desc<'a>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        website: Option<&'a Website>,
        user: Option<&'a User>,
    ) -> CursorPage<Self> {
        create_cursor_page!(
            core_context,
            cursor_page_params,
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
                    r#"SELECT * FROM blobs
                    WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
                    ORDER BY created_at DESC, id DESC LIMIT $5"#,
                    website_id,        // $1
                    user_id,           // $2
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
    use crate::test_utils::{insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    use super::{Blob, CursorPageParams};

    #[tokio::test]
    async fn should_get_zero_blobs() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page =
            Blob::paginate_by_created_at_desc(&core_context, &CursorPageParams::default(), Some(&website), Some(&user))
                .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_blob() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page =
            Blob::paginate_by_created_at_desc(&core_context, &CursorPageParams::default(), Some(&website), Some(&user))
                .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
