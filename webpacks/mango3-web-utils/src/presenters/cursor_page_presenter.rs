use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPage;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromModel;

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! cursor_page_presenter {
    ($core_context:expr, $page:expr) => {
        Ok(CursorPagePresenter::new($core_context, $page).await)
    };
}

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
impl<T> CursorPagePresenter<T> {
    pub async fn new<M>(core_context: &CoreContext, page: &CursorPage<M>) -> Self
    where
        T: FromModel<M>,
     {
        Self {
            end_cursor: page.end_cursor,
            nodes: futures::future::join_all(page.nodes.iter().map(|node| T::from_model(core_context, node))).await,
            has_next_page: page.has_next_page,
        }
    }
}
