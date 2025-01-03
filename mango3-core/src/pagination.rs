use std::future::Future;

use uuid::Uuid;

use crate::CoreContext;

#[derive(Clone)]
pub struct CursorPage<T> {
    pub end_cursor: Option<Uuid>,
    pub nodes: Vec<T>,
    pub has_next_page: bool,
}

impl<T> CursorPage<T>
where
    T: Clone,
{
    pub async fn new<'a, CT, CF, RT, RF, QF>(
        core_context: &'a CoreContext,
        cursor_page_params: &CursorPageParams,
        cursor_fn: CF,
        cursor_resource_fn: RF,
        query_fn: QF,
    ) -> Self
    where
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

        Self {
            end_cursor,
            nodes,
            has_next_page,
        }
    }
}

impl<T> Default for CursorPage<T> {
    fn default() -> Self {
        Self {
            end_cursor: None,
            nodes: vec![],
            has_next_page: false,
        }
    }
}

pub struct CursorPageParams {
    pub after: Option<Uuid>,
    pub first: u8,
}

impl Default for CursorPageParams {
    fn default() -> Self {
        Self { after: None, first: 10 }
    }
}
