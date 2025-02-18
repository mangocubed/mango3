use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::context::param_query;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::Page;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::{SearchPostsTab, SearchWebsitesTab};

#[component]
pub fn SearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let i18n = use_i18n();
    let active_tab = Memo::new(move |_| query_map.with(|params| params.get("tab")).unwrap_or_default());
    let text_title = async_t_string!(i18n, shared.search_results_for, query = param_query(query_map)).to_signal();

    view! {
        <Page title=text_title>
            <h1 class="h2">{move || text_title.get()}</h1>

            <section class="max-w-[720px] w-full mx-auto">
                <div role="tablist" class="tabs tabs-border mb-5">
                    <a
                        role="tab"
                        class="tab"
                        class:tab-active=move || active_tab.get() != "websites"
                        href=move || format!("/search?q={}", param_query(query_map))
                    >
                        {t!(i18n, shared.posts)}
                    </a>
                    <a
                        role="tab"
                        class="tab"
                        class:tab-active=move || active_tab.get() == "websites"
                        href=move || format!("/search?q={}&tab=websites", param_query(query_map))
                    >
                        {t!(i18n, home.websites)}
                    </a>
                </div>

                <div>
                    {move || {
                        if active_tab.get() != "websites" {
                            Either::Left(view! { <SearchPostsTab params_map=query_map /> })
                        } else {
                            Either::Right(view! { <SearchWebsitesTab params_map=query_map /> })
                        }
                    }}
                </div>

            </section>
        </Page>
    }
}
