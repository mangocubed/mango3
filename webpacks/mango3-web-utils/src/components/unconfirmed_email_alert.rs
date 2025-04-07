use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};

use crate::components::CurrentUser;
use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};
use crate::icons::ExclamationOutlined;

#[component]
pub fn UnconfirmedEmailAlert() -> impl IntoView {
    view! {
        <CurrentUser let:user>
            <Show when=move || {
                !user.email_is_confirmed
            }>
                {move || {
                    let i18n = use_i18n();
                    let basic_config = use_basic_config();
                    let (is_hidden, set_is_hidden) = use_cookie_with_options::<
                        bool,
                        FromToStringCodec,
                    >(
                        "_mango3_hide_unconfirmed_email_alert",
                        UseCookieOptions::default()
                            .max_age(3600000)
                            .domain(basic_config.domain.clone())
                            .path("/")
                            .same_site(SameSite::Strict),
                    );
                    let edit_email_url = format!("{}edit-email", basic_config.my_account_url);
                    let edit_email_url_label = edit_email_url
                        .clone()
                        .split("://")
                        .last()
                        .expect("Could not get edit email label")
                        .to_owned();

                    view! {
                        <div
                            role="alert"
                            class="alert alert-warning mx-3 mt-3"
                            class:hidden=move || is_hidden.get().unwrap_or_default()
                        >
                            <ExclamationOutlined />

                            <span>
                                {t!(i18n, shared.you_should_go_to_the_following_link_to_confirm_your_email_address)}
                                ": " <a class="link font-bold" href=edit_email_url.clone()>
                                    {edit_email_url_label.clone()}
                                </a>
                            </span>

                            <button
                                class="btn btn-circle btn-ghost"
                                on:click=move |event| {
                                    event.prevent_default();
                                    set_is_hidden.set(Some(true));
                                }
                            >
                                "✕"
                            </button>
                        </div>
                    }
                }}
            </Show>
        </CurrentUser>
    }
}
