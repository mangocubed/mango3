use leptos::prelude::*;

use crate::models::UserPreviewResp;

#[component]
pub fn UserIcon(#[prop(into)] user: UserPreviewResp, #[prop(default = 32)] size: u16) -> impl IntoView {
    view! {
        <div class="avatar">
            <div class="rounded-full" style:width=format!("{size}px") style:height=format!("{size}px")>
                <img alt=user.initials.clone() src=user.avatar_image_url(size) />
            </div>
        </div>
    }
}

#[component]
pub fn UserTag(
    #[prop(into)] user: UserPreviewResp,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] text_class: &'static str,
    #[prop(default = 32)] size: u16,
) -> impl IntoView {
    view! {
        <div class=format!("flex gap-2 items-center {class}")>
            <UserIcon user=user.clone() size=size />

            <div class=format!("text-left ml-2 {text_class}")>
                <div class="mb-1 font-bold">{user.display_name}</div>
                <div class="opacity-70">"@"{user.username}</div>
            </div>
        </div>
    }
}

#[component]
pub fn UserTagLink(#[prop(into)] user: UserPreviewResp, #[prop(optional)] text_class: &'static str) -> impl IntoView {
    view! {
        <a href=user.url title=format!("@{}", user.username)>
            <UserTag user=user.clone() text_class=text_class />
        </a>
    }
}
