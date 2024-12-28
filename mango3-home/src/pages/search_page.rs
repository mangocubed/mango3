use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use mango3_leptos_utils::context::param_query;
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::pages::Page;

use crate::components::{SearchPostsTab, SearchWebsitesTab};

#[component]
pub fn SearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let i18n = use_i18n();
    let active_tab = Memo::new(move |_| query_map.with(|params| params.get("tab")).unwrap_or_default());

    let title = move || t_string!(i18n, shared.search_results_for, query = param_query(query_map));

    view! {
        <Page title=title>
            <h1 class="h2">{title}</h1>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <div role="tablist" class="tabs tabs-bordered mb-5">
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
