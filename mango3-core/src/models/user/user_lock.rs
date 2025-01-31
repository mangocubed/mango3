use sqlx::query;

use crate::enums::MailerJobCommand;
use crate::models::{Blob, Post, PostComment, PostReaction, PostView, UserSession, Website};
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

impl User {
    pub async fn lock(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query!(
            "UPDATE users SET
                    email_confirmation_code_id = NULL,
                    password_reset_confirmation_code_id = NULL,
                    avatar_image_blob_id = NULL,
                    encrypted_password = '',
                    display_name = '',
                    full_name = '',
                    bio = '',
                    hashtag_ids = ARRAY[]::uuid [],
                    locked_at = current_timestamp
                WHERE locked_at IS NULL AND id = $1",
            self.id
        )
        .execute(&core_context.db_pool)
        .await;

        match result {
            Ok(_) => {
                UserSession::delete_all(core_context, self)
                    .await
                    .expect("could not delete user sessions");
                Blob::delete_all(core_context, self)
                    .await
                    .expect("could not delete blobs");
                Post::delete_all(core_context, self)
                    .await
                    .expect("could not delete posts");
                PostComment::delete_all(core_context, self)
                    .await
                    .expect("could not delete post comments");
                PostReaction::delete_all(core_context, self)
                    .await
                    .expect("could not delete post reactions");
                PostView::delete_all(core_context, self)
                    .await
                    .expect("could not delete post views");
                Website::delete_all(core_context, self)
                    .await
                    .expect("could not delete websites");

                core_context.jobs.mailer(self, MailerJobCommand::Locked).await;

                Ok(())
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    #[tokio::test]
    async fn should_lock_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let result = user.lock(&core_context).await;

        assert!(result.is_ok());
    }
}
