use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_meta::Script;

use crate::constants::COOKIE_NAME_LANGUAGE;
use crate::context::*;
use crate::i18n::I18nContextProvider;

#[component]
pub fn AppProvider(children: Children) -> impl IntoView {
    provide_basic_config();
    provide_current_user_resource();

    let basic_config = use_basic_config();
    let language_cookie_options = use_language_cookie_options::<FromToStringCodec>();
    let google_ads_client = basic_config.google_ads_client.clone();

    view! {
        <Show when={
            let has_google_ads = !google_ads_client.is_empty();
            move || has_google_ads
        }>
            <Script
                async_="true"
                crossorigin="anonymous"
                src=format!(
                    "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={}",
                    google_ads_client,
                )
            />
        </Show>

        <I18nContextProvider cookie_name=COOKIE_NAME_LANGUAGE cookie_options=language_cookie_options>
            <div class="flex flex-col min-h-screen">{children()}</div>
        </I18nContextProvider>
    }
}
