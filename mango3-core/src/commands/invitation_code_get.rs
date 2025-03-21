use std::future::Future;

use sqlx::query_as;
use uuid::Uuid;

use mango3_utils::models::InvitationCode;

use crate::CoreContext;

pub trait InvitationCodeGet {
    fn get_by_code(core_context: &CoreContext, code: &str) -> impl Future<Output = sqlx::Result<InvitationCode>>;

    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = sqlx::Result<InvitationCode>>;
}

impl InvitationCodeGet for InvitationCode {
    fn get_by_code(core_context: &CoreContext, code: &str) -> impl Future<Output = sqlx::Result<Self>> {
        query_as!(Self, "SELECT * FROM invitation_codes WHERE code = $1 LIMIT 1", code).fetch_one(&core_context.db_pool)
    }

    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = sqlx::Result<Self>> {
        query_as!(Self, "SELECT * FROM invitation_codes WHERE id = $1 LIMIT 1", id,).fetch_one(&core_context.db_pool)
    }
}
