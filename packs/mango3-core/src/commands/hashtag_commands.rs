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
}

#[cfg(feature = "get-hashtag-by-name")]
pub async fn get_hashtag_by_name(core_context: &CoreContext, name: &str) -> sqlx::Result<Hashtag> {
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE name = $1 LIMIT 1",
        name, // $1
    )
    .fetch_one(&core_context.db_pool)
}

#[cfg(feature = "get-or-insert-hashtag")]
pub async fn get_or_insert_hashtag(core_context: &CoreContext, name: &str) -> MutResult<Hashtag> {
    let name = name.trim().to_lowercase();

    if let Ok(hashtag) = Self::get_by_name(core_context, &name).await {
        return Ok(hashtag);
    };

    let mut validator = Validator::default();

    if validator.validate_presence(Input::Name, &name)
        && validator.validate_format(Input::Name, &name, &REGEX_HASHTAG)
        && validator.validate_length(Input::Name, &name, Some(1), Some(256))
        && validator.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(&name).is_err())
    {
        validator.custom_validation(Input::Name, InputError::IsInvalid, &|| {
            !BLACKLISTED_HASHTAGS.contains(&name.as_str())
        });
    }

    sqlx::query_as!(
        Self,
        "INSERT INTO hashtags (name) VALUES ($1) RETURNING *",
        name, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map_err(|_| ValidationErrors::default())
}

#[cfg(feature = "get-or-insert-many-hashtags")]
async fn get_or_insert_many_hashtags(core_context: &CoreContext, content: &str) -> MutResult<Vec<Hashtag>> {
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

    if hashtag_names.is_empty() {
        return Ok(vec![]);
    }

    Ok(future::join_all(
        hashtag_names
            .iter()
            .map(|name| get_or_insert_hashtag(core_context, name)),
    )
    .await
    .iter()
    .filter_map(|hashtag| hashtag.as_ref().ok().cloned())
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

        assert_eq!(result.ok().unwrap().len(), 4);
    }
}
