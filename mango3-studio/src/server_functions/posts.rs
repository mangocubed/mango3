use leptos::prelude::*;

#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use serde_json::Value;
#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, CursorPageResp, PostPreviewResp, PostResp};

#[cfg(feature = "ssr")]
use mango3_core::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS};
#[cfg(feature = "ssr")]
use mango3_core::hashtag_has_lookaround;
#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, Post, User, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::{FromCore, HashtagResp, UserPreviewResp};
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, extract_user, parse_html, render_handlebars};

use crate::models::EditPostResp;

#[cfg(feature = "ssr")]
use super::my_website;

#[cfg(feature = "ssr")]
async fn get_blobs(core_context: &CoreContext, user: &User, website: &Website, ids: Option<Vec<String>>) -> Vec<Blob> {
    let Some(ids) = ids else {
        return vec![];
    };

    Blob::all_by_ids(
        &core_context,
        &ids.iter().map(|id| Uuid::try_parse(id).unwrap()).collect(),
        Some(&user),
        Some(&website),
    )
    .await
}

#[server]
pub async fn preview_post(
    title: String,
    content: String,
    variables: String,
    cover_image_blob_id: Option<String>,
) -> Result<PostResp, ServerFnError> {
    let title = title.trim().to_owned();
    let content = content.trim();
    let variables = variables.parse::<Value>().unwrap_or_default();

    let core_context = expect_context();
    let user = extract_user().await?.unwrap();
    let content_html = parse_html(&render_handlebars(content, &variables)?, true);

    let mut hashtag_names = REGEX_FIND_HASHTAGS
        .captures_iter(content)
        .filter_map(|captures| {
            captures.name("name").and_then(|match_| {
                let name = match_.as_str();
                if !BLACKLISTED_HASHTAGS.contains(&name) && hashtag_has_lookaround(content, match_) {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<&str>>();

    hashtag_names.dedup();

    let hashtags = hashtag_names
        .iter()
        .map(|name| HashtagResp {
            id: String::new(),
            name: (*name).to_owned(),
        })
        .collect::<Vec<HashtagResp>>();

    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user))
            .await
            .map(|blob| blob.into())
            .ok()
    } else {
        None
    };

    Ok(PostResp {
        id: String::new(),
        user: UserPreviewResp::from_core(&core_context, &user).await,
        title,
        slug: String::new(),
        content_html,
        hashtags,
        cover_image_blob,
        blobs: vec![],
        is_published: true,
        url: String::new(),
        views_count: 0,
        comments_count: 0,
        reactions_count: 0,
        published_at: None,
        modified_at: None,
        created_at: Utc::now(),
        updated_at: None,
    })
}

#[server]
pub async fn attempt_to_create_post(
    website_id: String,
    title: String,
    slug: String,
    content: String,
    variables: String,
    blob_ids: Option<Vec<String>>,
    cover_image_blob_id: Option<String>,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(website) = my_website(&website_id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let blobs = get_blobs(&core_context, &user, &website, blob_ids).await;
    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let result = Post::insert(
        &core_context,
        &website,
        &user,
        &title,
        &slug,
        &content,
        &variables,
        blobs,
        cover_image_blob.as_ref(),
        publish.unwrap_or_default(),
    )
    .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_delete_post(website_id: String, id: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(post) = my_post(&website_id, &id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let result = post.delete(&core_context).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_post(
    website_id: String,
    id: String,
    title: String,
    slug: String,
    content: String,
    variables: String,
    blob_ids: Option<Vec<String>>,
    cover_image_blob_id: Option<String>,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(post) = my_post(&website_id, &id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let website = post.website(&core_context).await?;
    let blobs = get_blobs(&core_context, &user, &website, blob_ids).await;
    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let result = post
        .update(
            &core_context,
            &title,
            &slug,
            &content,
            &variables,
            blobs,
            cover_image_blob.as_ref(),
            publish.unwrap_or_default(),
        )
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_my_post(website_id: String, id: String) -> Result<Option<EditPostResp>, ServerFnError> {
    if let Some(post) = my_post(&website_id, &id).await? {
        let core_context = expect_core_context();

        Ok(Some(EditPostResp::from_core(&core_context, &post).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_posts(
    website_id: String,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let Some(website) = my_website(&website_id).await? else {
        return Ok(CursorPageResp::default());
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, Some(&website), Some(&user), None, None).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[cfg(feature = "ssr")]
async fn my_post(website_id: &str, id: &str) -> Result<Option<Post>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(Post::get_by_id(
        &core_context,
        Uuid::try_parse(id)?,
        Some(&website),
        Some(&user),
        None,
        None,
    )
    .await
    .ok())
}
