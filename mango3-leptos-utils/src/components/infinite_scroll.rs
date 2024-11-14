use std::hash::Hash;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_element_visibility;

#[component]
pub fn InfiniteScroll<T, IV, F, K, KF, MF>(
    children: F,
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(into)] items: Signal<Vec<T>>,
    key: KF,
    on_load_more: MF,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
    F: Fn(T) -> IV + Clone + Send + Sync + 'static,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + Clone + Send + 'static,
    MF: Fn(Option<&T>) + 'static,
{
    let node_ref = NodeRef::<Div>::new();
    let bottom_is_visible = use_element_visibility(node_ref);

    Effect::new(move || {
        if bottom_is_visible.get() {
            on_load_more(items.get().last());
        }
    });

    view! {
        <For each=move || items.get() key=key let:data>
            {children(data)}
        </For>

        <div node_ref=node_ref class="flex mt-4">
            <Show when=move || is_loading.get()>
                <span class="loading loading-spinner loading-lg m-auto"></span>
            </Show>
        </div>
    }
}
