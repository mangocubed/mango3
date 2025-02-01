use chrono::{DateTime, Utc};
use leptos::prelude::*;

use crate::components::TimeAgo;
use crate::i18n::{t, t_string, use_i18n};

#[component]
pub fn PostBottomBar(
    comments_count: i64,
    reactions_count: i64,
    views_count: i64,
    created_at: DateTime<Utc>,
    modified_at: Option<DateTime<Utc>>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="mt-4 opacity-70 flex gap-4">
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
                    if reactions_count == 1 {
                        t_string!(i18n, shared.one_reaction).to_owned()
                    } else {
                        t_string!(i18n, shared.count_reactions, count = reactions_count)
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

            <div class="flex-1 text-right">
                <TimeAgo value=created_at />

                {move || {
                    modified_at
                        .map(|modified_at| {
                            view! {
                                " ("
                                {t!(i18n, shared.edited)}
                                <span class="hidden md:inline">" "<TimeAgo value=modified_at /></span>
                                ")"
                            }
                        })
                }}
            </div>
        </div>
    }
}
