use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton};
use mango3_leptos_utils::i18n::use_i18n;

use crate::components::PostFormFields;
use crate::context::use_website_id_param;
use crate::server_functions::AttemptToCreatePost;

#[component]
pub fn NewPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let website_id = use_website_id_param();
    let server_action = ServerAction::<AttemptToCreatePost>::new();
    let action_value = server_action.value();

    view! {
        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
            <ActionFormAlert
                action_value=action_value
                error_message=move || { t_string!(i18n, studio.failed_to_create_post) }
                redirect_to=format!("/websites/{}/posts", &website_id)
                success_message=move || { t_string!(i18n, studio.post_created_successfully) }
            />

            <input type="hidden" name="website_id" value=website_id />

            <PostFormFields action_value=action_value />

            <SubmitButton is_loading=server_action.pending() />
        </ActionForm>
    }
}