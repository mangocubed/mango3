use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::CursorPage;
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
impl FromModel<CursorPage<M>> for CursorPagePresenter<P> {
    fn from_model(core_context: &CoreContext, page: &CursorPage<M>) -> impl std::future::Future<Output = Self> {
        async {
            Self {
                end_cursor: model.end_cursor,
                nodes: futures::future::join_all(model.nodes.iter().map(|node| P::from_model(core_context, node)))
                    .await,
                has_next_page: model.has_next_page,
            }
        }
    }
}
