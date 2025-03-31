use leptos::prelude::*;

use super::Page;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Page title="Error 404: Page not found">
            <h2 class="h2">"Whoops!"</h2>

            <h3 class="h3">"There is nothing here!"</h3>
        </Page>
    }
}
