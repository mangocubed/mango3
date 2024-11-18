use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};
use url::Url;

use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::{Blob, User, Website};

mod post_insert;
mod post_paginate;

#[derive(Clone)]
pub struct Post {
    pub id: Uuid,
    pub website_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub cover_image_blob_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Post {
    pub fn content_preview(&self) -> &str {
        self.content.lines().next().unwrap_or_default()
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
    ) -> sqlx::Result<Self> {
        let website_id = website.map(|website| website.id);
        let user_id = user.map(|user| user.id);
        query_as!(
            Self,
            "SELECT * FROM posts WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2)
                AND ($3::uuid IS NULL OR user_id = $3) LIMIT 1",
            id,         // $1
            website_id, // $2
            user_id     // $3
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_slug(core_context: &CoreContext, slug: &str, website: &Website) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "SELECT * FROM posts WHERE slug = $1 AND website_id = $2 AND published_at IS NOT NULL LIMIT 1",
            slug,       // $1
            website.id  // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    pub async fn url(&self, core_context: &CoreContext) -> Url {
        self.website(core_context)
            .await
            .unwrap()
            .url()
            .join(&self.slug)
            .unwrap()
    }

    pub async fn website(&self, core_context: &CoreContext) -> sqlx::Result<Website> {
        Website::get_by_id(core_context, self.website_id, None).await
    }
}
