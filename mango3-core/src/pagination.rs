use std::future::Future;

use uuid::Uuid;

use mango3_utils::models::{CursorPage, CursorPageParams};

use crate::CoreContext;

#[macro_export]
macro_rules! create_cursor_page {
    ($($tt:tt)*) => {
        $crate::pagination::create_cursor_page($($tt)*)
    };
}

pub async fn create_cursor_page<'a, T, CT, CF, RT, RF, QF>(
    core_context: &'a CoreContext,
    cursor_page_params: &CursorPageParams,
    cursor_fn: CF,
    cursor_resource_fn: RF,
    query_fn: QF,
) -> CursorPage<T>
where
    T: Clone,
    CF: Fn(T) -> Uuid,
    CT: Future<Output = Option<T>>,
    RF: Fn(&'a CoreContext, Uuid) -> CT,
    RT: Future<Output = Vec<T>>,
    QF: Fn(&'a CoreContext, Option<T>, i64) -> RT,
{
    let cursor_resource = if let Some(after) = cursor_page_params.after {
        cursor_resource_fn(core_context, after).await
    } else {
        None
    };
    let limit = cursor_page_params.first + 1;
    let mut nodes = query_fn(core_context, cursor_resource, limit.into()).await;

    let has_next_page = if nodes.len() > cursor_page_params.first as usize {
        nodes.remove(nodes.len() - 1);

        true
    } else {
        false
    };

    let end_cursor = nodes.last().map(|n| cursor_fn(n.clone()));

    CursorPage {
        end_cursor,
        nodes,
        has_next_page,
    }
}
