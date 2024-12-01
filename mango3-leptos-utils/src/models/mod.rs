use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use futures::future;
#[cfg(feature = "ssr")]
use pulldown_cmark::html::push_html;
#[cfg(feature = "ssr")]
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPage;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

mod action_form_resp;
mod basic_config_resp;
mod blob_resp;
mod navigation_item_resp;
mod page_resp;
mod post_resp;
mod user_profile_resp;
mod user_resp;
mod website_resp;

pub use action_form_resp::ActionFormResp;
pub use basic_config_resp::BasicConfigResp;
pub use blob_resp::BlobResp;
pub use navigation_item_resp::NavigationItemResp;
pub use page_resp::{PagePreviewResp, PageResp};
pub use post_resp::{PostPreviewResp, PostResp};
pub use user_profile_resp::UserProfileResp;
pub use user_resp::UserResp;
pub use website_resp::WebsiteResp;

#[cfg(feature = "ssr")]
fn parse_html(input: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let parser = Parser::new_ext(input, options).filter(|event| match event {
        Event::Start(Tag::HtmlBlock) | Event::End(TagEnd::HtmlBlock) | Event::Html(_) | Event::InlineHtml(_) => false,
        _ => true,
    });

    let mut html_output = String::new();

    push_html(&mut html_output, parser);

    html_output
}

#[cfg(feature = "ssr")]
#[async_trait]
pub trait FromCore<T> {
    async fn from_core(core_context: &CoreContext, value: &T) -> Self;
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CursorPageResp<R> {
    pub end_cursor: Option<String>,
    pub nodes: Vec<R>,
    pub has_next_page: bool,
}

impl<R> Default for CursorPageResp<R> {
    fn default() -> Self {
        Self {
            end_cursor: None,
            nodes: vec![],
            has_next_page: false,
        }
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl<C, R> FromCore<CursorPage<C>> for CursorPageResp<R>
where
    C: Sync,
    R: FromCore<C> + Send,
{
    async fn from_core(core_context: &CoreContext, page: &CursorPage<C>) -> Self {
        Self {
            end_cursor: page.end_cursor.map(|c| c.to_string()),
            nodes: future::join_all(page.nodes.iter().map(|node| R::from_core(core_context, node))).await,
            has_next_page: page.has_next_page,
        }
    }
}
