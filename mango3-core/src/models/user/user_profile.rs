use sqlx::query_as;
use sqlx::types::uuid::Uuid;

use crate::enums::{Input, UserRole};
use crate::models::{find_country, parse_date, Blob, Hashtag};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn update_profile(
        &self,
        core_context: &CoreContext,
        display_name: &str,
        full_name: &str,
        birthdate: &str,
        country_alpha2: &str,
        bio: &str,
        avatar_image_blob: Option<&Blob>,
    ) -> Result<User, ValidationErrors> {
        let mut validator = Validator::default();

        let display_name = display_name.trim();
        let full_name = full_name.trim();
        let birthdate = parse_date(birthdate);
        let country = find_country(country_alpha2);
        let bio = bio.trim();
        let avatar_image_blob_id = avatar_image_blob.map(|blob| blob.id);

        if validator.validate_presence(Input::DisplayName, display_name) {
            validator.validate_length(Input::DisplayName, display_name, Some(2), Some(256));
        }

        validator.validate_full_name(full_name);

        validator.validate_birthdate(birthdate);

        validator.validate_country(country);

        validator.validate_length(Input::Bio, bio, None, Some(1024));

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let hashtags = Hashtag::get_or_insert_all(core_context, bio).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

        let result = query_as!(
            User,
            r#"UPDATE users
            SET
                display_name = $2,
                full_name = $3,
                birthdate = $4,
                country_alpha2 = $5,
                bio = $6,
                hashtag_ids = $7,
                avatar_image_blob_id = $8
            WHERE disabled_at IS NULL AND id = $1 RETURNING
                id,
                username,
                email,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                disabled_at,
                created_at,
                updated_at"#,
            self.id,                 // $1
            display_name,            // $2
            full_name,               // $3
            birthdate,               // $4
            country.unwrap().alpha2, // $5
            bio,                     // $6
            &hashtag_ids,            // $7
            avatar_image_blob_id,    // $8
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                user.cache_remove().await;

                Ok(user)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
