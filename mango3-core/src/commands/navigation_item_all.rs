use std::future::Future;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::Uuid;

use mango3_utils::models::{NavigationItem, NavigationItems};

use crate::constants::PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE;
use crate::models::{async_redis_cache, Website};
use crate::CoreContext;

pub trait NavigationItemAll {
    fn all_by_website(core_context: &CoreContext, website: &Website) -> impl Future<Output = Vec<NavigationItem>>;
}

impl NavigationItemAll for NavigationItem {
    fn all_by_website(core_context: &CoreContext, website: &Website) -> impl Future<Output = Vec<Self>> {
        async {
            navigation_item_all_by_website(core_context, website)
                .await
                .map(|items| items.into())
                .unwrap_or_default()
        }
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ website.id }"#,
    ty = "AsyncRedisCache<Uuid, NavigationItems>",
    create = r##" { async_redis_cache(PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE).await } "##
)]
pub(crate) async fn navigation_item_all_by_website(
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_navigation_item, insert_test_website, setup_core_context};

    use super::{NavigationItem, NavigationItemAll};

    #[tokio::test]
    async fn should_get_zero_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let items = NavigationItem::all_by_website(&core_context, &website).await;

        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let items = NavigationItem::all_by_website(&core_context, &website).await;

        assert_eq!(items.len(), 1);
    }
}
