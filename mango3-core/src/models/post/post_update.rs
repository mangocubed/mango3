use sqlx::query_as;
use sqlx::types::uuid::Uuid;

use crate::models::{Blob, Hashtag, PostAttachment};
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn update(
        &self,
        core_context: &CoreContext,
        title: &str,
        slug: &str,
        content: &str,
        blobs: Vec<Blob>,
        cover_image_blob: Option<&Blob>,
        publish: bool,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let title = title.trim();
        let slug = slug.trim().to_lowercase();
        let content = content.trim();
        let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

        let hashtags = Hashtag::get_or_insert_all(core_context, content).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

        validator.validate_post_title(title);
        validator
            .validate_post_slug(
                core_context,
                Some(self),
                &self.website(core_context).await.unwrap(),
                &slug,
            )
            .await;
        validator.validate_post_content(content);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let result = query_as!(
            Self,
            r#"UPDATE posts SET
                title = $2,
                slug = $3,
                content = $4,
                hashtag_ids = $5,
                cover_image_blob_id = $6,
                published_at = CASE
                    WHEN $7 IS TRUE AND published_at IS NOT NULL THEN published_at
                    WHEN $7 IS TRUE THEN current_timestamp
                    ELSE NULL
                END
            WHERE id = $1
            RETURNING
                id,
                website_id,
                user_id,
                language::varchar as "language!",
                title,
                slug,
                content,
                hashtag_ids,
                cover_image_blob_id,
                published_at,
                NULL::real AS search_rank,
                created_at,
                updated_at"#,
            self.id,             // $1
            title,               // $2
            slug,                // $3
            content,             // $4
            &hashtag_ids,        // $5
            cover_image_blob_id, // $6
            publish,             // $7
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(post) => {
                let _ = PostAttachment::save_all(core_context, &post, blobs).await;

                Ok(post)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
