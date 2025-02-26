use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::{CurrentUser, LoadingSpinner};
use mango3_leptos_utils::context::{use_current_user_resource, use_info};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::PlusOutlined;

use crate::server_functions::{
    attempt_to_delete_post_reaction, attempt_to_insert_or_update_post_reaction, get_my_post_reaction_emoji,
    get_post_reaction_emojis_count,
};

#[component]
pub fn PostReactions(post_id: String) -> impl IntoView {
    let i18n = use_i18n();
    let post_id_store = StoredValue::new(post_id.clone());
    let my_emoji_resource =
        LocalResource::new(move || async move { get_my_post_reaction_emoji(post_id_store.read_value().clone()).await });
    let emojis_count_resource =
        LocalResource::new(
            move || async move { get_post_reaction_emojis_count(post_id_store.read_value().clone()).await },
        );

    view! {
        <section>
            <h2 class="h2 mt-5">{t!(i18n, websites.reactions)}</h2>

            <div class="flex flex-wrap gap-2 items-center">
                <Transition fallback=LoadingSpinner>
                    {move || Suspend::new({
                        async move {
                            emojis_count_resource
                                .get()
                                .and_then(|result| result.take().ok())
                                .map(|emojis_count| {
                                    let info = use_info();
                                    let insert_reaction_action = Action::new(move |emoji: &String| {
                                        let post_id = post_id_store.read_value().clone();
                                        let emoji = emoji.clone();
                                        async move {
                                            let _ = attempt_to_insert_or_update_post_reaction(post_id, emoji.to_owned())
                                                .await;
                                            emojis_count_resource.refetch();
                                            my_emoji_resource.refetch();
                                        }
                                    });
                                    let my_emoji = Memo::new(move |_| {
                                        my_emoji_resource
                                            .with(|emoji| emoji.clone().and_then(|result| result.take().ok()))
                                            .flatten()
                                    });
                                    view! {
                                        <CurrentUser let:_>
                                            <div class="dropdown">
                                                <button class="btn btn-ghost px-1">
                                                    <PlusOutlined />
                                                </button>

                                                <div class="dropdown-content bg-base-100 rounded-box z-[1] p-2 shadow flex flex-wrap w-[296px]">
                                                    <For
                                                        each={
                                                            let reaction_emojis = info.reaction_emojis.clone();
                                                            move || reaction_emojis.clone()
                                                        }
                                                        key=|emoji| emoji.to_owned()
                                                        let:emoji
                                                    >
                                                        <button
                                                            class="btn btn-ghost px-1 text-xl"
                                                            on:click=move |_| {
                                                                insert_reaction_action.dispatch(emoji.to_owned());
                                                            }
                                                        >
                                                            {emoji.clone()}
                                                        </button>
                                                    </For>
                                                </div>
                                            </div>
                                        </CurrentUser>

                                        {move || {
                                            let emojis_count_store = StoredValue::new(emojis_count.clone());
                                            if emojis_count_store.read_value().is_empty() {
                                                Either::Left(
                                                    view! {
                                                        <div class="text-center text-gray-500 flex-1 self-center">
                                                            {t!(i18n, websites.no_reactions_yet)}
                                                        </div>
                                                    },
                                                )
                                            } else {
                                                let can_insert_reaction = Memo::new(move |_| {
                                                    use_current_user_resource()
                                                        .get()
                                                        .and_then(|result| result.ok())
                                                        .flatten()
                                                        .is_some()
                                                });
                                                Either::Right(
                                                    view! {
                                                        {move || {
                                                            my_emoji
                                                                .get()
                                                                .map(|my_emoji| {
                                                                    let delete_reaction_action = Action::new(move |()| {
                                                                        let post_id = post_id_store.read_value().clone();
                                                                        async move {
                                                                            let _ = attempt_to_delete_post_reaction(post_id).await;
                                                                            emojis_count_resource.refetch();
                                                                            my_emoji_resource.refetch();
                                                                        }
                                                                    });
                                                                    let count = {
                                                                        let my_emoji = my_emoji.clone();
                                                                        move || {
                                                                            emojis_count_store
                                                                                .read_value()
                                                                                .iter()
                                                                                .find_map(|(emoji, count)| {
                                                                                    if *emoji == my_emoji.clone() { Some(count) } else { None }
                                                                                })
                                                                                .cloned()
                                                                                .unwrap_or(1)
                                                                        }
                                                                    };

                                                                    view! {
                                                                        <button
                                                                            class="btn btn-sm btn-outline px-1"
                                                                            on:click=move |_| {
                                                                                delete_reaction_action.dispatch(());
                                                                            }
                                                                        >
                                                                            <span class="text-xl">{my_emoji.clone()}</span>
                                                                            <div class="badge">{count}</div>
                                                                        </button>
                                                                    }
                                                                })
                                                        }}

                                                        <For
                                                            each=move || {
                                                                emojis_count_store
                                                                    .read_value()
                                                                    .iter()
                                                                    .filter(|(emoji, _)| {
                                                                        Some(emoji) != my_emoji.get().as_ref()
                                                                    })
                                                                    .cloned()
                                                                    .collect::<Vec<(String, i64)>>()
                                                            }
                                                            key=|(emoji, _)| emoji.clone()
                                                            children=move |(emoji, count)| {
                                                                view! {
                                                                    <button
                                                                        class="btn btn-ghost px-1"
                                                                        on:click=move |_| {
                                                                            if can_insert_reaction.get() {
                                                                                insert_reaction_action.dispatch(emoji.to_owned());
                                                                            }
                                                                        }
                                                                    >
                                                                        <span class="text-xl">{emoji.clone()}</span>
                                                                        <div class="badge">{count}</div>
                                                                    </button>
                                                                }
                                                            }
                                                        />
                                                    },
                                                )
                                            }
                                        }}
                                    }
                                })
                        }
                    })}
                </Transition>
            </div>
        </section>
    }
}
