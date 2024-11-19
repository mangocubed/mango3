use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::ActionFormResp;

use crate::components::MyWebsiteResource;
use crate::server_functions::AttemptToUpdateWebsite;

#[component]
pub fn EditPage() -> impl IntoView {
    let server_action = ServerAction::<AttemptToUpdateWebsite>::new();
    let action_value = server_action.value();
    let error_name = RwSignal::new(None);
    let error_description = RwSignal::new(None);
    let error_publish = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_name.set(response.error("name"));
        error_description.set(response.error("description"));
        error_publish.set(response.error("publish"));
    });

    view! {
        <MyWebsiteResource children=move |website| {
            website
                .map(|website| {
                    let i18n = use_i18n();
                    let value_name = RwSignal::new(website.name);
                    let value_description = RwSignal::new(website.description);
                    let value_icon_image_blob = RwSignal::new(website.icon_image_blob);
                    let value_cover_image_blob = RwSignal::new(website.cover_image_blob);
                    let value_publish = RwSignal::new(website.is_published);
                    view! {
                        <ActionForm
                            action=server_action
                            attr:autocomplete="off"
                            attr:novalidate="true"
                            attr:class="form"
                        >
                            <ActionFormAlert
                                action_value=action_value
                                error_message=move || { t_string!(i18n, studio.failed_to_update_website) }
                                redirect_to="/"
                                success_message=move || { t_string!(i18n, studio.website_updated_successfully) }
                            />

                            <input type="hidden" name="id" value=website.id />

                            <TextField
                                label=move || t_string!(i18n, studio.name)
                                name="name"
                                error=error_name
                                value=value_name
                            />

                            <TextareaField
                                label=move || t_string!(i18n, studio.description)
                                name="description"
                                error=error_description
                                value=value_description
                            />

                            <ImageUploadField
                                label=move || t_string!(i18n, studio.icon_image)
                                name="icon_image_blob_id"
                                value=value_icon_image_blob
                            />

                            <ImageUploadField
                                label=move || t_string!(i18n, studio.cover_image)
                                name="cover_image_blob_id"
                                width=288
                                value=value_cover_image_blob
                            />

                            <SwitchField
                                label=move || t_string!(i18n, studio.publish)
                                name="publish"
                                error=error_publish
                                is_checked=value_publish
                            />

                            <SubmitButton is_loading=server_action.pending() />
                        </ActionForm>
                    }
                })
        } />
    }
}
