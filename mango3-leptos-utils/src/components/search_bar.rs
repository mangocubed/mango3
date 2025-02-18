use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate, use_query_map};

use crate::context::param_query;
use crate::i18n::use_i18n;
use crate::icons::MagnifyingGlassMini;

#[component]
pub fn SearchBar() -> impl IntoView {
    let i18n = use_i18n();
    let location = use_location();
    let navigate = use_navigate();
    let query_map = use_query_map();
    let query = RwSignal::new(String::new());
    let action = Action::new(move |query: &String| {
        let navigate = navigate.clone();
        let query = query.clone();
        async move {
            let q = query.trim();

            if q.is_empty() {
                return;
            }

            navigate(&format!("/search?q={q}"), Default::default());
        }
    });

    Effect::new(move || {
        if location.pathname.get() == "/search" {
            query.set(param_query(query_map));
        }
    });

    view! {
        <form
            class="p-0"
            on:submit=move |event| {
                event.prevent_default();
                action.dispatch(query.get());
            }
        >
            <label class="input flex items-center gap-2 h-9 pr-2">
                <input
                    class="grow"
                    type="search"
                    placeholder=move || async_t_string!(i18n, home.search).get()
                    bind:value=query
                />
                <button class="btn btn-ghost p-0 min-h-7 h-7 w-7" type="submit">
                    <MagnifyingGlassMini />
                </button>
            </label>
        </form>
    }
}
