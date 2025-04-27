use dioxus::prelude::{use_context, Readable, Resource};

use crate::presenters::{AppConfigPresenter, InfoPresenter, RoutesPresenter};

pub fn use_app_config() -> AppConfigPresenter {
    use_app_config_resource().with(|config| config.clone().unwrap())
}

pub fn use_app_config_resource() -> Resource<AppConfigPresenter> {
    use_context::<Resource<AppConfigPresenter>>()
}

pub fn use_info() -> InfoPresenter {
    use_context::<Resource<InfoPresenter>>().with(|info| info.clone().unwrap())
}

pub fn use_routes() -> RoutesPresenter {
    use_context()
}
