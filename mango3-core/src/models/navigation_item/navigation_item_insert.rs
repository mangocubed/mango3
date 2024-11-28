use sqlx::query_as;

use crate::models::Website;
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

use super::NavigationItem;

impl NavigationItem {
    pub async fn insert(
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
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_name, fake_url, insert_test_website, setup_core_context};

    use super::NavigationItem;

    #[tokio::test]
    async fn should_insert_navigation_item() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context).await;
        let title = fake_name();
        let url = fake_url();

        let result = NavigationItem::insert(&core_context, &website, 0, &title, &url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_insert_navigation_item_when_title_is_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context).await;
        let url = fake_url();

        let result = NavigationItem::insert(&core_context, &website, 0, "", &url).await;

        assert!(result.is_err());
    }
}
