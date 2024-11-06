use leptos::prelude::*;
use leptos::text_prop::TextProp;

use crate::components::{RequireAuthentication, RequireNoAuthentication};
use crate::context::{use_current_user_resource, use_page_title};

mod not_found_page;

pub use not_found_page::NotFoundPage;

#[component]
pub fn AuthenticatedPage(children: Children, #[prop(into)] title: TextProp) -> impl IntoView {
    view! {
        <RequireAuthentication />

        <Page title=title>{children()}</Page>
    }
}

#[component]
pub fn GuestPage(children: Children, #[prop(into)] title: TextProp) -> impl IntoView {
    view! {
        <RequireNoAuthentication />

        <Page title=title>{children()}</Page>
    }
}

#[component]
pub fn Page(children: Children, #[prop(into)] title: TextProp) -> impl IntoView {
    let current_user_resource = use_current_user_resource();
    let page_title = use_page_title();

    Effect::new(move || {
        current_user_resource.refetch();
        page_title.value.set(Some(title.get().to_string()));
    });

    children()
}
