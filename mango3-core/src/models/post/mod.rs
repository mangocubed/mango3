use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::{JsonValue, Uuid};
use url::Url;

#[cfg(feature = "post_cache_remove")]
use futures::future;
#[cfg(feature = "post_write")]
use sqlx::query;

#[cfg(feature = "hashtag_all")]
use mango3_utils::models::Hashtag;
#[cfg(feature = "post_reaction_count")]
use mango3_utils::models::PostReaction;
#[cfg(feature = "post_view_count")]
use mango3_utils::models::PostView;

use crate::CoreContext;

#[cfg(feature = "hashtag_all")]
use crate::commands::HashtagAll;
#[cfg(feature = "post_reaction_count")]
use crate::commands::PostReactionCount;
#[cfg(feature = "post_view_count")]
use crate::commands::PostViewCount;
#[cfg(feature = "post_write")]
use crate::config::MISC_CONFIG;
#[cfg(feature = "post_cache_remove")]
use crate::constants::{
    BLACKLISTED_SLUGS, PREFIX_GET_POST_BY_ID, PREFIX_GET_POST_BY_SLUG, PREFIX_POST_CONTENT_HTML,
    PREFIX_POST_CONTENT_PREVIEW_HTML, REGEX_SLUG,
};
#[cfg(feature = "post_write")]
use crate::enums::{Input, InputError};
#[cfg(feature = "post_write")]
use crate::validator::{Validator, ValidatorTrait};

#[cfg(feature = "post_cache_remove")]
use super::AsyncRedisCacheTrait;

use super::{Blob, PostComment, User, Website};

mod post_get;

#[cfg(any(feature = "post_content_html", feature = "post_content_preview_html"))]
mod post_content;
#[cfg(feature = "post_write")]
mod post_delete;
#[cfg(feature = "post_write")]
mod post_insert;
#[cfg(feature = "post_paginate")]
mod post_paginate;
#[cfg(feature = "post_search")]
mod post_search;
#[cfg(feature = "post_write")]
mod post_update;

#[cfg(feature = "post_cache_remove")]
use post_content::{POST_CONTENT_HTML, POST_CONTENT_PREVIEW_HTML};
#[cfg(feature = "post_cache_remove")]
use post_get::{GET_POST_BY_ID, GET_POST_BY_SLUG};

#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub website_id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub variables: JsonValue,
    pub hashtag_ids: Vec<Uuid>,
    pub cover_image_blob_id: Option<Uuid>,
    pub blob_ids: Vec<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Post {
    pub async fn blobs(&self, core_context: &CoreContext) -> Vec<Blob> {
        Blob::all_by_ids(core_context, self.blob_ids.clone(), None, None).await
    }

    #[cfg(feature = "post_cache_remove")]
    async fn cache_remove(&self, core_context: &CoreContext) {
        future::join4(
            POST_CONTENT_HTML.cache_remove(PREFIX_POST_CONTENT_HTML, &self.id),
            POST_CONTENT_PREVIEW_HTML.cache_remove(PREFIX_POST_CONTENT_PREVIEW_HTML, &self.id),
            GET_POST_BY_ID.cache_remove(PREFIX_GET_POST_BY_ID, &self.id),
            async {
                let website = self.website(core_context).await.expect("Could not get website");

                GET_POST_BY_SLUG
                    .cache_remove(
                        PREFIX_GET_POST_BY_SLUG,
                        &Self::cache_key_get_by_slug(&self.slug, &website),
                    )
                    .await
            },
        )
        .await;
    }

    pub async fn comments_count(&self, core_context: &CoreContext) -> i64 {
        PostComment::count(core_context, self).await
    }

    pub async fn cover_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None, None).await)
        } else {
            None
        }
    }

    #[cfg(feature = "hashtag_all")]
    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        Hashtag::all_by_ids(core_context, &self.hashtag_ids).await
    }

    pub async fn is_published(&self, core_context: &CoreContext) -> bool {
        self.website(core_context).await.unwrap().is_published() && self.published_at.is_some()
    }

    #[cfg(feature = "post_reaction_count")]
    pub async fn reactions_count(&self, core_context: &CoreContext) -> i64 {
        PostReaction::count(core_context, self).await
    }

    pub async fn url(&self, core_context: &CoreContext) -> Url {
        self.website(core_context)
            .await
            .unwrap()
            .url()
            .join(&self.slug)
            .unwrap()
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        User::get_by_id(core_context, self.user_id).await
    }

    #[cfg(feature = "post_view_count")]
    pub async fn views_count(&self, core_context: &CoreContext) -> i64 {
        PostView::count(core_context, self).await
    }

    pub async fn website(&self, core_context: &CoreContext) -> sqlx::Result<Website> {
        Website::get_by_id(core_context, self.website_id, None).await
    }
}

#[cfg(feature = "post_write")]
impl Validator {
    fn validate_post_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value)
            && self.validate_length(Input::Title, value, None, Some(256))
            && self.custom_validation(Input::Title, InputError::IsInvalid, &|| Uuid::try_parse(value).is_err())
    }

    async fn validate_post_slug(
        &mut self,
        core_context: &CoreContext,
        post: Option<&Post>,
        website: &Website,
        slug: &str,
    ) -> bool {
        if self.validate_presence(Input::Slug, slug)
            && self.validate_format(Input::Slug, slug, &REGEX_SLUG)
            && self.validate_length(Input::Slug, slug, None, Some(256))
            && self.custom_validation(Input::Slug, InputError::IsInvalid, &|| Uuid::try_parse(slug).is_err())
            && self.custom_validation(Input::Slug, InputError::IsInvalid, &|| {
                !BLACKLISTED_SLUGS.contains(&slug)
            })
        {
            let id = post.map(|p| p.id);
            let slug_exists = query!(
                "SELECT id FROM posts WHERE ($1::uuid IS NULL OR id != $1) AND LOWER(slug) = $2 AND website_id = $3 LIMIT 1",
                id,         // $1
                slug,       // $2
                website.id  // $3
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            self.custom_validation(Input::Slug, InputError::AlreadyInUse, &|| !slug_exists)
        } else {
            false
        }
    }

    fn validate_post_content(&mut self, value: &str) -> bool {
        self.validate_length(Input::Content, value, None, Some(MISC_CONFIG.max_post_content_length))
    }

    fn validate_post_variables(&mut self, value: Option<&JsonValue>) -> bool {
        self.custom_validation(Input::Variables, InputError::IsInvalid, &|| value.is_some())
    }
}
