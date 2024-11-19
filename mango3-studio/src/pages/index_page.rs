use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{InfiniteScroll, WebsiteCard};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::WebsiteResp;
use mango3_leptos_utils::pages::AuthenticatedPage;

use crate::server_functions::get_my_websites;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let my_websites_resource = Resource::new_blocking(move || after.get(), get_my_websites);

    view! {
        <AuthenticatedPage title=move || t_string!(i18n, shared.home)>
            <h2 class="h2">{t!(i18n, studio.my_websites)}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll
                    after=after
                    key=|website: &WebsiteResp| website.id.clone()
                    resource=my_websites_resource
                    let:website
                >
                    <WebsiteCard
                        website=website.clone()
                        actions=move || {
                            view! {
                                <a class="btn btn-ghost font-bold" href=format!("/websites/{}", website.id)>
                                    {t!(i18n, shared.view_more)}
                                </a>
                            }
                        }
                    />
                </InfiniteScroll>
            </section>
        </AuthenticatedPage>
    }
}
