use sqlx::query;

use crate::enums::MailerJobCommand;
use crate::models::UserSession;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn lock(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query!(
            "UPDATE users SET locked_at = current_timestamp WHERE locked_at IS NULL AND id = $1",
            self.id
        )
        .execute(&core_context.db_pool)
        .await;

        match result {
            Ok(_) => {
                UserSession::delete_all(core_context, self)
                    .await
                    .expect("could not delete user sessions");

                core_context.jobs.mailer(self, MailerJobCommand::Locked).await;

                self.cache_remove();

                Ok(())
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_user, setup_core_context};

    #[tokio::test]
    async fn should_lock_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = user.lock(&core_context).await;

        assert!(result.is_ok());
    }
}
