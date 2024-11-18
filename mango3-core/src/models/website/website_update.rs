use sqlx::query_as;

use crate::models::Blob;
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn update(
        &self,
        core_context: &CoreContext,
        name: &str,
        description: &str,
        icon_image_blob: Option<&Blob>,
        cover_image_blob: Option<&Blob>,
        publish: bool,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let name = name.trim();
        let description = description.trim();
        let icon_image_blob_id = icon_image_blob.map(|blob| blob.id);
        let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

        validator.validate_name(core_context, Some(self.id), name).await;

        validator.validate_description(description);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "UPDATE websites SET
                name = $2,
                description = $3,
                icon_image_blob_id = $4,
                cover_image_blob_id = $5,
                published_at = CASE
                    WHEN $6 IS TRUE AND published_at IS NOT NULL THEN published_at
                    WHEN $6 IS TRUE THEN current_timestamp
                    ELSE NULL
                END
            WHERE id = $1 RETURNING *",
            self.id,             // $1
            name,                // $2
            description,         // $3
            icon_image_blob_id,  // $4
            cover_image_blob_id, // $5
            publish,             // $6
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
