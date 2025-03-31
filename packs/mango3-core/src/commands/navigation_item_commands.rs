use std::future::Future;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::Uuid;

use mango3_utils::models::{NavigationItem, NavigationItems};

use crate::constants::PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE;
use crate::models::{async_redis_cache, Website};
use crate::CoreContext;

impl Validator {
    fn validate_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value) && self.validate_length(Input::Title, value, None, Some(256))
    }
}

#[cfg(feature = "all-navigation-items-by-website")]
fn all_navigation_items_by_website(core_context: &CoreContext, website: &Website) -> impl Future<Output = Vec<Self>> {
    async {
        navigation_item_all_by_website(core_context, website)
            .await
            .map(|items| items.into())
            .unwrap_or_default()
    }
}

#[cfg(feature = "all-navigation-items-by-website")]
#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ website.id }"#,
    ty = "AsyncRedisCache<Uuid, NavigationItems>",
    create = r##" { async_redis_cache(PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE).await } "##
)]
async fn all_cached_navigation_items_by_website(
    core_context: &CoreContext,
    website: &Website,
) -> sqlx::Result<NavigationItems> {
    query_as!(
        NavigationItem,
        "SELECT * FROM navigation_items WHERE website_id = $1 ORDER BY position ASC",
        website.id // $1
    )
    .fetch_all(&core_context.db_pool)
    .await
    .map(|items| items.into())
}

#[cfg(feature = "delete-all-navigation-items")]
async fn delete_all_navigation_items(
    core_context: &CoreContext,
    skip: Vec<NavigationItem>,
    website: &Website,
) -> MutResult {
    let _ = sqlx::query!(
        "DELETE FROM navigation_items WHERE id != ALL($1) AND website_id = $2",
        &skip.iter().map(|item| item.id.clone()).collect::<Vec<Uuid>>(), // $1
        website.id                                                       // $2
    )
    .execute(&core_context.db_pool)
    .await;

    NAVIGATION_ITEM_ALL_BY_WEBSITE
        .cache_remove(PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE, &website.id)
        .await;

    Ok(())
}

#[cfg(feature = "get-navigation-item-by-id")]
pub async fn get_navigation_item_by_id(
    core_context: &CoreContext,
    id: Uuid,
    website: Option<&Website>,
) -> sqlx::Result<NavigationItem> {
    let website_id = website.map(|website| website.id);

    sqlx::query_as!(
        Self,
        "SELECT * FROM navigation_items WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2) LIMIT 1",
        id,         // $1
        website_id, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "insert-navigation-item")]
async fn insert_navigation_item(
    core_context: &CoreContext,
    website: &Website,
    position: i16,
    title: &str,
    url: &str,
) -> MutResult<NavigationItem> {
    let title = title.trim();
    let url = url.trim().to_lowercase();

    let mut validator = Validator::default();

    validator.validate_title(title);

    if !validator.is_valid {
        return Err(validator.errors);
    }

    sqlx::query_as!(
        Self,
        "INSERT INTO navigation_items (website_id, position, title, url) VALUES ($1, $2, $3, $4) RETURNING *",
        website.id, // $1
        position,   // $2
        title,      // $3
        url,        // $4
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "save-all-navigation-items")]
pub async fn save_all(
    core_context: &CoreContext,
    website: &Website,
    items: Vec<(Option<Uuid>, String, String)>,
) -> MutResult {
    let mut position = 0;

    let mut skip_from_removal = vec![];

    for (id, title, url) in items {
        if let Some(id) = id {
            let Ok(nav_item) = get_navigation_item_by_id(core_context, id, Some(website)).await else {
                continue;
            };

            let Ok(nav_item) = update_navigation_item(core_context, nav_item, position, &title, &url).await else {
                continue;
            };

            skip_from_removal.push(nav_item);
        } else {
            let Ok(nav_item) = insert_navigation_item(core_context, website, position, &title, &url).await else {
                continue;
            };

            skip_from_removal.push(nav_item);
        }

        position += 1
    }

    let _ = delete_all_navigation_items(core_context, skip_from_removal, website).await;

    Ok(())
}

#[cfg(feature = "update-navigation-item")]
async fn update_navigation_item(
    core_context: &CoreContext,
    navigation_item: &NavigationItem,
    position: i16,
    title: &str,
    url: &str,
) -> Result<Self, ValidationErrors> {
    let title = title.trim();
    let url = url.trim().to_lowercase();

    let mut validator = validator!();

    validator.validate_title(title);

    if !validator.is_valid {
        return Err(validator.errors);
    }

    query_as!(
        Self,
        "UPDATE navigation_items SET position = $2, title = $3, url = $4 WHERE id = $1 RETURNING *",
        navigation_item.id, // $1
        position,           // $2
        title,              // $3
        url,                // $4
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_navigation_item, insert_test_website, setup_core_context};

    use super::{all_navigation_items_by_website, get_navigation_item_by_id};

    #[tokio::test]
    async fn should_get_zero_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let items = all_navigation_items_by_website(&core_context, &website).await;

        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let items = all_navigation_items_by_website(&core_context, &website).await;

        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn should_get_navigation_item_by_id() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;

        let result = get_navigation_item_by_id(&core_context, navigation_item.id, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_navigation_item_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_navigation_item_by_id(&core_context, id, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_delete_all_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let result = delete_all_navigation_items(&core_context, vec![], &website).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_insert_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let title = fake_name();
        let url = fake_url();

        let result = insert_navigation_item(&core_context, &website, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_insert_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let url = fake_url();

        let result = insert_navigation_item(&core_context, &website, 0, "", &url).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_update_navigation_item() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;
        let title = fake_name();
        let url = fake_url();

        let result = navigation_item.update(&core_context, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;
        let url = fake_url();

        let result = update_navigation_item(&core_context, &navigation_item, 0, "", &url).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_save_all_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let items = vec![
            (None, fake_name(), fake_url()),
            (None, fake_name(), fake_url()),
            (None, fake_name(), fake_url()),
        ];

        let result = save_all_navigation_items(&core_context, &website, items).await;

        assert!(result.is_ok());
    }
}
