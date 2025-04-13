use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::MutPresenter;

#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{extract_user, require_authentication};

#[cfg(feature = "ssr")]
use super::posts::current_post;

#[server]
pub async fn attempt_to_delete_post_reaction(post_id: Uuid) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();

    let post_reaction = mango3_core::commands::get_post_reaction_by_post_and_user(&post, &user).await?;

    let result = mango3_core::commands::delete_post_reaction(&post_reaction).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_insert_or_update_post_reaction(
    post_id: Uuid,
    emoji: String,
) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();

    let result = mango3_core::commands::insert_or_update_post_reaction(&post, &user, &emoji).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn get_my_post_reaction_emoji(post_id: Uuid) -> Result<Option<String>, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(None);
    };

    let post = current_post(post_id).await?;

    let result = mango3_core::commands::get_post_reaction_by_post_and_user(&post, &user).await;

    match result {
        Ok(reaction) => Ok(Some(reaction.emoji.to_string())),
        Err(_) => Ok(None),
    }
}

#[server]
pub async fn get_post_reaction_emojis_count(post_id: Uuid) -> Result<Vec<(String, i64)>, ServerFnError> {
    let post = current_post(post_id).await?;

    Ok(mango3_core::commands::get_post_reaction_emojis_count(&post).await?)
}
