use leptos::either::Either;
use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{CurrentUser, WebsiteCard};
use mango3_web_utils::context::use_basic_config;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::PlusOutlined;
use mango3_web_utils::pages::AuthenticatedPage;
use mango3_web_utils::utils::ToSignalTrait;

use crate::components::MyWebsitesInfiniteScroll;
use crate::context::use_selected_website;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <AuthenticatedPage title=async_t_string!(i18n, shared.home).to_signal()>
            <h1 class="h1">{t!(i18n, studio.my_websites)}</h1>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <MyWebsitesInfiniteScroll let:website>
                    <WebsiteCard
                        website=website.clone()
                        actions=move || {
                            view! {
                                <a class="btn btn-ghost font-bold" href=format!("/websites/{}", website.id)>
                                    {t!(i18n, shared.select)}
                                </a>
                            }
                        }
                    />
                </MyWebsitesInfiniteScroll>

                <CurrentUser children=move |user| {
                    let basic_config = use_basic_config();
                    if user.can_insert_website {
                        Either::Left(
                            view! {
                                <a
                                    class="btn btn-block ml-auto mr-auto mt-4"
                                    href=basic_config.new_website_url.to_string()
                                >
                                    <PlusOutlined />

                                    {t!(i18n, shared.new_website)}
                                </a>
                            },
                        )
                    } else {
                        Either::Right(())
                    }
                } />
            </section>
        </AuthenticatedPage>
    }
}
