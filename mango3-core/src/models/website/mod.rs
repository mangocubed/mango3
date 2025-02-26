use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

#[cfg(feature = "website_cache_remove")]
use futures::future;

use crate::config::BASIC_CONFIG;
use crate::CoreContext;

#[cfg(feature = "website_cache_remove")]
use crate::constants::{
    PREFIX_GET_WEBSITE_BY_ID, PREFIX_GET_WEBSITE_BY_SUBDOMAIN, PREFIX_WEBSITE_DESCRIPTION_HTML,
    PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML,
};
#[cfg(feature = "website_write")]
use crate::enums::{Input, InputError};
#[cfg(feature = "website_write")]
use crate::validator::{Validator, ValidatorTrait};

use super::{Blob, Hashtag, User};

#[cfg(feature = "website_cache_remove")]
use super::AsyncRedisCacheTrait;

mod website_get;
mod website_paginate;
mod website_search;

#[cfg(feature = "website_write")]
mod website_delete;
#[cfg(any(feature = "website_description_html", feature = "website_description_preview_html"))]
mod website_description;
#[cfg(feature = "website_write")]
mod website_insert;
#[cfg(feature = "website_write")]
mod website_update;

#[cfg(feature = "website_cache_remove")]
use website_description::{WEBSITE_DESCRIPTION_HTML, WEBSITE_DESCRIPTION_PREVIEW_HTML};
#[cfg(feature = "website_cache_remove")]
use website_get::{GET_WEBSITE_BY_ID, GET_WEBSITE_BY_SUBDOMAIN};

#[derive(Clone, Deserialize, Serialize)]
pub struct Website {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub description: String,
    pub hashtag_ids: Vec<Uuid>,
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

impl Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Website {
    #[cfg(feature = "website_cache_remove")]
    async fn cache_remove(&self) {
        future::join4(
            WEBSITE_DESCRIPTION_HTML.cache_remove(PREFIX_WEBSITE_DESCRIPTION_HTML, &self.id),
            WEBSITE_DESCRIPTION_PREVIEW_HTML.cache_remove(PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML, &self.id),
            GET_WEBSITE_BY_ID.cache_remove(PREFIX_GET_WEBSITE_BY_ID, &self.id),
            GET_WEBSITE_BY_SUBDOMAIN.cache_remove(PREFIX_GET_WEBSITE_BY_SUBDOMAIN, &self.subdomain.to_lowercase()),
        )
        .await;
    }

    pub async fn count(core_context: &CoreContext, user: Option<&User>) -> sqlx::Result<i64> {
        let user_id = user.map(|u| u.id);

        query!(
            "SELECT COUNT(*) FROM websites WHERE $1::uuid IS NULL OR user_id = $1 LIMIT 1",
            user_id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
    }

    pub async fn cover_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None, None).await)
        } else {
            None
        }
    }

    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        Hashtag::all_by_ids(core_context, &self.hashtag_ids).await
    }

    pub fn host(&self) -> String {
        self.url().host().unwrap().to_string()
    }

    pub async fn icon_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.icon_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None, None).await)
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

#[cfg(feature = "website_write")]
impl Validator {
    async fn validate_name(&mut self, core_context: &CoreContext, website: Option<&Website>, value: &str) -> bool {
        if self.validate_presence(Input::Name, value)
            && self.validate_length(Input::Name, value, Some(3), Some(256))
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
