use leptos::prelude::*;

use mango3_utils::models::Hashtag;

#[component]
pub fn Hashtags(
    #[prop(into)] hashtags: Vec<Hashtag>,
    #[prop(default = "/".to_owned())] base_url: String,
) -> impl IntoView {
    view! {
        <For each=move || hashtags.clone() key=|hashtag| hashtag.id let:hashtag>
            <a class="btn btn-sm btn-outline btn-accent" href=format!("{}hashtags/{}", base_url, hashtag.name)>
                "#"
                {hashtag.name.clone()}
            </a>
        </For>
    }
}
