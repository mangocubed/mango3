use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct CursorPage<T> {
    pub end_cursor: Option<Uuid>,
    pub nodes: Vec<T>,
    pub has_next_page: bool,
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
