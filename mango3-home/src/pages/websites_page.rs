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
    let websites: RwSignal<Vec<WebsiteResp>> = RwSignal::new(vec![]);
    let is_loading = RwSignal::new(true);
    let has_more = RwSignal::new(true);

    Effect::new(move || {
        if let Some(Ok(more_websites)) = websites_resource.get() {
            websites.update(|w| {
                let ids: Vec<String> = w.iter().map(|w| w.id.clone()).collect();
                let mut filtered_websites: Vec<WebsiteResp> = more_websites
                    .clone()
                    .iter()
                    .filter(|mw| !ids.contains(&mw.id))
                    .cloned()
                    .collect();

                has_more.set(!more_websites.is_empty());

                w.append(&mut filtered_websites);
            });

            is_loading.set(false);
        }
    });

    let on_load_more = move |last_item: Option<&WebsiteResp>| {
        if is_loading.get() || !has_more.get() {
            return;
        }

        is_loading.set(true);

        after.set(last_item.map(|item| item.id.clone()));
        websites_resource.refetch();
    };

    let title = move || t_string!(i18n, home.websites);

    view! {
        <Page title=title>
            <h2 class="h2">{title}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll
                    is_loading=is_loading
                    items=websites
                    key=|website| website.id.clone()
                    on_load_more=on_load_more
                    let:website
                >
                    <WebsiteCard website=website />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
