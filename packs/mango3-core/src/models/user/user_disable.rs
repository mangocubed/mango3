use sqlx::query;

use mango3_utils::models::UserSession;

use crate::commands::UserSessionDelete;
use crate::enums::MailerJobCommand;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn enable(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query!(
            "UPDATE users SET disabled_at = NULL WHERE disabled_at IS NOT NULL AND id = $1",
            self.id
        )
        .execute(&core_context.db_pool)
        .await;

        match result {
            Ok(_) => {
                core_context.jobs.mailer(self, MailerJobCommand::Enabled).await;

                self.cache_remove().await;

                Ok(())
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_user, setup_core_context};
}
