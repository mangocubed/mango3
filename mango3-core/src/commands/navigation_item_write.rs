use std::future::Future;

use sqlx::{query, query_as};
use uuid::Uuid;

use mango3_utils::models::NavigationItem;

use crate::constants::PREFIX_NAVIGATION_ITEM_ALL_BY_WEBSITE;
use crate::enums::Input;
use crate::models::{AsyncRedisCacheTrait, Website};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{NavigationItemGet, NAVIGATION_ITEM_ALL_BY_WEBSITE};

pub trait NavigationItemBulkWrite {
    fn save_all(
        core_context: &CoreContext,
        website: &Website,
        items: Vec<(Option<Uuid>, String, String)>,
    ) -> impl Future<Output = Result<(), ValidationErrors>>;

    fn delete_all(
        core_context: &CoreContext,
        skip: Vec<NavigationItem>,
        website: &Website,
    ) -> impl Future<Output = Result<(), ValidationErrors>> {
        async move {
            let _ = query!(
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
    }
}

impl NavigationItemBulkWrite for NavigationItem {
    fn save_all(
        core_context: &CoreContext,
        website: &Website,
        items: Vec<(Option<Uuid>, String, String)>,
    ) -> impl Future<Output = Result<(), ValidationErrors>> {
        async {
            let mut position = 0;

            let mut skip_from_removal = vec![];

            for (id, title, url) in items {
                if let Some(id) = id {
                    let Ok(nav_item) = Self::get_by_id(core_context, id, Some(website)).await else {
                        continue;
                    };

                    let Ok(nav_item) = nav_item.update(core_context, position, &title, &url).await else {
                        continue;
                    };

                    skip_from_removal.push(nav_item);
                } else {
                    let Ok(nav_item) = Self::insert(core_context, website, position, &title, &url).await else {
                        continue;
                    };

                    skip_from_removal.push(nav_item);
                }

                position += 1
            }

            let _ = Self::delete_all(core_context, skip_from_removal, website).await;

            Ok(())
        }
    }
}

pub(crate) trait NavigationItemWrite {
    async fn insert(
        core_context: &CoreContext,
        website: &Website,
        position: i16,
        title: &str,
        url: &str,
    ) -> Result<NavigationItem, ValidationErrors>;

    async fn update(
        &self,
        core_context: &CoreContext,
        position: i16,
        title: &str,
        url: &str,
    ) -> Result<NavigationItem, ValidationErrors>;
}

impl NavigationItemWrite for NavigationItem {
    async fn insert(
        core_context: &CoreContext,
        website: &Website,
        position: i16,
        title: &str,
        url: &str,
    ) -> Result<Self, ValidationErrors> {
        let title = title.trim();
        let url = url.trim().to_lowercase();

        let mut validator = Validator::default();

        validator.validate_title(title);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "INSERT INTO navigation_items (website_id, position, title, url) VALUES ($1, $2, $3, $4) RETURNING *",
            website.id, // $1
            position,   // $2
            title,      // $3
            url,        // $4
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    async fn update(
        &self,
        core_context: &CoreContext,
        position: i16,
        title: &str,
        url: &str,
    ) -> Result<Self, ValidationErrors> {
        let title = title.trim();
        let url = url.trim().to_lowercase();

        let mut validator = Validator::default();

        validator.validate_title(title);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "UPDATE navigation_items SET position = $2, title = $3, url = $4 WHERE id = $1 RETURNING *",
            self.id,  // $1
            position, // $2
            title,    // $3
            url,      // $4
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}

impl Validator {
    fn validate_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value) && self.validate_length(Input::Title, value, None, Some(256))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        fake_name, fake_url, insert_test_navigation_item, insert_test_website, setup_core_context,
    };

    use super::{NavigationItem, NavigationItemBulkWrite, NavigationItemWrite};

    #[tokio::test]
    async fn should_delete_all_navigation_items() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        insert_test_navigation_item(&core_context, Some(&website)).await;

        let result = NavigationItem::delete_all(&core_context, vec![], &website).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_insert_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let title = fake_name();
        let url = fake_url();

        let result = NavigationItem::insert(&core_context, &website, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_insert_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let url = fake_url();

        let result = NavigationItem::insert(&core_context, &website, 0, "", &url).await;

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

        let result = navigation_item.update(&core_context, 0, "", &url).await;

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

        let result = NavigationItem::save_all(&core_context, &website, items).await;

        assert!(result.is_ok());
    }
}
