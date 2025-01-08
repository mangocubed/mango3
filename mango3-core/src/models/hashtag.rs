use futures::future;
use sqlx::query_as;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS, REGEX_HASHTAG};
use crate::enums::{Input, InputError};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::{hashtag_has_lookaround, CoreContext};

#[derive(Clone)]
pub struct Hashtag {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Hashtag {
    pub async fn all_by_ids(core_context: &CoreContext, ids: &Vec<Uuid>) -> Vec<Self> {
        if ids.is_empty() {
            return vec![];
        }

        query_as!(
            Self,
            "SELECT * FROM hashtags WHERE id = ANY($1)",
            ids // $1
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
    }

    pub async fn get_by_name(core_context: &CoreContext, name: &str) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "SELECT * FROM hashtags WHERE name = $1 LIMIT 1",
            name, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    async fn get_or_insert(core_context: &CoreContext, name: &str) -> Result<Self, ValidationErrors> {
        let name = name.trim().to_lowercase();

        if let Ok(hashtag) = Self::get_by_name(core_context, &name).await {
            return Ok(hashtag);
        };

        let mut validator = Validator::default();

        if validator.validate_presence(Input::Name, &name)
            && validator.validate_format(Input::Name, &name, &REGEX_HASHTAG)
            && validator.validate_length(Input::Name, &name, Some(1), Some(255))
            && validator.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(&name).is_err())
        {
            validator.custom_validation(Input::Name, InputError::IsInvalid, &|| {
                !BLACKLISTED_HASHTAGS.contains(&name.as_str())
            });
        }

        query_as!(
            Self,
            "INSERT INTO hashtags (name) VALUES ($1) RETURNING *",
            name, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_or_insert_all(core_context: &CoreContext, content: &str) -> Result<Vec<Self>, ValidationErrors> {
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

        Ok(
            future::join_all(hashtag_names.iter().map(|name| Self::get_or_insert(core_context, name)))
                .await
                .iter()
                .filter_map(|hashtag| hashtag.as_ref().ok().cloned())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_slug, setup_core_context};

    use super::Hashtag;

    #[tokio::test]
    async fn should_return_all_by_ids() {
        let core_context = setup_core_context().await;

        let result = Hashtag::all_by_ids(&core_context, &vec![]).await;

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn should_get_or_insert() {
        let core_context = setup_core_context().await;
        let slug = fake_slug();

        let result = Hashtag::get_or_insert(&core_context, &slug).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_or_insert_all() {
        let core_context = setup_core_context().await;
        let content = "#helloworld #this-is-a-hashtag
#other-hashtag thisisnotvalid
#byebye";

        let result = Hashtag::get_or_insert_all(&core_context, content).await;

        assert!(result.is_ok());

        assert_eq!(result.ok().unwrap().len(), 4);
    }
}
