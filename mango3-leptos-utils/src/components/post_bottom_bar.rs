use leptos::prelude::*;

use crate::i18n::{t_string, use_i18n};

#[component]
pub fn PostBottomBar(comments_count: i64, views_count: i64) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="mt-4 opacity-70 flex gap-3">
            <div>
                {move || {
                    if views_count == 1 {
                        t_string!(i18n, shared.one_view).to_owned()
                    } else {
                        t_string!(i18n, shared.count_views, count = views_count)
                    }
                }}
            </div>

            <div>
                {move || {
                    if comments_count == 1 {
                        t_string!(i18n, shared.one_comment).to_owned()
                    } else {
                        t_string!(i18n, shared.count_comments, count = comments_count)
                    }
                }}
            </div>
        </div>
    }
}
