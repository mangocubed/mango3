use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::enums::Input;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::Website;

mod navigation_item_insert;
mod navigation_item_save_all;
mod navigation_item_update;

pub struct NavigationItem {
    pub id: Uuid,
    pub website_id: Uuid,
    pub position: i16,
    pub title: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NavigationItem {
    pub async fn all_by_website(core_context: &CoreContext, website: &Website) -> Vec<Self> {
        query_as!(
            Self,
            "SELECT * FROM navigation_items WHERE website_id = $1 ORDER BY position ASC",
            website.id // $1
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
    }

    pub async fn delete_all(
        core_context: &CoreContext,
        skip: Vec<Self>,
        website: &Website,
    ) -> Result<(), ValidationErrors> {
        query!(
            "DELETE FROM navigation_items WHERE id != ALL($1) AND website_id = $2",
            &skip.iter().map(|item| item.id).collect::<Vec<Uuid>>(), // $1
            website.id                                               // $2
        )
        .execute(&core_context.db_pool)
        .await
        .map(|_| ())
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid, website: Option<&Website>) -> sqlx::Result<Self> {
        let website_id = website.map(|website| website.id);
        query_as!(
            Self,
            "SELECT * FROM navigation_items WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2) LIMIT 1",
            id,         // $1
            website_id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }
}

impl Validator {
    fn validate_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value) && self.validate_length(Input::Title, value, None, Some(255))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_navigation_item, insert_test_website, setup_core_context};

    use super::NavigationItem;

    #[tokio::test]
    async fn should_get_zero_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context).await;

        let items = NavigationItem::all_by_website(&core_context, &website).await;

        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let items = NavigationItem::all_by_website(&core_context, &website).await;

        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn should_delete_all_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let result = NavigationItem::delete_all(&core_context, vec![], &website).await;

        assert!(result.is_ok());
    }

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
