use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal};
use mango3_web_utils::i18n::{t, use_i18n};

use crate::components::{MyWebsitePageWrapper, PostFormFields};
use crate::server_functions::AttemptToCreatePost;

#[component]
pub fn NewPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToCreatePost>::new();
    let action_value = server_action.value();

    view! {
        <MyWebsitePageWrapper children=move |website| {
            let navigate = use_navigate();
            let website_id = website.id;
            view! {
                <h1 class="h1">{t!(i18n, studio.new_post)}</h1>

                <ActionForm
                    action=server_action
                    attr:autocomplete="off"
                    attr:novalidate="true"
                    attr:class="form max-w-5xl"
                >
                    <FormErrorAlert action_value=action_value />

                    <PostFormFields
                        action_value=action_value
                        is_loading=server_action.pending()
                        website_id=website_id.to_string()
                    />
                </ActionForm>

                <FormSuccessModal
                    action_value=action_value
                    on_close=move || {
                        navigate(&format!("/websites/{}/posts", &website_id), Default::default());
                    }
                />
            }
        } />
    }
}
