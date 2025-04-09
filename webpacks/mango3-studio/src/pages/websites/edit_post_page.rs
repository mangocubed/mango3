use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal};
use mango3_web_utils::i18n::{t, use_i18n};

use crate::components::{MyWebsitePageWrapper, PostFormFields};
use crate::constants::KEY_PARAM_POST_ID;
use crate::server_functions::{get_my_post, AttemptToUpdatePost};

#[component]
pub fn EditPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let server_action = ServerAction::<AttemptToUpdatePost>::new();
    let action_value = server_action.value();

    view! {
        <MyWebsitePageWrapper children=move |website| {
            let post_resource = LocalResource::new({
                let website_id = website.id;
                move || get_my_post(
                    website_id,
                    params_map.with(|params| params.get(KEY_PARAM_POST_ID).unwrap_or_default()),
                )
            });
            view! {
                <Suspense>
                    {move || {
                        let website_id = website.id;
                        Suspend::new(async move {
                            if let Some(Ok(Some(post))) = post_resource.get().map(|resource| resource.take()) {
                                let navigate = use_navigate();
                                Either::Left(
                                    view! {
                                        <h1 class="h1">{t!(i18n, studio.edit_post)}</h1>

                                        <ActionForm
                                            action=server_action
                                            attr:autocomplete="off"
                                            attr:novalidate="true"
                                            attr:class="form max-w-5xl"
                                        >
                                            <FormErrorAlert action_value=action_value />

                                            <input type="hidden" name="id" value=post.id.to_string() />

                                            <PostFormFields
                                                action_value=action_value
                                                is_loading=server_action.pending()
                                                website_id=website_id.to_string()
                                                post=post
                                            />
                                        </ActionForm>

                                        <FormSuccessModal
                                            action_value=action_value
                                            on_close=move || {
                                                navigate(
                                                    &format!("/websites/{}/posts", &website_id),
                                                    Default::default(),
                                                );
                                            }
                                        />
                                    },
                                )
                            } else {
                                Either::Right(())
                            }
                        })
                    }}
                </Suspense>
            }
        } />
    }
}
