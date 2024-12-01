use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::Title;

use crate::components::{RequireAuthentication, RequireNoAuthentication};
use crate::context::use_current_user_resource;

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
pub fn Page(
    children: Children,
    #[prop(into)] title: TextProp,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let current_user_resource = use_current_user_resource();

    Effect::new(move || {
        current_user_resource.refetch();
    });

    view! {
        <Title text=title />

        <div class=class>{children()}</div>
    }
}
