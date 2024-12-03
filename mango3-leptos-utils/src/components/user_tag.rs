use leptos::{either::Either, prelude::*};

use crate::models::UserPreviewResp;

#[component]
pub fn UserTag(#[prop(into)] user: UserPreviewResp) -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center">
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
            }} <div class="text-left">
                <div class="mb-1">{user.display_name}</div>
                <div class="font-bold">"@"{user.username}</div>
            </div>
        </div>
    }
}
