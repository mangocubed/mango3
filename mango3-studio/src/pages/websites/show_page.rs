use leptos::prelude::*;

use crate::components::MyWebsitePageWrapper;

#[component]
pub fn ShowPage() -> impl IntoView {
    view! {
        <MyWebsitePageWrapper let:_>
            <div />
        </MyWebsitePageWrapper>
    }
}
