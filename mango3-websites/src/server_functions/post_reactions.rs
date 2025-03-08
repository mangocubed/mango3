use leptos::prelude::*;

use mango3_leptos_utils::models::FormResp;

#[cfg(feature = "ssr")]
use mango3_core::models::PostReaction;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, extract_user, require_authentication};

#[cfg(feature = "ssr")]
use super::posts::current_post;

#[server]
pub async fn attempt_to_delete_post_reaction(post_id: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();
    let core_context = expect_core_context();

    let post_reaction = PostReaction::get_by_post_and_user(&core_context, &post, &user).await?;

    let result = post_reaction.delete(&core_context).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_insert_or_update_post_reaction(
    post_id: String,
    emoji: String,
) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();
    let core_context = expect_core_context();

    let result = PostReaction::insert_or_update(&core_context, &post, &user, &emoji).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn get_my_post_reaction_emoji(post_id: String) -> Result<Option<String>, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(None);
    };

    let post = current_post(post_id).await?;
    let core_context = expect_core_context();

    let result = PostReaction::get_by_post_and_user(&core_context, &post, &user).await;

    match result {
        Ok(reaction) => Ok(Some(reaction.emoji)),
        Err(_) => Ok(None),
    }
}

#[server]
pub async fn get_post_reaction_emojis_count(post_id: String) -> Result<Vec<(String, i64)>, ServerFnError> {
    let core_context = expect_core_context();
    let post = current_post(post_id).await?;

    Ok(PostReaction::get_emojis_count(&core_context, &post).await?)
}
