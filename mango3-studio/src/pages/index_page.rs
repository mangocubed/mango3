use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{CurrentUser, WebsiteCard};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::PlusOutlined;
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::MyWebsitesInfiniteScroll;
use crate::context::use_selected_website;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <AuthenticatedPage title=async_t_string!(i18n, shared.home).to_signal()>
            <h2 class="h2">{t!(i18n, studio.my_websites)}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <MyWebsitesInfiniteScroll let:website>
                    <WebsiteCard
                        website=website.clone()
                        actions=move || {
                            view! {
                                <a class="btn btn-ghost font-bold" href=format!("/websites/{}", website.id)>
                                    {t!(i18n, studio.select)}
                                </a>
                            }
                        }
                    />
                </MyWebsitesInfiniteScroll>

                <CurrentUser let:user>
                    <Show when=move || {
                        user.can_insert_website
                    }>
                        {move || {
                            let basic_config = use_basic_config();
                            view! {
                                <a class="btn btn-block ml-auto mr-auto mt-4" href=basic_config.new_website_url>
                                    <PlusOutlined />

                                    {t!(i18n, shared.new_website)}
                                </a>
                            }
                        }}
                    </Show>
                </CurrentUser>
            </section>
        </AuthenticatedPage>
    }
}
