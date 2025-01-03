use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};
use url::Url;

use crate::config::BASIC_CONFIG;
use crate::enums::{Input, InputError};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{Blob, User};

mod website_insert;
mod website_paginate;
mod website_search;
mod website_update;

#[derive(Clone)]
pub struct Website {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub description: String,
    pub icon_image_blob_id: Option<Uuid>,
    pub cover_image_blob_id: Option<Uuid>,
    pub light_theme: String,
    pub dark_theme: String,
    pub language: String,
    pub published_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Website {
    pub async fn cover_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None).await)
        } else {
            None
        }
    }

    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM websites WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub fn description_preview(&self) -> &str {
        self.description
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim())
            .unwrap_or_default()
    }

    pub async fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        user: Option<&User>,
        query: Option<&str>,
    ) -> sqlx::Result<Self> {
        let user_id = user.map(|user| user.id);
        query_as!(
            Self,
            r#"SELECT
                id,
                user_id,
                name,
                subdomain,
                description,
                icon_image_blob_id,
                cover_image_blob_id,
                light_theme,
                dark_theme,
                language::varchar AS "language!",
                published_at,
                CASE WHEN $3::varchar IS NOT NULL THEN ts_rank(search, websearch_to_tsquery($3)) ELSE NULL END AS search_rank,
                created_at,
                updated_at
            FROM websites WHERE id = $1 AND ($2::uuid IS NULL OR user_id = $2) LIMIT 1"#,
            id,      // $1
            user_id, // $2
            query,   // $3

        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_subdomain(core_context: &CoreContext, subdomain: &str) -> sqlx::Result<Self> {
        query_as!(
            Self,
            r#"SELECT
                id,
                user_id,
                name,
                subdomain,
                description,
                icon_image_blob_id,
                cover_image_blob_id,
                light_theme,
                dark_theme,
                language::varchar AS "language!",
                published_at,
                NULL::real AS search_rank,
                created_at,
                updated_at
            FROM websites WHERE subdomain = $1 AND published_at IS NOT NULL LIMIT 1"#,
            subdomain // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub fn host(&self) -> String {
        self.url().host().unwrap().to_string()
    }

    pub async fn icon_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.icon_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None).await)
        } else {
            None
        }
    }

    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    pub fn text_icon_url(&self) -> Url {
        BASIC_CONFIG.text_icon_url(&self.initials())
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.website_url(&self.subdomain)
    }
}

impl Validator {
    async fn validate_name(&mut self, core_context: &CoreContext, website: Option<&Website>, value: &str) -> bool {
        if self.validate_presence(Input::Name, value)
            && self.validate_length(Input::Name, value, Some(3), Some(255))
            && self.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(value).is_err())
        {
            let id = website.map(|w| w.id);
            let name_exists = query!(
                "SELECT id FROM websites WHERE ($1::uuid IS NULL OR id != $1) AND LOWER(name) = $2 LIMIT 1",
                id,                   // $1
                value.to_lowercase()  // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            self.custom_validation(Input::Name, InputError::AlreadyInUse, &|| !name_exists)
        } else {
            false
        }
    }

    fn validate_description(&mut self, value: &str) -> bool {
        self.validate_length(Input::Description, value, None, Some(1024))
    }
}
