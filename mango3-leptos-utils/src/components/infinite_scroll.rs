use std::hash::Hash;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_element_visibility;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::PageResp;

use super::LoadingSpinner;

#[component]
pub fn InfiniteScroll<T, IV, F, K, KF>(
    after: RwSignal<Option<String>>,
    children: F,
    key: KF,
    #[prop(into)] resource: Resource<Result<PageResp<T>, ServerFnError>>,
) -> impl IntoView
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
    IV: IntoView + 'static,
    F: Fn(T) -> IV + Clone + Send + Sync + 'static,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + Clone + Send + 'static,
{
    let node_ref = NodeRef::<Div>::new();
    let bottom_is_visible = use_element_visibility(node_ref);
    let full_page = RwSignal::new(PageResp::default());
    let is_loading = RwSignal::new(true);

    Effect::new(move || {
        if let Some(Ok(mut new_page)) = resource.get() {
            full_page.update(|fp| {
                fp.end_cursor = new_page.end_cursor;
                fp.has_next_page = new_page.has_next_page;
                fp.nodes.append(&mut new_page.nodes);
            });

            is_loading.set(false);
        }
    });

    Effect::new(move || {
        if !bottom_is_visible.get() || is_loading.get() || !full_page.get().has_next_page {
            return;
        }

        is_loading.set(true);

        after.set(full_page.get().end_cursor);
        resource.refetch();
    });

    view! {
        <For each=move || full_page.get().nodes key=key let:data>
            {children(data)}
        </For>

        <div node_ref=node_ref class="flex mt-4">
            <Show when=move || is_loading.get()>
                <LoadingSpinner />
            </Show>
        </div>
    }
}
