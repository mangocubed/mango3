use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{RequireAuthentication, RequireNoAuthentication};
use crate::context::use_current_user_resource;

mod not_found_page;

pub use not_found_page::NotFoundPage;

#[component]
pub fn AuthenticatedPage(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into)] title: Signal<String>,
) -> impl IntoView {
    view! {
        <RequireAuthentication />

        <Page class=class title=title>
            {children()}
        </Page>
    }
}

#[component]
pub fn GuestPage(children: Children, #[prop(into)] title: Signal<&'static str>) -> impl IntoView {
    view! {
        <RequireNoAuthentication />

        <Page title=title>{children()}</Page>
    }
}

#[component]
pub fn Page(
    children: Children,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into)] title: Signal<String>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let current_user_resource = use_current_user_resource();

    Effect::new(move || {
        current_user_resource.refetch();
    });

    view! {
        <Title text=move || title.get() />

        <div id=id class=class>
            {children()}
        </div>
    }
}
