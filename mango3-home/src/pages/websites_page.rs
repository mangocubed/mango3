use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{InfiniteScroll, WebsiteCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::WebsiteResp;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_websites;

#[component]
pub fn WebsitesPage() -> impl IntoView {
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let websites_resource = Resource::new_blocking(move || after.get(), get_websites);

    let title = move || t_string!(i18n, home.websites);

    view! {
        <Page title=title>
            <h2 class="h2">{title}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll
                    after=after
                    key=|website: &WebsiteResp| website.id.clone()
                    resource=websites_resource
                    let:website
                >
                    <WebsiteCard website=website />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
