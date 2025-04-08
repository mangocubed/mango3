use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use serde_json::Value;

use mango3_web_utils::presenters::{CursorPagePresenter, MutPresenter, PostMinPresenter, PostPresenter};

#[cfg(feature = "ssr")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "ssr")]
use mango3_core::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS};
#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::utils::hashtag_has_lookaround;
#[cfg(feature = "ssr")]
use mango3_core::utils::{parse_html, render_handlebars, CursorPageParams};
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n, extract_user};

use crate::presenters::EditPostPresenter;

#[cfg(feature = "ssr")]
use super::{get_blobs_by_ids, my_website};

#[server]
pub async fn preview_post(
    title: String,
    content: String,
    variables: String,
    cover_image_blob_id: Option<Uuid>,
) -> Result<PostPresenter, ServerFnError> {
    use mango3_web_utils::presenters::{BlobPresenter, HashtagPresenter, UserMinPresenter};

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
        .map(|name| HashtagPresenter {
            id: Uuid::new_v4(),
            name: (*name).to_owned(),
        })
        .collect::<Vec<HashtagPresenter>>();

    let cover_image_blob = if let Some(id) = cover_image_blob_id {
        if let Ok(blob) = mango3_core::commands::get_blob_by_id(&core_context, id, None, Some(&user)).await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        }
    } else {
        None
    };

    Ok(PostPresenter {
        id: Uuid::new_v4(),
        user: UserMinPresenter::from_model(&user).await,
        title,
        slug: String::new(),
        content_html,
        hashtags,
        cover_image_blob,
        blobs: vec![],
        is_published: true,
        url: BASIC_CONFIG.home_url(),
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
    website_id: Uuid,
    title: String,
    slug: String,
    content: String,
    variables: String,
    blob_ids: Option<Vec<Uuid>>,
    cover_image_blob_id: Option<Uuid>,
    publish: Option<bool>,
) -> Result<MutPresenter, ServerFnError> {
    use crate::constants::ssr::{KEY_TEXT_FAILED_TO_CREATE_POST, KEY_TEXT_POST_CREATED_SUCCESSFULLY};

    let i18n = extract_i18n().await?;
    let error_message = i18n.text(KEY_TEXT_FAILED_TO_CREATE_POST);

    let Some(website) = my_website(website_id).await? else {
        return mango3_web_utils::mut_presenter_error!(error_message);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let blobs = get_blobs_by_ids(&website, &user, blob_ids).await;
    let cover_image_blob = if let Some(id) = cover_image_blob_id {
        mango3_core::commands::get_blob_by_id(&core_context, id, Some(&website), Some(&user))
            .await
            .ok()
    } else {
        None
    };

    let result = mango3_core::commands::insert_post(
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
    let success_message = i18n.text(KEY_TEXT_POST_CREATED_SUCCESSFULLY);

    mango3_web_utils::mut_presenter!(result, success_message, error_message)
}

#[server]
pub async fn attempt_to_delete_post(website_id: Uuid, id: Uuid) -> Result<MutPresenter, ServerFnError> {
    let Some(post) = my_post(website_id, id).await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();

    let result = mango3_core::commands::delete_post(&core_context, &post).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_update_post(
    website_id: Uuid,
    id: Uuid,
    title: String,
    slug: String,
    content: String,
    variables: String,
    blob_ids: Option<Vec<Uuid>>,
    cover_image_blob_id: Option<Uuid>,
    publish: Option<bool>,
) -> Result<MutPresenter, ServerFnError> {
    use crate::constants::ssr::{KEY_TEXT_FAILED_TO_UPDATE_POST, KEY_TEXT_POST_UPDATED_SUCCESSFULLY};

    let i18n = extract_i18n().await?;
    let error_message = i18n.text(KEY_TEXT_FAILED_TO_UPDATE_POST);

    let Some(post) = my_post(website_id, id).await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let website = post.website(&core_context).await?;
    let blobs = get_blobs_by_ids(&website, &user, blob_ids).await;
    let cover_image_blob = if let Some(id) = cover_image_blob_id {
        mango3_core::commands::get_blob_by_id(&core_context, id, Some(&website), Some(&user))
            .await
            .ok()
    } else {
        None
    };

    let result = mango3_core::commands::update_post(
        &core_context,
        &post,
        &title,
        &slug,
        &content,
        &variables,
        blobs,
        cover_image_blob.as_ref(),
        publish.unwrap_or_default(),
    )
    .await;
    let success_message = i18n.text(KEY_TEXT_POST_UPDATED_SUCCESSFULLY);

    mango3_web_utils::mut_presenter!(result, success_message, error_message)
}

#[server]
pub async fn get_my_post(website_id: Uuid, id: String) -> Result<Option<EditPostPresenter>, ServerFnError> {
    if let Some(post) = my_post(website_id, Uuid::try_parse(&id)?).await? {
        Ok(Some(EditPostPresenter::from_model(&post).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_posts(
    website_id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return mango3_web_utils::cursor_page_presenter!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams { after, first: 10 };
    let page =
        mango3_core::commands::paginate_posts(&core_context, &page_params, Some(&website), Some(&user), None, None)
            .await;

    mango3_web_utils::cursor_page_presenter!(&page)
}

#[cfg(feature = "ssr")]
async fn my_post(website_id: Uuid, id: Uuid) -> Result<Option<Post>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(
        mango3_core::commands::get_post_by_id(&core_context, id, Some(&website), Some(&user), None)
            .await
            .ok(),
    )
}
