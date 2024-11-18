use sqlx::{query, query_as};
use uuid::Uuid;

use crate::constants::{BLACKLISTED_SLUGS, REGEX_SLUG};
use crate::enums::{Input, InputError};
use crate::models::{Blob, User, Website};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
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

        if validator.validate_presence(Input::Title, title)
            && validator.validate_length(Input::Title, title, Some(1), Some(255))
        {
            validator.custom_validation(Input::Title, InputError::IsInvalid, &|| Uuid::try_parse(title).is_err());
        }

        if validator.validate_presence(Input::Slug, &slug)
            && validator.validate_format(Input::Slug, &slug, &REGEX_SLUG)
            && validator.validate_length(Input::Slug, &slug, Some(1), Some(255))
            && validator.custom_validation(Input::Slug, InputError::IsInvalid, &|| Uuid::try_parse(&slug).is_err())
            && validator.custom_validation(Input::Username, InputError::IsInvalid, &|| {
                !BLACKLISTED_SLUGS.contains(&slug)
            })
        {
            let slug_exists = query!(
                "SELECT id FROM posts WHERE LOWER(slug) = $1 AND website_id = $2 LIMIT 1",
                slug,       // $1
                website.id  // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Slug, InputError::AlreadyInUse, &|| !slug_exists);
        }

        validator.validate_length(Input::Content, content, None, Some(2048));

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
