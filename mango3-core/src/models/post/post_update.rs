use sqlx::query_as;

use crate::models::{Blob, PostAttachment};
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
            "UPDATE posts SET
                title = $2,
                slug = $3,
                content = $4,
                cover_image_blob_id = $5,
                published_at = CASE
                    WHEN $6 IS TRUE AND published_at IS NOT NULL THEN published_at
                    WHEN $6 IS TRUE THEN current_timestamp
                    ELSE NULL
                END
            WHERE id = $1 RETURNING *",
            self.id,             // $1
            title,               // $2
            slug,                // $3
            content,             // $4
            cover_image_blob_id, // $5
            publish,             // $6
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
