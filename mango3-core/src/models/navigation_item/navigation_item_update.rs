use sqlx::query_as;

use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::NavigationItem;

impl NavigationItem {
    pub async fn update(
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_name, fake_url, insert_test_navigation_item, setup_core_context};

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
}
