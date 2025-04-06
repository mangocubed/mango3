use leptos::prelude::*;

use mango3_web_utils::components::CopyableText;
use mango3_web_utils::i18n::{t, use_i18n};

use crate::components::{MyWebsitePageWrapper, WebsiteStorageProgress};

#[component]
pub fn ShowPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <MyWebsitePageWrapper let:website>
            <h2 class="h2">{t!(i18n, shared.home)}</h2>

            <WebsiteStorageProgress website=website.clone() />

            <section class="max-w-[720px] w-full mx-auto mt-4">
                <h3 class="h3">{t!(i18n, studio.url)}</h3>

                <div class="flex gap-2">
                    <CopyableText value=website.url.clone() />

                    <a
                        class="btn btn-primary btn-outline"
                        href=if website.is_published { Some(website.url.to_string()) } else { None }
                    >
                        {t!(i18n, studio.go_to_website)}
                    </a>
                </div>
            </section>
        </MyWebsitePageWrapper>
    }
}
