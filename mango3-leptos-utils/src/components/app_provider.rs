use codee::string::FromToStringCodec;
use leptos::prelude::*;

use crate::constants::COOKIE_NAME_LANGUAGE;
use crate::context::{provide_basic_config, provide_current_user_resource, provide_info, use_language_cookie_options};
use crate::i18n::I18nContextProvider;

#[component]
pub fn AppProvider(children: Children) -> impl IntoView {
    provide_basic_config();
    provide_current_user_resource();
    provide_info();

    let language_cookie_options = use_language_cookie_options::<FromToStringCodec>();

    view! {
        <I18nContextProvider cookie_name=COOKIE_NAME_LANGUAGE cookie_options=language_cookie_options>
            <div class="flex flex-col min-h-screen">{children()}</div>
        </I18nContextProvider>
    }
}
