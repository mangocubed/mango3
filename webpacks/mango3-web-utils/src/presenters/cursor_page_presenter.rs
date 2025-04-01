use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPage;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct CursorPagePresenter<T> {
    pub end_cursor: Option<Uuid>,
    pub nodes: Vec<T>,
    pub has_next_page: bool,
}

impl<T> Default for CursorPagePresenter<T> {
    fn default() -> Self {
        Self {
            end_cursor: None,
            nodes: vec![],
            has_next_page: false,
        }
    }
}

#[cfg(feature = "ssr")]
impl<M, P> FromModel<CursorPage<M>> for CursorPagePresenter<P>
where
    P: FromModel<M>,
{
    fn from_model(core_context: &CoreContext, page: &CursorPage<M>) -> impl std::future::Future<Output = Self> {
        async {
            Self {
                end_cursor: page.end_cursor,
                nodes: futures::future::join_all(page.nodes.iter().map(|node| P::from_model(core_context, node))).await,
                has_next_page: page.has_next_page,
            }
        }
    }
}
