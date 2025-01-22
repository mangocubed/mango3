use leptos::prelude::*;

use crate::models::HashtagResp;

#[component]
pub fn Hashtags(
    #[prop(into)] hashtags: Vec<HashtagResp>,
    #[prop(default = "/".to_owned())] base_url: String,
) -> impl IntoView {
    view! {
        <For each=move || hashtags.clone() key=|hashtag| hashtag.id.clone() let:hashtag>
            <a class="btn btn-sm btn-outline" href=format!("{}hashtags/{}", base_url, hashtag.name)>
                "#"
                {hashtag.name.clone()}
            </a>
        </For>
    }
}
