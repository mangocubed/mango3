use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::constants::{BLACKLISTED_SLUGS, REGEX_SUBDOMAIN};
use crate::enums::{Input, InputError};
use crate::models::{Hashtag, User};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn insert(
        core_context: &CoreContext,
        user: &User,
        name: &str,
        subdomain: &str,
        description: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let name = name.trim();
        let subdomain = subdomain.trim().to_lowercase();
        let description = description.trim();

        validator.validate_name(core_context, None, name).await;

        if validator.validate_presence(Input::Subdomain, &subdomain)
            && validator.validate_format(Input::Subdomain, &subdomain, &REGEX_SUBDOMAIN)
            && validator.validate_length(Input::Subdomain, &subdomain, Some(3), Some(256))
            && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
                Uuid::try_parse(&subdomain).is_err()
            })
            && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
                !BLACKLISTED_SLUGS.contains(&subdomain.as_str())
            })
        {
            let subdomain_exists = query!(
                "SELECT id FROM websites WHERE LOWER(subdomain) = $1 LIMIT 1",
                subdomain // $1
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Subdomain, InputError::AlreadyInUse, &|| !subdomain_exists);
        }

        validator.validate_description(description);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let hashtags = Hashtag::get_or_insert_all(core_context, description).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

        query_as!(
            Self,
            r#"INSERT INTO websites (user_id, name, subdomain, description, hashtag_ids) VALUES ($1, $2, $3, $4, $5)
            RETURNING
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
            user.id,      // $1
            name,         // $2
            subdomain,    // $3
            description,  // $4
            &hashtag_ids, // $5
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
