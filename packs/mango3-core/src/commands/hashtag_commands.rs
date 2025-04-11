use uuid::Uuid;

use crate::models::*;

#[cfg(feature = "all-hashtags-by-ids")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(',') }"#,
    ty = "cached::AsyncRedisCache<String, Hashtags>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_ALL_HASHTAGS_BY_IDS).await } "##
)]
async fn all_cached_hashtags_by_ids(ids: &Vec<Uuid>) -> sqlx::Result<Hashtag's> {
    let db_pool = crate::db_pool().await;
    
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE id = ANY($1)",
        ids // $1
    )
    .fetch_all(db_pool)
    .await
    .map(|hashtag| hashtag.into())
}

#[cfg(feature = "all-hashtags-by-ids")]
pub async fn all_hashtags_by_ids(ids: &Vec<Uuid>) -> Vec<Hashtag> {
    if ids.is_empty() {
        return vec![];
    }
    
    all_cached_hashtags_by_ids(ids)
    .await
    .unwrap_or_default()
}

#[cfg(feature = "get-hashtag-by-id")]
pub async fn get_hashtag_by_id(id: Uuid) -> sqlx::Result<Hashtag> {
    let db_pool = crate::db_pool().await;
    
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE id = $1 LIMIT 1",
        id, // $1
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "get-hashtag-by-name")]
pub async fn get_hashtag_by_name(name: &str) -> sqlx::Result<Hashtag> {
    let db_pool = crate::db_pool().await;
    
    sqlx::query_as!(
        Hashtag,
        "SELECT * FROM hashtags WHERE name = $1 LIMIT 1",
        name, // $1
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "get-or-insert-hashtag")]
pub async fn get_or_insert_hashtag(name: &str) -> crate::utils::MutResult<Hashtag> {
    use crate::enums::{Input, InputError};
    use crate::utils::ValidatorTrait;

    let name = name.trim().to_lowercase();

    if let Ok(hashtag) = get_hashtag_by_name(name).await {
        return crate::mut_success!(hashtag);
    };

    
    let db_pool = crate::db_pool().await;
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
    .fetch_one(db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(feature = "get-or-insert-many-hashtags")]
pub async fn get_or_insert_many_hashtags(content: &str) -> crate::utils::MutResult<Vec<Hashtag>> {
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
            .map(|name| get_or_insert_hashtag(name)),
    )
    .await
    .iter()
    .filter_map(|result| result.as_ref().map(|success| success.data.clone()).ok())
    .collect())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::fake_slug;

    use super::{all_hashtags_by_ids, get_or_insert_hashtag, get_or_insert_many_hashtags};

    #[tokio::test]
    async fn should_return_all_by_ids() {
        let result = all_hashtags_by_ids(&vec![]).await;

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn should_get_or_insert() {
        let slug = fake_slug();

        let result = get_or_insert_hashtag(&slug).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_or_insert_all() {
        let content = "#helloworld #this-is-a-hashtag
    #other-hashtag thisisnotvalid
    #byebye";

        let result = get_or_insert_many_hashtags(content).await;

        assert!(result.is_ok());

        assert_eq!(result.ok().unwrap().data.len(), 4);
    }
}
