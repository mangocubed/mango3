use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use futures::future;
#[cfg(feature = "ssr")]
use handlebars::{Handlebars, RenderError};
#[cfg(feature = "ssr")]
use pulldown_cmark::html::push_html;
#[cfg(feature = "ssr")]
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
#[cfg(feature = "ssr")]
use regex::Captures;
#[cfg(feature = "ssr")]
use serde_json::{Map, Value};

#[cfg(feature = "ssr")]
use mango3_core::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS, REGEX_HANDLEBARS};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPage;
#[cfg(feature = "ssr")]
use mango3_core::{hashtag_has_lookaround, CoreContext};

#[cfg(feature = "ssr")]
use crate::constants::REGEX_HANDLEBARS_DECLARE;

mod action_form_resp;
mod basic_config_resp;
mod blob_resp;
mod hashtag_resp;
mod navigation_item_resp;
mod post_resp;
mod user_profile_resp;
mod user_resp;
mod website_resp;

pub use action_form_resp::ActionFormResp;
pub use basic_config_resp::BasicConfigResp;
pub use blob_resp::BlobResp;
pub use hashtag_resp::HashtagResp;
pub use navigation_item_resp::NavigationItemResp;
pub use post_resp::{PostAttachmentResp, PostPreviewResp, PostResp};
pub use user_profile_resp::UserProfileResp;
pub use user_resp::{UserPreviewResp, UserResp};
pub use website_resp::{WebsitePreviewResp, WebsiteResp};

#[cfg(feature = "ssr")]
fn render_handlebars(input: &str) -> Result<String, RenderError> {
    if !REGEX_HANDLEBARS.is_match(input) {
        return Ok(input.to_owned());
    }

    let mut registry = Handlebars::new();
    let mut data = Map::new();

    registry.set_prevent_indent(true);

    let input = REGEX_HANDLEBARS_DECLARE.replace_all(input, |captures: &Captures| {
        let key = captures.name("key").expect("Could not get match").as_str().to_owned();

        let value = if let Some(value) = captures.name("bool") {
            Value::Bool(value.as_str() == "true")
        } else if let Some(value) = captures.name("number") {
            Value::Number(serde_json::from_str(value.as_str()).unwrap_or_else(|_| 0.into()))
        } else if let Some(value) = captures.name("string") {
            Value::String(value.as_str().to_owned())
        } else if let Some(value) = captures.name("array") {
            Value::Array(serde_json::from_str(value.as_str()).unwrap_or_default())
        } else if let Some(value) = captures.name("object") {
            Value::Object(serde_json::from_str(value.as_str()).unwrap_or_default())
        } else {
            Value::Null
        };

        data.insert(key, value);

        ""
    });

    registry.render_template(&input, &data)
}

#[cfg(feature = "ssr")]
fn parse_html(input: &str, enable_links: bool) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let input = REGEX_FIND_HASHTAGS.replace_all(input, |captures: &Captures| {
        let match_ = captures.name("name").expect("Could not get match");
        let name = match_.as_str();

        if !BLACKLISTED_HASHTAGS.contains(&name) && hashtag_has_lookaround(input, match_) {
            format!("[#{name}](/hashtags/{name})")
        } else {
            format!("#{name}")
        }
    });

    let parser = Parser::new_ext(&input, options).filter(|event| match event {
        Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        })
        | Event::End(TagEnd::Heading(HeadingLevel::H1)) => false,
        Event::Start(Tag::HtmlBlock) | Event::End(TagEnd::HtmlBlock) | Event::Html(_) | Event::InlineHtml(_) => false,
        Event::Start(Tag::Link { .. }) | Event::End(TagEnd::Link) => enable_links,
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
