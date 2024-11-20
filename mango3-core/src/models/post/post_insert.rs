use sqlx::query_as;

use crate::models::{Blob, User, Website};
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
        cover_image_blob: Option<&Blob>,
        publish: bool,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let title = title.trim();
        let slug = slug.trim().to_lowercase();
        let content = content.trim();
        let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

        validator.validate_title(title);
        validator.validate_slug(core_context, None, website, &slug).await;
        validator.validate_content(content);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "INSERT INTO posts (
                website_id, user_id, title, slug, content, cover_image_blob_id, published_at
            ) VALUES ($1, $2, $3, $4, $5, $6, CASE WHEN $7 IS TRUE THEN current_timestamp ELSE NULL END) RETURNING *",
            website.id,          // $1
            user.id,             // $2
            title,               // $3
            slug,                // $4
            content,             // $5
            cover_image_blob_id, // $6
            publish,             // $7
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
