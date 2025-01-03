use leptos::prelude::*;

use mango3_leptos_utils::components::WebsiteCard;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::icons::PlusOutlined;
use mango3_leptos_utils::pages::AuthenticatedPage;

use crate::components::MyWebsitesInfiniteScroll;
use crate::context::use_selected_website;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let basic_config = use_basic_config();
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <AuthenticatedPage title=move || t_string!(i18n, shared.home)>
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

                <a class="btn btn-block ml-auto mr-auto mt-4" href=basic_config.new_website_url>
                    <PlusOutlined />

                    {t!(i18n, shared.new_website)}
                </a>
            </section>
        </AuthenticatedPage>
    }
}
