use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_element_visibility;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::i18n::{t, use_i18n};
use crate::models::CursorPageResp;

use super::LoadingSpinner;

pub trait InfiniteScrollControllerTrait<R, T>
where
    Self: Send + Sync,
{
    fn new<RF>(resource_fn: RF) -> Arc<Self>
    where
        RF: Fn(RwSignal<Option<String>>) -> R;

    fn after(&self) -> RwSignal<Option<String>>;

    fn end_cursor(&self) -> RwSignal<Option<String>>;

    fn has_next_page(&self) -> RwSignal<bool>;

    fn is_loading(&self) -> RwSignal<bool>;

    fn nodes(&self) -> RwSignal<Vec<T>>;

    fn resource_get(&self) -> Option<CursorPageResp<T>>;

    fn resource_refetch(&self);

    fn clear_and_refetch(&self);
}

pub struct InfiniteScrollLocalResourceController<T>
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
{
    after: RwSignal<Option<String>>,
    end_cursor: RwSignal<Option<String>>,
    has_next_page: RwSignal<bool>,
    is_loading: RwSignal<bool>,
    pub nodes: RwSignal<Vec<T>>,
    resource: LocalResource<Result<CursorPageResp<T>, ServerFnError>>,
}

impl<T> InfiniteScrollControllerTrait<LocalResource<Result<CursorPageResp<T>, ServerFnError>>, T>
    for InfiniteScrollLocalResourceController<T>
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
{
    fn new<RF>(resource_fn: RF) -> Arc<Self>
    where
        RF: Fn(RwSignal<Option<String>>) -> LocalResource<Result<CursorPageResp<T>, ServerFnError>>,
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

    fn after(&self) -> RwSignal<Option<String>> {
        self.after
    }

    fn end_cursor(&self) -> RwSignal<Option<String>> {
        self.end_cursor
    }

    fn has_next_page(&self) -> RwSignal<bool> {
        self.has_next_page
    }

    fn is_loading(&self) -> RwSignal<bool> {
        self.is_loading
    }

    fn nodes(&self) -> RwSignal<Vec<T>> {
        self.nodes
    }

    fn resource_get(&self) -> Option<CursorPageResp<T>> {
        let resource = self.resource.get()?;

        resource.as_ref().ok().cloned()
    }

    fn resource_refetch(&self) {
        self.resource.refetch();
    }

    fn clear_and_refetch(&self) {
        self.nodes.set(vec![]);
        self.after.set(None);
        self.end_cursor.set(None);
        self.is_loading.set(true);
        self.resource.refetch();
    }
}

pub struct InfiniteScrollResourceController<T>
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
{
    after: RwSignal<Option<String>>,
    end_cursor: RwSignal<Option<String>>,
    has_next_page: RwSignal<bool>,
    is_loading: RwSignal<bool>,
    pub nodes: RwSignal<Vec<T>>,
    resource: Resource<Result<CursorPageResp<T>, ServerFnError>>,
}

impl<T> InfiniteScrollControllerTrait<Resource<Result<CursorPageResp<T>, ServerFnError>>, T>
    for InfiniteScrollResourceController<T>
where
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
{
    fn new<RF>(resource_fn: RF) -> Arc<Self>
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

    fn after(&self) -> RwSignal<Option<String>> {
        self.after
    }

    fn end_cursor(&self) -> RwSignal<Option<String>> {
        self.end_cursor
    }

    fn has_next_page(&self) -> RwSignal<bool> {
        self.has_next_page
    }

    fn is_loading(&self) -> RwSignal<bool> {
        self.is_loading
    }

    fn nodes(&self) -> RwSignal<Vec<T>> {
        self.nodes
    }

    fn resource_get(&self) -> Option<CursorPageResp<T>> {
        self.resource.get().and_then(|result| result.clone().ok())
    }

    fn resource_refetch(&self) {
        self.resource.refetch();
    }

    fn clear_and_refetch(&self) {
        self.nodes.set(vec![]);
        self.after.set(None);
        self.end_cursor.set(None);
        self.is_loading.set(true);
        self.resource.refetch();
    }
}

#[component]
pub fn InfiniteScroll<T, R, CT, IV, CF, K, KF>(
    controller: Arc<CT>,
    children: CF,
    key: KF,
    #[prop(default = PhantomData)] _marker: PhantomData<(T, R, IV, K)>,
) -> impl IntoView
where
    R: Send + Sync + 'static,
    T: Clone + DeserializeOwned + Send + Serialize + Sync + 'static,
    CT: InfiniteScrollControllerTrait<R, T> + 'static,
    IV: IntoView + 'static,
    CF: Fn(T) -> IV + Clone + Send + Sync + 'static,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + Clone + Send + Sync + 'static,
{
    let i18n = use_i18n();
    let node_ref = NodeRef::<Div>::new();
    let bottom_is_visible = use_element_visibility(node_ref);

    Effect::new({
        let controller = controller.clone();
        move || {
            if let Some(mut page) = controller.resource_get() {
                controller.end_cursor().set(page.end_cursor);
                controller.has_next_page().set(page.has_next_page);
                controller.nodes().update(|nodes| {
                    nodes.append(&mut page.nodes);
                });
                controller.is_loading().set(false);
            }
        }
    });

    Effect::new({
        let controller = controller.clone();
        move || {
            if !bottom_is_visible.get() || controller.is_loading().get() || !controller.has_next_page().get() {
                return;
            }

            controller.is_loading().set(true);

            controller.after().set(controller.end_cursor().get());
            controller.resource_refetch();
        }
    });

    view! {
        <div>
            <For
                each={
                    let controller = controller.clone();
                    move || controller.clone().nodes().get()
                }
                key=key.clone()
                let:data
            >
                {children(data)}
            </For>
        </div>

        <Show when={
            let controller = controller.clone();
            move || !controller.is_loading().get() && controller.nodes().get().is_empty()
        }>
            <div class="text-center text-gray-500">{t!(i18n, shared.no_results_found)}</div>
        </Show>

        <div node_ref=node_ref>
            <Show when=move || controller.is_loading().get()>
                <LoadingSpinner />
            </Show>
        </div>
    }
}
