use sqlx::query_as;

use mango3_utils::models::{CursorPage, CursorPageParams};

use crate::create_cursor_page;
use crate::models::{Post, User};
use crate::CoreContext;

use super::PostComment;

impl PostComment {
    pub async fn paginate_by_created_at_desc<'a>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        post: Option<&'a Post>,
        user: Option<&'a User>,
    ) -> CursorPage<Self> {
        create_cursor_page!(
            core_context,
            cursor_page_params,
            |node: Self| node.id,
            move |core_context, after| async move { Self::get_by_id(core_context, after, user).await.ok() },
            move |core_context, cursor_resource, limit| async move {
                let post_id = post.map(|u| u.id);
                let user_id = user.map(|u| u.id);
                let (cursor_id, cursor_created_at) = cursor_resource
                    .map(|c| (Some(c.id), Some(c.created_at)))
                    .unwrap_or_default();

                query_as!(
                    Self,
                    r#"SELECT id, post_id, user_id, content, created_at, updated_at
                    FROM post_comments
                    WHERE ($1::uuid IS NULL OR post_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
                    ORDER BY created_at DESC, id DESC LIMIT $5"#,
                    post_id,           // $1
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
    use crate::test_utils::{insert_test_post, insert_test_post_comment, insert_test_user, setup_core_context};

    use super::{CursorPageParams, PostComment};

    #[tokio::test]
    async fn should_get_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let post = insert_test_post(&core_context, None, None).await;

        let cursor_page = PostComment::paginate_by_created_at_desc(
            &core_context,
            &CursorPageParams::default(),
            Some(&post),
            Some(&user),
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let post = insert_test_post(&core_context, None, None).await;

        insert_test_post_comment(&core_context, Some(&post), Some(&user)).await;

        let cursor_page = PostComment::paginate_by_created_at_desc(
            &core_context,
            &CursorPageParams::default(),
            Some(&post),
            Some(&user),
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
