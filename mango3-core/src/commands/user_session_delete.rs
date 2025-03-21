use std::future::Future;

use futures::future;
use sqlx::query;

use mango3_utils::models::UserSession;

use crate::constants::PREFIX_GET_USER_SESSION_BY_ID;
use crate::models::{AsyncRedisCacheTrait, User};
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::{UserSessionAll, GET_USER_SESSION_BY_ID};

pub trait UserSessionDelete {
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>>;

    fn delete_all(core_context: &CoreContext, user: &User) -> impl Future<Output = Result<(), ValidationErrors>>;
}

impl UserSessionDelete for UserSession {
    #[allow(clippy::manual_async_fn)]
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>> {
        async {
            query!("DELETE FROM user_sessions WHERE id = $1", self.id)
                .execute(&core_context.db_pool)
                .await
                .map(|_| ())
                .map_err(|_| ValidationErrors::default())?;

            GET_USER_SESSION_BY_ID
                .cache_remove(PREFIX_GET_USER_SESSION_BY_ID, &self.id)
                .await;

            Ok(())
        }
    }

    #[allow(clippy::manual_async_fn)]
    fn delete_all(core_context: &CoreContext, user: &User) -> impl Future<Output = Result<(), ValidationErrors>> {
        async {
            future::join_all(
                Self::all_by_user(core_context, user)
                    .await
                    .iter()
                    .map(|user_session| user_session.delete(core_context)),
            )
            .await;

            Ok(())
        }
    }
}
