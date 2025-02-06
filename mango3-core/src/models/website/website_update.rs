use sqlx::query_as;
use sqlx::types::uuid::Uuid;

use crate::constants::{DARK_THEMES, LIGHT_THEMES};
use crate::enums::{Input, InputError};
use crate::models::{Blob, Hashtag};
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
        light_theme: &str,
        dark_theme: &str,
        publish: bool,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let name = name.trim();
        let description = description.trim();
        let icon_image_blob_id = icon_image_blob.map(|blob| blob.id);
        let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);
        let light_theme = light_theme.trim();
        let dark_theme = dark_theme.trim();

        validator.validate_name(core_context, Some(self), name).await;
        validator.validate_description(description);
        validator.custom_validation(Input::LightTheme, InputError::IsInvalid, &|| {
            LIGHT_THEMES.contains(&light_theme)
        });
        validator.custom_validation(Input::DarkTheme, InputError::IsInvalid, &|| {
            DARK_THEMES.contains(&dark_theme)
        });

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let hashtags = Hashtag::get_or_insert_all(core_context, description).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

        let result = query_as!(
            Self,
            r#"UPDATE websites SET
                name = $2,
                description = $3,
                hashtag_ids = $4,
                icon_image_blob_id = $5,
                cover_image_blob_id = $6,
                light_theme = $7,
                dark_theme = $8,
                published_at = CASE
                    WHEN $9 IS TRUE AND published_at IS NOT NULL THEN published_at
                    WHEN $9 IS TRUE THEN current_timestamp
                    ELSE NULL
                END
            WHERE id = $1 RETURNING
                id,
                user_id,
                name,
                subdomain,
                description,
                hashtag_ids,
                icon_image_blob_id,
                cover_image_blob_id,
                light_theme,
                dark_theme,
                language::varchar AS "language!",
                published_at,
                NULL::real AS search_rank,
                created_at,
                updated_at"#,
            self.id,             // $1
            name,                // $2
            description,         // $3
            &hashtag_ids,        // $4
            icon_image_blob_id,  // $5
            cover_image_blob_id, // $6
            light_theme,         // $7
            dark_theme,          // $8
            publish,             // $9
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(website) => {
                website.cache_remove();

                Ok(website)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_name, fake_sentence, insert_test_website, setup_core_context};

    #[tokio::test]
    async fn should_update_website() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let name = fake_name();
        let description = fake_sentence();

        let result = website
            .update(&core_context, &name, &description, None, None, "light", "dark", true)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_website_when_fields_are_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = website.update(&core_context, "", "", None, None, "", "", true).await;

        assert!(result.is_err());
    }
}
