use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::constants::{BLACKLISTED_SUBDOMAINS, REGEX_SUBDOMAIN};
use crate::enums::{Input, InputError};
use crate::models::User;
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

        if validator.validate_presence(Input::Name, name)
            && validator.validate_length(Input::Name, name, Some(3), Some(255))
            && validator.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(name).is_err())
        {
            let name_exists = query!(
                "SELECT id FROM websites WHERE LOWER(name) = $1 LIMIT 1",
                name.to_lowercase() // $1
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Name, InputError::AlreadyInUse, &|| !name_exists);
        }

        if validator.validate_presence(Input::Subdomain, &subdomain)
            && validator.validate_format(Input::Subdomain, &subdomain, &REGEX_SUBDOMAIN)
            && validator.validate_length(Input::Subdomain, &subdomain, Some(3), Some(255))
            && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
                Uuid::try_parse(&subdomain).is_err()
            })
            && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
                !BLACKLISTED_SUBDOMAINS.contains(&subdomain)
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

        validator.validate_length(Input::Description, description, None, Some(1024));

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "INSERT INTO websites (user_id, name, subdomain, description) VALUES ($1, $2, $3, $4) RETURNING *",
            user.id,     // $1
            name,        // $2
            subdomain,   // $3
            description, // $4
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
