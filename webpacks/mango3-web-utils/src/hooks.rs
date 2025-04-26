use dioxus::prelude::{consume_context, use_context, Readable, Resource};

use crate::presenters::{AppConfigPresenter, BasicConfigPresenter, InfoPresenter, RoutesPresenter};

pub fn use_app_config() -> AppConfigPresenter {
    use_app_config_resource().with(|config| config.clone().unwrap())
}

pub fn use_app_config_resource() -> Resource<AppConfigPresenter> {
    use_context::<Resource<AppConfigPresenter>>()
}

pub fn use_basic_config() -> BasicConfigPresenter {
    use_context::<Resource<BasicConfigPresenter>>().with(|config| config.clone().unwrap())
}

pub fn use_info() -> InfoPresenter {
    use_context::<Resource<InfoPresenter>>().with(|info| info.clone().unwrap())
}

pub fn use_routes() -> RoutesPresenter {
    consume_context()
}
