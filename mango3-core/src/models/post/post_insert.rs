use sqlx::query_as;
use sqlx::types::uuid::Uuid;
use sqlx::types::JsonValue;

use crate::models::{Blob, Hashtag, User, Website};
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn insert(
        core_context: &CoreContext,
        website: &Website,
        user: &User,
        title: &str,
        slug: &str,
        content: &str,
        variables: &str,
        blobs: Vec<Blob>,
        cover_image_blob: Option<&Blob>,
        publish: bool,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let title = title.trim();
        let slug = slug.trim().to_lowercase();
        let content = content.trim();
        let variables = variables.parse::<JsonValue>().ok();
        let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

        let hashtags = Hashtag::get_or_insert_all(core_context, content).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();
        let blob_ids = blobs.iter().map(|blob| blob.id).collect::<Vec<Uuid>>();

        validator.validate_post_title(title);
        validator.validate_post_slug(core_context, None, website, &slug).await;
        validator.validate_post_content(content);
        validator.validate_post_variables(variables.as_ref());

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            r#"INSERT INTO posts (
                website_id,
                user_id,
                title,
                slug,
                content,
                variables,
                hashtag_ids,
                cover_image_blob_id,
                blob_ids,
                published_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CASE WHEN $10 IS TRUE THEN current_timestamp ELSE NULL END)
            RETURNING
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
                (SELECT COUNT(*) FROM post_views WHERE post_id = posts.id LIMIT 1) AS "views_count!",
                (SELECT COUNT(*) FROM post_comments WHERE post_id = posts.id LIMIT 1) AS "comments_count!",
                (SELECT COUNT(*) FROM post_reactions WHERE post_id = posts.id LIMIT 1) AS "reactions_count!",
                published_at,
                modified_at,
                NULL::real AS search_rank,
                created_at,
                updated_at"#,
            website.id,          // $1
            user.id,             // $2
            title,               // $3
            slug,                // $4
            content,             // $5
            variables.unwrap(),  // $6
            &hashtag_ids,        // $7
            cover_image_blob_id, // $8
            &blob_ids,           // $9
            publish,             // $10
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
