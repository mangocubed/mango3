use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_web_utils::async_t_string;
use mango3_web_utils::components::forms::{
    CountryField, FormErrorAlert, FormSuccessModal, ImageUploadField, MarkdownEditorField, SubmitButton, TextField,
};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::pages::AuthenticatedPage;
use mango3_web_utils::utils::ToSignalTrait;

use crate::server_functions::{get_user_profile, AttemptToUpdateProfile};

#[component]
pub fn EditProfilePage() -> impl IntoView {
    let i18n = use_i18n();
    let user_profile_resource = Resource::new_blocking(|| (), |_| get_user_profile());
    let server_action = ServerAction::<AttemptToUpdateProfile>::new();
    let action_value = server_action.value();
    let title = async_t_string!(i18n, my_account.edit_profile).to_signal();

    view! {
        <AuthenticatedPage title=title>
            <h1 class="h1">{move || title.get()}</h1>

            <Suspense>
                {move || Suspend::new(async move {
                    user_profile_resource
                        .get()
                        .and_then(|result| result.ok().unwrap_or_default())
                        .map(|user_profile| {
                            let navigate = use_navigate();
                            let value_display_name = RwSignal::new(user_profile.display_name);
                            let value_full_name = RwSignal::new(user_profile.full_name);
                            let value_birthdate = RwSignal::new(user_profile.birthdate);
                            let value_country_alpha2 = RwSignal::new(user_profile.country_alpha2);
                            let value_bio = RwSignal::new(user_profile.bio);
                            let value_avatar_image_blob = RwSignal::new(user_profile.avatar_image_blob);
                            view! {
                                <ActionForm
                                    action=server_action
                                    attr:autocomplete="off"
                                    attr:novalidate="true"
                                    attr:class="form"
                                >
                                    <FormErrorAlert
                                        action_value=action_value
                                        message=move || t!(i18n, my_account.failed_to_update_profile)
                                    />

                                    <TextField
                                        action_value=action_value
                                        id="display_name"
                                        label=move || t!(i18n, my_account.display_name)
                                        name="display_name"
                                        value=value_display_name
                                    />

                                    <TextField
                                        action_value=action_value
                                        id="full_name"
                                        label=move || t!(i18n, shared.full_name)
                                        name="full_name"
                                        value=value_full_name
                                    />

                                    <TextField
                                        action_value=action_value
                                        id="birthdate"
                                        label=move || t!(i18n, shared.birthdate)
                                        name="birthdate"
                                        input_type="date"
                                        value=value_birthdate
                                    />

                                    <CountryField
                                        action_value=action_value
                                        id="country_alpha2"
                                        label=move || t!(i18n, shared.country)
                                        name="country_alpha2"
                                        value=value_country_alpha2
                                    />

                                    <MarkdownEditorField
                                        action_value=action_value
                                        id="bio"
                                        label=move || t!(i18n, my_account.bio)
                                        name="bio"
                                        value=value_bio
                                    />

                                    <ImageUploadField
                                        label=move || t!(i18n, my_account.avatar_image)
                                        id="avatar_image_blob_id"
                                        name="avatar_image_blob_id"
                                        value=value_avatar_image_blob
                                    />

                                    <SubmitButton is_loading=server_action.pending() />
                                </ActionForm>

                                <FormSuccessModal
                                    action_value=action_value
                                    message=move || t!(i18n, my_account.profile_updated_successfully)
                                    on_close=move || navigate("/", Default::default())
                                />
                            }
                        })
                })}
            </Suspense>

        </AuthenticatedPage>
    }
}
