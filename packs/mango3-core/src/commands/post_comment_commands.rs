use crate::models::*;
use crate::CoreContext;

#[cfg(feature = "delete-post-comment")]
pub async fn delete_post_comment(core_context: &CoreContext, post_comment: &PostComment) -> MutResult {
    sqlx::query!("DELETE FROM post_comments WHERE id = $1", post_comment.id)
        .execute(&core_context.db_pool)
        .await?;

    crate::modesl::POST_COMMENT_CONTENT_HTML
        .cache_remove(crate::constants::PREFIX_POST_COMMENT_CONTENT_HTML, post_comment.id)
        .await;

    Ok(())
}

#[cfg(feature = "get-post-comment-by-id")]
pub async fn get_post_comment_by_id(
    core_context: &CoreContext,
    id: Uuid,
    user: Option<&User>,
) -> sqlx::Result<PostComment> {
    let user_id = user.map(|user| user.id);

    sqlx::query_as!(
        PostComment,
        r#"SELECT id, post_id, user_id, content, created_at, updated_at
        FROM post_comments WHERE id = $1 AND ($2::uuid IS NULL OR user_id = $2) LIMIT 1"#,
        id,      // $1
        user_id, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-post-comments-count")]
pub async fn get_post_comments_count(core_context: &CoreContext, post: &Post) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) FROM post_comments WHERE post_id = $1 LIMIT 1",
        post.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map(|record| record.count.unwrap_or_default())
    .unwrap_or_default()
}

#[cfg(feature = "insert-post-comment")]
pub async fn insert_post_comment(
    core_context: &CoreContext,
    post: &Post,
    user: &User,
    content: &str,
) -> MutResult<PostComment> {
    let mut validator = crate::validator!();

    let content = content.trim();

    if validator.validate_presence(Input::Content, content) {
        validator.validate_length(
            Input::Content,
            content,
            Some(1),
            Some(MISC_CONFIG.max_comment_content_length),
        );
    }

    if !validator.is_valid {
        return Err(validator.errors);
    }

    sqlx::query_as!(
        PostComment,
        "INSERT INTO post_comments (post_id, user_id, content) VALUES ($1, $2, $3) RETURNING
            id, post_id, user_id, content, created_at, updated_at",
        post.id, // $1
        user.id, // $2
        content, // $3
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "paginate-post-comments")]
pub async fn paginate_post_commentc<'a>(
    core_context: &'a CoreContext,
    cursor_page_params: &CursorPageParams,
    post: Option<&'a Post>,
    user: Option<&'a User>,
) -> CursorPage<Self> {
    crate::cursor_page!(
        core_context,
        cursor_page_params,
        |node: Self| node.id,
        move |core_context, after| async move { get_post_comment_by_id(core_context, after, user).await.ok() },
        move |core_context, cursor_resource, limit| async move {
            let post_id = post.map(|u| u.id);
            let user_id = user.map(|u| u.id);
            let (cursor_id, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), Some(c.created_at)))
                .unwrap_or_default();

            query_as!(
                PostComment,
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_paragraph, insert_test_post, insert_test_user, setup_core_context};

    use super::PostComment;

    #[tokio::test]
    async fn should_count_post_comments() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = get_post_comments_count(&core_context, &post).await;

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_insert_post_comment() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;
        let content = fake_paragraph();

        let result = insert_post_comment(&core_context, &post, &user, &content).await;

        assert!(result.is_ok());
    }

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
