use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::*;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::server_functions::{get_user_profile, AttemptToUpdateProfile};

#[component]
pub fn EditProfilePage() -> impl IntoView {
    let i18n = use_i18n();
    let user_profile_resource = Resource::new_blocking(|| (), |_| get_user_profile());
    let server_action = ServerAction::<AttemptToUpdateProfile>::new();
    let action_value = server_action.value();
    let error_display_name = RwSignal::new(None);
    let error_full_name = RwSignal::new(None);
    let error_birthdate = RwSignal::new(None);
    let error_country_alpha2 = RwSignal::new(None);
    let error_bio = RwSignal::new(None);
    let title = async_t_string!(i18n, my_account.edit_profile).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_display_name.set(response.error("display-name"));
        error_full_name.set(response.error("full-name"));
        error_birthdate.set(response.error("birthdate"));
        error_country_alpha2.set(response.error("country-alpha2"));
        error_bio.set(response.error("bio"));
    });

    view! {
        <AuthenticatedPage title=title>
            <h2 class="h2">{move || title.get()}</h2>

            <Suspense>
                {move || Suspend::new(async move {
                    user_profile_resource
                        .get()
                        .and_then(|result| result.ok().unwrap_or_default())
                        .map(|user_profile| {
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
                                    <ActionFormAlert
                                        action_value=action_value
                                        error_message=move || t!(i18n, my_account.failed_to_update_profile)
                                        redirect_to="/"
                                        success_message=move || t!(i18n, my_account.profile_updated_successfully)
                                    />

                                    <TextField
                                        label=move || t!(i18n, my_account.display_name)
                                        name="display_name"
                                        error=error_display_name
                                        value=value_display_name
                                    />

                                    <TextField
                                        label=move || t!(i18n, shared.full_name)
                                        name="full_name"
                                        error=error_full_name
                                        value=value_full_name
                                    />

                                    <TextField
                                        label=move || t!(i18n, shared.birthdate)
                                        name="birthdate"
                                        input_type="date"
                                        error=error_birthdate
                                        value=value_birthdate
                                    />

                                    <CountryField
                                        label=move || t!(i18n, shared.country)
                                        name="country_alpha2"
                                        error=error_country_alpha2
                                        value=value_country_alpha2
                                    />

                                    <TextareaField
                                        label=move || t!(i18n, my_account.bio)
                                        name="bio"
                                        error=error_bio
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
                            }
                        })
                })}
            </Suspense>

        </AuthenticatedPage>
    }
}
