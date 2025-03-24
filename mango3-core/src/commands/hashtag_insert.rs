use std::future::Future;

use futures::future;
use sqlx::query_as;
use uuid::Uuid;

use mango3_utils::models::Hashtag;

use crate::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS, REGEX_HASHTAG};
use crate::enums::{Input, InputError};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::{hashtag_has_lookaround, CoreContext};

use super::HashtagGet;

trait HashtagInsert {
    async fn get_or_insert(core_context: &CoreContext, name: &str) -> Result<Hashtag, ValidationErrors>;
}

pub trait HashtagInsertAll {
    fn get_or_insert_all(
        core_context: &CoreContext,
        content: &str,
    ) -> impl Future<Output = Result<Vec<Hashtag>, ValidationErrors>>;
}

impl HashtagInsert for Hashtag {
    async fn get_or_insert(core_context: &CoreContext, name: &str) -> Result<Hashtag, ValidationErrors> {
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

        query_as!(
            Self,
            "INSERT INTO hashtags (name) VALUES ($1) RETURNING *",
            name, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}

impl HashtagInsertAll for Hashtag {
    fn get_or_insert_all(
        core_context: &CoreContext,
        content: &str,
    ) -> impl Future<Output = Result<Vec<Self>, ValidationErrors>> {
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

        async move {
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
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_slug, setup_core_context};

    use super::{Hashtag, HashtagInsert, HashtagInsertAll};

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
