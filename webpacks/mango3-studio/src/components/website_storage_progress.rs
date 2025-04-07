use leptos::prelude::*;

use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::presenters::WebsitePresenter;

#[component]
pub fn WebsiteStorageProgress(website: WebsitePresenter) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="max-w-[720px] w-full mx-auto">
            <h3 class="h3">{t!(i18n, studio.storage)}</h3>

            <progress class="progress progress-primary" value=website.used_storage max=website.max_storage />

            <div>{website.used_storage_str} " / " {website.max_storage_str}</div>
        </section>
    }
}
