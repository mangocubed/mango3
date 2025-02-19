use sqlx::query;

use crate::enums::MailerJobCommand;
use crate::models::UserSession;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn disable(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query!(
            "UPDATE users SET disabled_at = current_timestamp WHERE role = 'user' AND disabled_at IS NULL AND id = $1",
            self.id
        )
        .execute(&core_context.db_pool)
        .await;

        match result {
            Ok(_) => {
                UserSession::delete_all(core_context, self)
                    .await
                    .expect("could not delete user sessions");

                core_context.jobs.mailer(self, MailerJobCommand::Disabled).await;

                self.cache_remove().await;

                Ok(())
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

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

    pub fn is_disabled(&self) -> bool {
        self.disabled_at.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_user, setup_core_context};

    #[tokio::test]
    async fn should_disable_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = user.disable(&core_context).await;

        assert!(result.is_ok());
    }
}
