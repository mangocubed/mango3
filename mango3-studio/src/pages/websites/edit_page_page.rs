use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_router::hooks::use_params_map;
use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton};

use crate::components::PageFormFields;
use crate::constants::KEY_PARAM_PAGE_ID;
use crate::context::use_website_id_param;
use crate::server_functions::{get_my_page, AttemptToUpdatePage};
use mango3_leptos_utils::i18n::use_i18n;

#[component]
pub fn EditPagePage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let website_id = use_website_id_param();
    let website_id_clone = website_id.clone();
    let page_id = params_map.with(|params| params.get(KEY_PARAM_PAGE_ID).unwrap_or_default());
    let server_action = ServerAction::<AttemptToUpdatePage>::new();
    let action_value = server_action.value();
    let page_resource = Resource::new_blocking(
        move || (website_id_clone.clone(), page_id.clone()),
        |(website_id, id)| async { get_my_page(website_id, id).await },
    );

    view! {
        <Suspense>
            {move || {
                let website_id = website_id.clone();
                Suspend::new(async move {
                    if let Some(Ok(Some(page))) = page_resource.get() {
                        Either::Left(
                            view! {
                                <ActionForm
                                    action=server_action
                                    attr:autocomplete="off"
                                    attr:novalidate="true"
                                    attr:class="form"
                                >
                                    <ActionFormAlert
                                        action_value=action_value
                                        error_message=move || { t_string!(i18n, studio.failed_to_update_page) }
                                        redirect_to=format!("/websites/{}/pages", &website_id)
                                        success_message=move || { t_string!(i18n, studio.page_updated_successfully) }
                                    />

                                    <input type="hidden" name="website_id" value=website_id />

                                    <input type="hidden" name="id" value=page.id.clone() />

                                    <PageFormFields action_value=action_value page=page />

                                    <SubmitButton is_loading=server_action.pending() />
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
