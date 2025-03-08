use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

use mango3_leptos_utils::components::forms::{FormErrorAlert, FormSuccessModal};
use mango3_leptos_utils::i18n::{t, use_i18n};

use crate::components::PostFormFields;
use crate::context::param_website_id;
use crate::server_functions::AttemptToCreatePost;

#[component]
pub fn NewPostPage() -> impl IntoView {
    let navigate = use_navigate();
    let params_map = use_params_map();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToCreatePost>::new();
    let action_value = server_action.value();

    view! {
        <h2 class="h2">{t!(i18n, studio.new_post)}</h2>

        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form max-w-5xl">
            <FormErrorAlert action_value=action_value message=move || t!(i18n, studio.failed_to_create_post) />

            <PostFormFields
                action_value=action_value
                is_loading=server_action.pending()
                website_id=move || param_website_id(params_map).unwrap_or_default()
            />
        </ActionForm>

        <FormSuccessModal
            action_value=action_value
            message=move || t!(i18n, studio.post_created_successfully)
            on_close=move || {
                navigate(
                    &format!("/websites/{}/posts", param_website_id(params_map).unwrap_or_default()),
                    Default::default(),
                );
            }
        />
    }
}
