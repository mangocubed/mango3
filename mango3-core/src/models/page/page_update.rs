use sqlx::query_as;

use crate::models::Blob;
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::Page;

impl Page {
    pub async fn update(
        &self,
        core_context: &CoreContext,
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

        validator.validate_page_title(title);
        validator
            .validate_page_slug(
                core_context,
                Some(self),
                &self.website(core_context).await.unwrap(),
                &slug,
            )
            .await;
        validator.validate_page_content(content);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "UPDATE pages SET
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
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
