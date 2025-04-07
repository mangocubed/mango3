pub const KEY_PARAM_POST_ID: &str = "post_id";
pub const KEY_PARAM_WEBSITE_ID: &str = "website_id";

#[cfg(feature = "ssr")]
pub mod ssr {
    pub const KEY_TEXT_FAILED_TO_CREATE_WEBSITE: &str = "failed-to-create-website";
    pub const KEY_TEXT_WEBSITE_CREATED_SUCCESSFULLY: &str = "website-created-successfully";
}
