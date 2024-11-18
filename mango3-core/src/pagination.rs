use std::future::Future;

use uuid::Uuid;

use crate::CoreContext;

#[derive(Clone)]
pub struct Page<T> {
    pub end_cursor: Option<Uuid>,
    pub nodes: Vec<T>,
    pub has_next_page: bool,
}

impl<T> Page<T>
where
    T: Clone,
{
    pub async fn new<'a, CT, CF, RT, RF, QF>(
        core_context: &'a CoreContext,
        page_params: &PageParams,
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
        let cursor_resource = if let Some(after) = page_params.after {
            cursor_resource_fn(core_context, after).await
        } else {
            None
        };
        let limit = page_params.first + 1;
        let mut nodes = query_fn(core_context, cursor_resource, limit.into()).await;

        let has_next_page = if nodes.len() > page_params.first as usize {
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

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            end_cursor: None,
            nodes: vec![],
            has_next_page: false,
        }
    }
}

pub struct PageParams {
    pub after: Option<Uuid>,
    pub first: i8,
}

impl Default for PageParams {
    fn default() -> Self {
        Self { after: None, first: 10 }
    }
}
