use std::hash::Hash;
use std::sync::Arc;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_element_visibility;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::i18n::{t, use_i18n};
use crate::models::CursorPageResp;

use super::LoadingSpinner;

pub struct InfiniteScrollController<T>
where
    T: DeserializeOwned + Send + Serialize + Sync + 'static,
{
    after: RwSignal<Option<String>>,
    end_cursor: RwSignal<Option<String>>,
    has_next_page: RwSignal<bool>,
    is_loading: RwSignal<bool>,
    pub nodes: RwSignal<Vec<T>>,
    resource: Resource<Result<CursorPageResp<T>, ServerFnError>>,
}

impl<T> InfiniteScrollController<T>
where
    T: DeserializeOwned + Send + Serialize + Sync + 'static,
{
    pub fn new<RF>(resource_fn: RF) -> Arc<Self>
    where
        RF: Fn(RwSignal<Option<String>>) -> Resource<Result<CursorPageResp<T>, ServerFnError>>,
    {
        let after = RwSignal::new(None);
        Arc::new(Self {
            after,
            end_cursor: RwSignal::new(None),
            has_next_page: RwSignal::new(false),
            is_loading: RwSignal::new(true),
            nodes: RwSignal::new(vec![]),
            resource: resource_fn(after),
        })
    }

    pub fn clear_and_refetch(&self) {
        self.nodes.set(vec![]);
        self.after.set(None);
        self.end_cursor.set(None);
        self.is_loading.set(true);
        self.resource.refetch();
    }
}

#[component]
pub fn InfiniteScroll<T, IV, CF, K, KF>(
    #[prop(into)] controller: Arc<InfiniteScrollController<T>>,
    children: CF,
    key: KF,
) -> impl IntoView
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
    IV: IntoView + 'static,
    CF: Fn(T) -> IV + Clone + Send + Sync + 'static,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + Clone + Send + 'static,
{
    let i18n = use_i18n();
    let node_ref = NodeRef::<Div>::new();
    let bottom_is_visible = use_element_visibility(node_ref);

    Effect::new({
        let controller = controller.clone();
        move || {
            if let Some(Ok(mut page)) = controller.resource.get().clone() {
                controller.end_cursor.set(page.end_cursor);
                controller.has_next_page.set(page.has_next_page);
                controller.nodes.update(|nodes| {
                    nodes.append(&mut page.nodes);
                });
                controller.is_loading.set(false);
            }
        }
    });

    Effect::new({
        let controller = controller.clone();
        move || {
            if !bottom_is_visible.get() || controller.is_loading.get() || !controller.has_next_page.get() {
                return;
            }

            controller.is_loading.set(true);

            controller.after.set(controller.end_cursor.get());
            controller.resource.refetch();
        }
    });

    view! {
        <div>
            <For
                each={
                    let controller = controller.clone();
                    move || controller.clone().nodes.get()
                }
                key=key.clone()
                let:data
            >
                {children(data)}
            </For>
        </div>

        <Show when={
            let controller = controller.clone();
            move || !controller.is_loading.get() && controller.nodes.get().is_empty()
        }>
            <div class="text-center text-gray-500">{t!(i18n, shared.no_results_found)}</div>
        </Show>

        <div node_ref=node_ref>
            <Show when=move || controller.is_loading.get()>
                <LoadingSpinner />
            </Show>
        </div>
    }
}
