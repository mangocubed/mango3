use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};
use url::Url;

use crate::config::MISC_CONFIG;
use crate::constants::{BLACKLISTED_SLUGS, REGEX_SLUG};
use crate::enums::{Input, InputError};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{Blob, PostAttachment, PostView, User, Website};

mod post_insert;
mod post_paginate;
mod post_search;
mod post_update;

#[derive(Clone)]
pub struct Post {
    pub id: Uuid,
    pub website_id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub cover_image_blob_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Post {
    pub async fn attachments(&self, core_context: &CoreContext) -> Vec<PostAttachment> {
        PostAttachment::all(core_context, Some(self)).await
    }

    pub fn content_preview(&self) -> &str {
        self.content
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim())
            .unwrap_or_default()
    }

    pub async fn cover_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(Blob::get_by_id(core_context, id, None).await)
        } else {
            None
        }
    }

    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM posts WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
        user: Option<&User>,
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
                cover_image_blob_id,
                published_at,
                CASE WHEN $4::varchar IS NOT NULL THEN ts_rank(search, websearch_to_tsquery($4)) ELSE NULL END AS search_rank,
                created_at,
                updated_at
            FROM posts WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2)
                AND ($3::uuid IS NULL OR user_id = $3) LIMIT 1"#,
            id,         // $1
            website_id, // $2
            user_id,    // $3
            query,      // $4
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
                cover_image_blob_id,
                published_at,
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

    pub async fn host(&self, core_context: &CoreContext) -> String {
        self.website(core_context).await.unwrap().host()
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

    pub async fn views_count(&self, core_context: &CoreContext) -> sqlx::Result<i64> {
        PostView::count(core_context, self).await
    }

    pub async fn website(&self, core_context: &CoreContext) -> sqlx::Result<Website> {
        Website::get_by_id(core_context, self.website_id, None, None).await
    }
}

impl Validator {
    fn validate_post_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value)
            && self.validate_length(Input::Title, value, Some(3), Some(255))
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
            && self.validate_length(Input::Slug, slug, Some(1), Some(255))
            && self.custom_validation(Input::Slug, InputError::IsInvalid, &|| Uuid::try_parse(slug).is_err())
            && self.custom_validation(Input::Username, InputError::IsInvalid, &|| {
                !BLACKLISTED_SLUGS.contains(&slug.to_owned())
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
}
