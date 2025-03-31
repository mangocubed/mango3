use std::future::Future;

use sqlx::query_as;

use mango3_utils::models::UserSession;

use crate::enums::MailerJobCommand;
use crate::models::User;
use crate::validator::ValidationErrors;
use crate::CoreContext;

pub trait UserSessionInsert {
    fn insert(core_context: &CoreContext, user: &User) -> impl Future<Output = Result<UserSession, ValidationErrors>>;
}

impl UserSessionInsert for UserSession {
    fn insert(core_context: &CoreContext, user: &User) -> impl Future<Output = Result<Self, ValidationErrors>> {
        async {
            let result = query_as!(
                Self,
                "INSERT INTO user_sessions (user_id) VALUES ($1) RETURNING *",
                user.id, // $1
            )
            .fetch_one(&core_context.db_pool)
            .await;

            match result {
                Ok(user_session) => {
                    core_context.jobs.mailer(user, MailerJobCommand::NewUserSession).await;

                    Ok(user_session)
                }
                Err(_) => Err(ValidationErrors::default()),
            }
        }
    }
}
