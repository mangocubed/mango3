pub const KEY_PARAM_POST_ID: &str = "post_id";
pub const KEY_PARAM_WEBSITE_ID: &str = "website_id";

#[cfg(feature = "ssr")]
pub mod ssr {
    pub const KEY_TEXT_FAILED_TO_CREATE_POST: &str = "failed-to-create-post";
    pub const KEY_TEXT_FAILED_TO_CREATE_WEBSITE: &str = "failed-to-create-website";
    pub const KEY_TEXT_FAILED_TO_SAVE_NAVIGATION: &str = "failed-to-save-navigation";
    pub const KEY_TEXT_FAILED_TO_UPDATE_POST: &str = "failed-to-update-post";
    pub const KEY_TEXT_FAILED_TO_UPDATE_WEBSITE: &str = "failed-to-update-website";
    pub const KEY_TEXT_NAVIGATION_SAVED_SUCCESSFULLY: &str = "navigation-saved-successfully";
    pub const KEY_TEXT_POST_CREATED_SUCCESSFULLY: &str = "post-created-successfully";
    pub const KEY_TEXT_POST_UPDATED_SUCCESSFULLY: &str = "post-updated-successfully";
    pub const KEY_TEXT_WEBSITE_CREATED_SUCCESSFULLY: &str = "website-created-successfully";
    pub const KEY_TEXT_WEBSITE_UPDATED_SUCCESSFULLY: &str = "website-updated-successfully";
}
