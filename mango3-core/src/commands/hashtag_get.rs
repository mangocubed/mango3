use std::future::Future;

use sqlx::query_as;
use uuid::Uuid;

use mango3_utils::models::Hashtag;

use crate::CoreContext;

pub trait HashtagGet {
    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = sqlx::Result<Hashtag>>;

    fn get_by_name(core_context: &CoreContext, name: &str) -> impl Future<Output = sqlx::Result<Hashtag>>;
}

impl HashtagGet for Hashtag {
    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = sqlx::Result<Self>> {
        query_as!(
            Self,
            "SELECT * FROM hashtags WHERE id = $1 LIMIT 1",
            id, // $1
        )
        .fetch_one(&core_context.db_pool)
    }

    fn get_by_name(core_context: &CoreContext, name: &str) -> impl Future<Output = sqlx::Result<Self>> {
        query_as!(
            Self,
            "SELECT * FROM hashtags WHERE name = $1 LIMIT 1",
            name, // $1
        )
        .fetch_one(&core_context.db_pool)
    }
}
