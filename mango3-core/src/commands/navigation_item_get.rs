use std::future::Future;

use sqlx::query_as;
use uuid::Uuid;

use crate::models::Website;
use crate::CoreContext;

use mango3_utils::models::NavigationItem;

pub trait NavigationItemGet {
    fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
    ) -> impl Future<Output = sqlx::Result<NavigationItem>>;
}

impl NavigationItemGet for NavigationItem {
    fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
    ) -> impl Future<Output = sqlx::Result<Self>> {
        let website_id = website.map(|website| website.id);
        query_as!(
            Self,
            "SELECT * FROM navigation_items WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2) LIMIT 1",
            id,         // $1
            website_id, // $2
        )
        .fetch_one(&core_context.db_pool)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_navigation_item, setup_core_context};

    use super::{NavigationItem, NavigationItemGet};

    #[tokio::test]
    async fn should_get_navigation_item_by_id() {
        let core_context = setup_core_context().await;
        let navigation_item = insert_test_navigation_item(&core_context, None).await;

        let result = NavigationItem::get_by_id(&core_context, navigation_item.id, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_navigation_item_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = NavigationItem::get_by_id(&core_context, id, None).await;

        assert!(result.is_err());
    }
}
