use leptos::{either::Either, prelude::*};

use crate::models::UserPreviewResp;

#[component]
pub fn UserTag(
    #[prop(into)] user: UserPreviewResp,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] text_class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex gap-2 items-center {class}",
        )>
            {move || {
                if let Some(avatar_image_blob) = &user.avatar_image_blob {
                    Either::Left(
                        view! {
                            <div class="avatar">
                                <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                    <img alt=user.initials.clone() src=avatar_image_blob.variant_url(32, 32, true) />
                                </div>
                            </div>
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <div class="avatar placeholder">
                                <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                    <span class="text-xs">{user.initials.clone()}</span>
                                </div>
                            </div>
                        },
                    )
                }
            }} <div class=format!("text-left ml-2 {text_class}")>
                <div class="mb-1 font-bold">{user.display_name}</div>
                <div class="opacity-70">"@"{user.username}</div>
            </div>
        </div>
    }
}
