use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::WebsiteCard;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_websites;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let basic_config = use_basic_config();
    let websites_resource = Resource::new_blocking(move || None, get_websites);

    let page_title = move || {
        t_string!(
            i18n,
            home.a_cloud_platform_to_create_websites_in_the_easiest_way_possible
        )
    };

    view! {
        <Page title=page_title>
            <section>
                <div class="hero">
                    <div class="hero-content text-center flex-col">
                        <h2 class="text-2xl font-bold">
                            {t!(i18n, home.welcome_to_title, title = basic_config.title)}
                        </h2>
                        <p class="py-3">{page_title}</p>
                    </div>
                </div>
            </section>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <h3 class="h3">{t!(i18n, home.recent_websites)}</h3>

                <Suspense>
                    {move || Suspend::new(async move {
                        websites_resource
                            .get()
                            .and_then(|result| result.ok())
                            .map(|page| {
                                view! {
                                    <For
                                        each=move || page.nodes.clone()
                                        key=|website| website.id.clone()
                                        let:website
                                    >
                                        <WebsiteCard website=website />
                                    </For>
                                }
                            })
                    })}
                </Suspense>

                <a class="btn btn-block ml-auto mr-auto mt-4" href="/websites">
                    {t!(i18n, shared.view_more)}
                </a>
            </section>
        </Page>
    }
}
