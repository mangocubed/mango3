mod handlebars_utils;
mod markdown;

#[cfg(feature = "blob_read")]
mod text_icon;

pub use handlebars_utils::render_handlebars;
pub use markdown::parse_html;

#[cfg(feature = "blob_read")]
pub use text_icon::text_icon;
