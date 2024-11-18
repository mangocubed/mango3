use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use pulldown_cmark::html::push_html;
#[cfg(feature = "ssr")]
use pulldown_cmark::Parser;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::FromCore;

#[cfg(feature = "ssr")]
fn parse_html(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();

    push_html(&mut html_output, parser);

    html_output
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub content_html: String,
    pub content_preview_html: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            slug: post.slug.clone(),
            content: post.content.clone(),
            content_html: parse_html(&post.content),
            content_preview_html: parse_html(&post.content_preview()),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: post.is_published(),
            url: post.url(&core_context).await.to_string(),
        }
    }
}
