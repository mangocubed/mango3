use uuid::Uuid;

use crate::models::*;

#[cfg(feature = "insert-navigation-item")]
impl crate::utils::Validator {
    fn validate_navigation_item_title(&mut self, value: &str) -> bool {
        use crate::enums::Input;
        use crate::utils::ValidatorTrait;

        self.validate_presence(Input::Title, value) && self.validate_length(Input::Title, value, None, Some(256))
    }
}

#[cfg(feature = "all-navigation-items-by-website")]
pub async fn all_navigation_items_by_website(core_context: &CoreContext, website: &Website) -> Vec<NavigationItem> {
    all_cached_navigation_items_by_website(core_context, website)
        .await
        .map(|items| items.into())
        .unwrap_or_default()
}

#[cfg(feature = "all-navigation-items-by-website")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ website.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, NavigationItems>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_ALL_NAVIGATION_ITEMS_BY_WEBSITE).await } "##
)]
async fn all_cached_navigation_items_by_website(website: &Website) -> sqlx::Result<NavigationItems> {
    let db_pool = crate::db_pool().await;

    sqlx::query_as!(
        NavigationItem,
        "SELECT * FROM navigation_items WHERE website_id = $1 ORDER BY position ASC",
        website.id // $1
    )
    .fetch_all(db_pool)
    .await
    .map(|item| item.into())
}

#[cfg(feature = "all-navigation-items-by-website")]
pub async fn all_navigation_items_by_website(website: &Website) -> Vec<NavigationItem> {
    all_cached_navigation_items_by_website(website)
        .await
        .map(|items| items.into())
        .unwrap_or_default()
}

#[cfg(feature = "delete-all-navigation-items")]
async fn delete_all_navigation_items(skip: Vec<NavigationItem<'_>>, website: &Website) -> crate::utils::MutResult {
    use crate::utils::AsyncRedisCacheTrait;

    let db_pool = crate::db_pool().await;

    let _ = sqlx::query!(
        "DELETE FROM navigation_items WHERE id != ALL($1) AND website_id = $2",
        &skip.iter().map(|item| item.id.clone()).collect::<Vec<Uuid>>(), // $1
        website.id                                                       // $2
    )
    .execute(db_pool)
    .await;

    ALL_CACHED_NAVIGATION_ITEMS_BY_WEBSITE
        .cache_remove(crate::constants::PREFIX_ALL_NAVIGATION_ITEMS_BY_WEBSITE, &website.id)
        .await;

    crate::mut_success!()
}

#[cfg(feature = "get-navigation-item-by-id")]
pub async fn get_navigation_item_by_id(id: Uuid, website: Option<&Website>) -> sqlx::Result<NavigationItem> {
    let db_pool = crate::db_pool().await;
    let website_id = website.map(|website| website.id);

    sqlx::query_as!(
        NavigationItem,
        "SELECT * FROM navigation_items WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2) LIMIT 1",
        id,         // $1
        website_id, // $2
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "insert-navigation-item")]
pub async fn insert_navigation_item<'a>(
    website: &Website,
    position: i16,
    title: &str,
    url: &str,
) -> crate::utils::MutResult<NavigationItem<'a>> {
    let db_pool = crate::db_pool().await;
    let title = title.trim();
    let url = url.trim().to_lowercase();

    let mut validator = crate::validator!();

    validator.validate_navigation_item_title(title);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let result = sqlx::query_as!(
        NavigationItem,
        "INSERT INTO navigation_items (website_id, position, title, url) VALUES ($1, $2, $3, $4) RETURNING *",
        website.id, // $1
        position,   // $2
        title,      // $3
        url,        // $4
    )
    .fetch_one(db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(feature = "insert-or-update-many-navigation-items")]
pub async fn insert_or_update_many_navigation_items(
    website: &Website,
    items: Vec<(Option<Uuid>, String, String)>,
) -> crate::utils::MutResult {
    let mut position = 0;
    let mut skip_from_removal = vec![];

    for (id, title, url) in items {
        if let Some(id) = id {
            let Ok(nav_item) = get_navigation_item_by_id(id, Some(website)).await else {
                continue;
            };

            let Ok(nav_item) = update_navigation_item(&nav_item, position, &title, &url).await else {
                continue;
            };

            skip_from_removal.push(nav_item.data);
        } else {
            let Ok(nav_item) = insert_navigation_item(website, position, &title, &url).await else {
                continue;
            };

            skip_from_removal.push(nav_item.data);
        }

        position += 1
    }

    let _ = delete_all_navigation_items(skip_from_removal, website).await;

    crate::mut_success!()
}

#[cfg(feature = "update-navigation-item")]
async fn update_navigation_item<'a>(
    navigation_item: &NavigationItem<'a>,
    position: i16,
    title: &str,
    url: &str,
) -> crate::utils::MutResult<NavigationItem<'a>> {
    let db_pool = crate::db_pool().await;
    let title = title.trim();
    let url = url.trim().to_lowercase();
    let mut validator = crate::validator!();

    validator.validate_navigation_item_title(title);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let result = sqlx::query_as!(
        NavigationItem,
        "UPDATE navigation_items SET position = $2, title = $3, url = $4 WHERE id = $1 RETURNING *",
        navigation_item.id, // $1
        position,           // $2
        title,              // $3
        url,                // $4
    )
    .fetch_one(db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        fake_name, fake_url, fake_uuid, insert_test_navigation_item, insert_test_website, setup_core_context,
    };

    use super::{
        all_navigation_items_by_website, delete_all_navigation_items, get_navigation_item_by_id,
        insert_navigation_item, insert_or_update_many_navigation_items, update_navigation_item,
    };

    #[tokio::test]
    async fn should_get_zero_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let items = all_navigation_items_by_website(&website).await;

        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let items = all_navigation_items_by_website(&website).await;

        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn should_get_navigation_item_by_id() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;

        let result = get_navigation_item_by_id(navigation_item.id, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_navigation_item_by_id_when_id_is_invalid() {
        let id = fake_uuid();

        let result = get_navigation_item_by_id(id, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_delete_all_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let result = delete_all_navigation_items(vec![], &website).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_insert_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let title = fake_name();
        let url = fake_url();

        let result = insert_navigation_item(&website, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_insert_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let url = fake_url();

        let result = insert_navigation_item(&website, 0, "", &url).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_update_navigation_item() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;
        let title = fake_name();
        let url = fake_url();

        let result = update_navigation_item(&navigation_item, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;
        let url = fake_url();

        let result = update_navigation_item(&navigation_item, 0, "", &url).await;

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

        let result = insert_or_update_many_navigation_items(&website, items).await;

        assert!(result.is_ok());
    }
}
