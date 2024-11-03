use leptos::prelude::*;
use leptos::text_prop::TextProp;

use crate::context::use_page_title;

mod not_found_page;

pub use not_found_page::NotFoundPage;

#[component]
pub fn Page(children: Children, #[prop(into)] title: TextProp) -> impl IntoView {
    let page_title = use_page_title();

    Effect::new(move || {
        page_title.value.set(Some(title.get().to_string()));
    });

    children()
}
