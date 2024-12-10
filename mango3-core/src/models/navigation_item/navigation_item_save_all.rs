use sqlx::types::Uuid;

use crate::models::Website;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::NavigationItem;

impl NavigationItem {
    pub async fn save_all(
        core_context: &CoreContext,
        website: &Website,
        items: Vec<(Option<Uuid>, String, String)>,
    ) -> Result<(), ValidationErrors> {
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_name, fake_url, insert_test_website, setup_core_context};

    use super::NavigationItem;
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
