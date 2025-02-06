use cached::IOCachedAsync;
use futures::future;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::{JsonValue, Uuid};
use sqlx::{query, query_as};
use url::Url;

use crate::config::MISC_CONFIG;
use crate::constants::{BLACKLISTED_SLUGS, REGEX_SLUG};
use crate::enums::{Input, InputError};
use crate::validator::{Validator, ValidatorTrait};
use crate::CoreContext;

use super::{Blob, Hashtag, User, Website};

mod post_content;
mod post_delete;
mod post_insert;
mod post_paginate;
mod post_search;
mod post_update;

use post_content::{POST_CONTENT_HTML, POST_CONTENT_PREVIEW_HTML};

#[derive(Clone)]
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
    pub views_count: i64,
    pub comments_count: i64,
    pub reactions_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Post {
    pub async fn blobs(&self, core_context: &CoreContext) -> Vec<Blob> {
        Blob::all_by_ids(core_context, self.blob_ids.clone(), None, None).await
    }

    async fn cache_remove(&self) {
        future::join(
            async {
                if let Some(cache) = POST_CONTENT_HTML.get() {
                    let _ = cache.cache_remove(&self.id).await;
                }
            },
            async {
                if let Some(cache) = POST_CONTENT_PREVIEW_HTML.get() {
                    let _ = cache.cache_remove(&self.id).await;
                }
            },
        )
        .await;
    }

    pub async fn cover_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None, None).await)
        } else {
            None
        }
    }

    pub async fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
        user: Option<&User>,
        is_published: Option<bool>,
        query: Option<&str>,
    ) -> sqlx::Result<Self> {
        let website_id = website.map(|website| website.id);
        let user_id = user.map(|user| user.id);
        query_as!(
            Self,
            r#"SELECT
                id,
                website_id,
                user_id,
                language::varchar AS "language!",
                title,
                slug,
                content,
                variables,
                hashtag_ids,
                cover_image_blob_id,
                blob_ids,
                (SELECT COUNT(*) FROM post_views WHERE post_id = posts.id LIMIT 1) AS "views_count!",
                (SELECT COUNT(*) FROM post_comments WHERE post_id = posts.id LIMIT 1) AS "comments_count!",
                (SELECT COUNT(*) FROM post_reactions WHERE post_id = posts.id LIMIT 1) AS "reactions_count!",
                published_at,
                modified_at,
                CASE
                    WHEN $5::varchar IS NOT NULL THEN ts_rank(search, websearch_to_tsquery($5)) ELSE NULL
                END AS search_rank,
                created_at,
                updated_at
            FROM posts
            WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2)
                AND ($3::uuid IS NULL OR user_id = $3)
                AND (
                    $4::bool IS NULL OR ($4 IS TRUE AND published_at IS NOT NULL)
                    OR ($4 IS FALSE AND published_at IS NULL)
                )
            LIMIT 1"#,
            id,           // $1
            website_id,   // $2
            user_id,      // $3
            is_published, // $4
            query,        // $5
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_slug(core_context: &CoreContext, slug: &str, website: &Website) -> sqlx::Result<Self> {
        query_as!(
            Self,
            r#"SELECT
                id,
                website_id,
                user_id,
                language::varchar as "language!",
                title,
                slug,
                content,
                variables,
                hashtag_ids,
                cover_image_blob_id,
                blob_ids,
                (SELECT COUNT(*) FROM post_views WHERE post_id = posts.id LIMIT 1) AS "views_count!",
                (SELECT COUNT(*) FROM post_comments WHERE post_id = posts.id LIMIT 1) AS "comments_count!",
                (SELECT COUNT(*) FROM post_reactions WHERE post_id = posts.id LIMIT 1) AS "reactions_count!",
                published_at,
                modified_at,
                NULL::real AS search_rank,
                created_at,
                updated_at
            FROM posts WHERE slug = $1 AND website_id = $2 AND published_at IS NOT NULL LIMIT 1"#,
            slug,       // $1
            website.id  // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        Hashtag::all_by_ids(core_context, &self.hashtag_ids).await
    }

    pub async fn is_published(&self, core_context: &CoreContext) -> bool {
        self.website(core_context).await.unwrap().is_published() && self.published_at.is_some()
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

    pub async fn website(&self, core_context: &CoreContext) -> sqlx::Result<Website> {
        Website::get_by_id(core_context, self.website_id, None, None).await
    }
}

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
