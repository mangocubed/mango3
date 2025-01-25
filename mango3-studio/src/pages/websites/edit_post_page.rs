use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::ActionFormAlert;
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};

use crate::components::PostFormFields;
use crate::constants::KEY_PARAM_POST_ID;
use crate::context::param_website_id;
use crate::server_functions::{get_my_post, AttemptToUpdatePost};

#[component]
pub fn EditPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let server_action = ServerAction::<AttemptToUpdatePost>::new();
    let action_value = server_action.value();
    let post_resource = Resource::new_blocking(
        move || {
            (
                param_website_id(params_map).unwrap_or_default(),
                params_map.with(|params| params.get(KEY_PARAM_POST_ID).unwrap_or_default()),
            )
        },
        |(website_id, id)| async { get_my_post(website_id, id).await },
    );

    view! {
        <Suspense>
            {move || {
                Suspend::new(async move {
                    if let Some(Ok(Some(post))) = post_resource.get() {
                        Either::Left(
                            view! {
                                <h2 class="h2">{t!(i18n, studio.edit_post)}</h2>

                                <ActionForm
                                    action=server_action
                                    attr:autocomplete="off"
                                    attr:novalidate="true"
                                    attr:class="form max-w-5xl"
                                >
                                    <ActionFormAlert
                                        action_value=action_value
                                        error_message=move || { t_string!(i18n, studio.failed_to_update_post) }
                                        redirect_to=format!(
                                            "/websites/{}/posts",
                                            param_website_id(params_map).unwrap_or_default(),
                                        )
                                        success_message=move || { t_string!(i18n, studio.post_updated_successfully) }
                                    />

                                    <input type="hidden" name="id" value=post.id.clone() />

                                    <PostFormFields
                                        action_value=action_value
                                        is_loading=server_action.pending()
                                        website_id=move || param_website_id(params_map).unwrap_or_default()
                                        post=post
                                    />
                                </ActionForm>
                            },
                        )
                    } else {
                        Either::Right(())
                    }
                })
            }}
        </Suspense>
    }
}
