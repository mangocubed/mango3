use uuid::Uuid;

use crate::models::Hashtag;
use crate::CoreContext;

#[cfg(feature = "all-hashtags-by-ids")]
pub async fn all_hashtags_by_ids(core_context: &CoreContext, ids: &Vec<Uuid>) -> Vec<Hashtag> {
    if ids.is_empty() {
        return vec![];
    }

    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE id = ANY($1)",
        ids // $1
    )
    .fetch_all(&core_context.db_pool)
    .await
    .unwrap_or_default()
}

#[cfg(feature = "get-hashtag-by-id")]
pub async fn get_hashtag_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Hashtag> {
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE id = $1 LIMIT 1",
        id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-hashtag-by-name")]
pub async fn get_hashtag_by_name(core_context: &CoreContext, name: &str) -> sqlx::Result<Hashtag> {
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE name = $1 LIMIT 1",
        name, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-or-insert-hashtag")]
pub async fn get_or_insert_hashtag(core_context: &CoreContext, name: &str) -> crate::utils::MutResult<Hashtag> {
    use crate::enums::{Input, InputError};
    use crate::utils::ValidatorTrait;

    let name = name.trim().to_lowercase();

    if let Ok(hashtag) = get_hashtag_by_name(core_context, &name).await {
        return crate::mut_success!(hashtag);
    };

    let mut validator = crate::validator!();

    if validator.validate_presence(Input::Name, &name)
        && validator.validate_format(Input::Name, &name, &crate::constants::REGEX_HASHTAG)
        && validator.validate_length(Input::Name, &name, Some(1), Some(256))
        && validator.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(&name).is_err())
    {
        validator.custom_validation(Input::Name, InputError::IsInvalid, &|| {
            !crate::constants::BLACKLISTED_HASHTAGS.contains(&name.as_str())
        });
    }

    let result = sqlx::query_as!(
        Hashtag,
        "INSERT INTO hashtags (name) VALUES ($1) RETURNING *",
        name, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(feature = "get-or-insert-many-hashtags")]
pub async fn get_or_insert_many_hashtags(
    core_context: &CoreContext,
    content: &str,
) -> crate::utils::MutResult<Vec<Hashtag>> {
    let mut hashtag_names = crate::constants::REGEX_FIND_HASHTAGS
        .captures_iter(content)
        .filter_map(|captures| {
            captures.name("name").and_then(|match_| {
                let name = match_.as_str();
                if !crate::constants::BLACKLISTED_HASHTAGS.contains(&name)
                    && crate::utils::hashtag_has_lookaround(content, match_)
                {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<&str>>();

    hashtag_names.dedup();

    if hashtag_names.is_empty() {
        return crate::mut_success!(vec![]);
    }

    crate::mut_success!(futures::future::join_all(
        hashtag_names
            .iter()
            .map(|name| get_or_insert_hashtag(core_context, name)),
    )
    .await
    .iter()
    .filter_map(|result| result.as_ref().map(|success| success.data.clone()).ok())
    .collect())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_slug, setup_core_context};

    use super::{all_hashtags_by_ids, get_or_insert_hashtag, get_or_insert_many_hashtags};

    #[tokio::test]
    async fn should_return_all_by_ids() {
        let core_context = setup_core_context().await;

        let result = all_hashtags_by_ids(&core_context, &vec![]).await;

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn should_get_or_insert() {
        let core_context = setup_core_context().await;
        let slug = fake_slug();

        let result = get_or_insert_hashtag(&core_context, &slug).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_or_insert_all() {
        let core_context = setup_core_context().await;
        let content = "#helloworld #this-is-a-hashtag
    #other-hashtag thisisnotvalid
    #byebye";

        let result = get_or_insert_many_hashtags(&core_context, content).await;

        assert!(result.is_ok());

        assert_eq!(result.ok().unwrap().data.len(), 4);
    }
}
