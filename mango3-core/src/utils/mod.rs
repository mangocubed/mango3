#[cfg(feature = "handlebars")]
mod handlebars_utils;
#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "blob_read")]
mod text_icon;

#[cfg(feature = "handlebars")]
pub use handlebars_utils::render_handlebars;
#[cfg(feature = "markdown")]
pub use markdown::parse_html;
#[cfg(feature = "blob_read")]
pub use text_icon::text_icon;
