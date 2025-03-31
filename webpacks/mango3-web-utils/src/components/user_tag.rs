use leptos::prelude::*;

use crate::i18n::{t, use_i18n};
use crate::presenters::UserMinPresenter;

#[component]
pub fn UserAvatar(
    #[prop(optional)] class: &'static str,
    #[prop(into)] user: UserMinPresenter,
    #[prop(default = 32)] size: u16,
) -> impl IntoView {
    view! {
        <div class=format!("avatar {class}")>
            <div class="rounded-full" style:width=format!("{size}px") style:height=format!("{size}px")>
                <img alt=user.initials.clone() src=user.avatar_image_url(size).to_string() />
            </div>
        </div>
    }
}

#[component]
pub fn UserLabels(#[prop(into)] user: UserMinPresenter) -> impl IntoView {
    let i18n = use_i18n();

    let user_role = user.role.clone();

    let is_admin = move || user_role != "user";
    let is_disabled = move || user.is_disabled;

    view! {
        <Show when=is_admin>
            <a class="btn btn-sm btn-outline btn-primary no-animation">{user.role.clone()}</a>
        </Show>

        <Show when=is_disabled>
            <a class="btn btn-sm btn-outline btn-error no-animation">{t!(i18n, shared.disabled)}</a>
        </Show>
    }
}

#[component]
pub fn UserTag(
    #[prop(into)] user: UserMinPresenter,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] text_class: &'static str,
    #[prop(default = 32)] size: u16,
) -> impl IntoView {
    view! {
        <div class=format!("flex gap-2 items-center {class}") class:opacity-50=move || user.is_disabled>
            <UserAvatar user=user.clone() size=size />

            <div class=format!("text-left ml-2 {text_class}")>
                <div class="mb-1 font-bold">{user.display_name}</div>
                <div class="opacity-70">"@"{user.username}</div>
            </div>
        </div>
    }
}

#[component]
pub fn UserTagLink(#[prop(into)] user: UserMinPresenter, #[prop(optional)] text_class: &'static str) -> impl IntoView {
    let href = if !user.is_disabled {
        Some(user.url.to_string())
    } else {
        None
    };

    view! {
        <a href=href title=format!("@{}", user.username)>
            <UserTag user=user.clone() text_class=text_class />
        </a>
    }
}
