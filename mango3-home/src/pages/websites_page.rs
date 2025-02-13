use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, WebsiteCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::WebsitePreviewResp;
use mango3_leptos_utils::pages::Page;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::server_functions::get_websites;

#[component]
pub fn WebsitesPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollController::new(|after| {
        Resource::new_blocking(
            move || after.get(),
            |after| async move { get_websites(10, after).await },
        )
    });

    let text_title = async_t_string!(i18n, home.websites).to_signal();

    view! {
        <Page title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll controller=controller key=|website: &WebsitePreviewResp| website.id.clone() let:website>
                    <WebsiteCard website=website />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
