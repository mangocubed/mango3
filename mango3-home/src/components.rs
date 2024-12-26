use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};

use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::icons::MagnifyingGlassMini;

use crate::context::param_query;

#[component]
pub fn SearchBar() -> impl IntoView {
    let i18n = use_i18n();
    let navigate = use_navigate();
    let query_map = use_query_map();
    let query = RwSignal::new(param_query(query_map));
    let action = Action::new(move |()| {
        let navigate = navigate.clone();
        async move {
            let q = query.get();
            let q = q.trim();

            if q.is_empty() {
                return;
            }

            navigate(&format!("/search?q={q}"), Default::default());
        }
    });

    Effect::new(move || {
        query.set(param_query(query_map));
    });

    view! {
        <form
            class="p-0"
            on:submit=move |event| {
                event.prevent_default();
                action.dispatch(());
            }
        >
            <label class="input input-bordered flex items-center gap-2 h-9 pr-2">
                <input class="grow" type="search" placeholder=move || t_string!(i18n, home.search) bind:value=query />
                <button class="btn btn-ghost p-0 min-h-7 h-7 w-7" type="submit">
                    <MagnifyingGlassMini />
                </button>
            </label>
        </form>
    }
}
