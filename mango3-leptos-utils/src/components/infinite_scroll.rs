use std::hash::Hash;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_element_visibility;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::CursorPageResp;

use super::LoadingSpinner;

#[component]
pub fn InfiniteScroll<T, IV, F, K, KF>(
    after: RwSignal<Option<String>>,
    children: F,
    key: KF,
    #[prop(into, optional)] nodes: RwSignal<Vec<T>>,
    #[prop(into)] resource: Resource<Result<CursorPageResp<T>, ServerFnError>>,
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
    let end_cursor = RwSignal::new(None);
    let has_next_page = RwSignal::new(false);
    let is_loading = RwSignal::new(true);

    Effect::new(move || {
        if let Some(Ok(mut page)) = resource.get() {
            end_cursor.set(page.end_cursor);
            has_next_page.set(page.has_next_page);
            nodes.update(|n| {
                n.append(&mut page.nodes);
            });
            is_loading.set(false);
        }
    });

    Effect::new(move || {
        if !bottom_is_visible.get() || is_loading.get() || !has_next_page.get() {
            return;
        }

        is_loading.set(true);

        after.set(end_cursor.get());
        resource.refetch();
    });

    Effect::new(move || {});

    view! {
        <For each=move || nodes.get() key=key let:data>
            {children(data)}
        </For>

        <div node_ref=node_ref>
            <Show when=move || is_loading.get()>
                <LoadingSpinner />
            </Show>
        </div>
    }
}
