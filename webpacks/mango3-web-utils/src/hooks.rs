use dioxus::prelude::{use_context, Readable, Resource};

use crate::presenters::{AppConfigPresenter, AppInfoPresenter, AppRoutesPresenter};

pub fn use_app_config() -> AppConfigPresenter {
    use_app_config_resource().with(|app_config| app_config.clone().unwrap())
}

pub fn use_app_config_resource() -> Resource<AppConfigPresenter> {
    use_context::<Resource<AppConfigPresenter>>()
}

pub fn use_app_info() -> AppInfoPresenter {
    use_context()
}

pub fn use_app_routes() -> AppRoutesPresenter {
    use_context()
}
