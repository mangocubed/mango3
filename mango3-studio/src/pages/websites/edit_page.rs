use leptos::prelude::*;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;

use crate::components::{MyWebsite, ThemeSelectorField};
use crate::server_functions::AttemptToUpdateWebsite;

const DARK_THEMES: [&str; 13] = [
    "dark",
    "aqua",
    "black",
    "business",
    "coffee",
    "dim",
    "dracula",
    "forest",
    "halloween",
    "luxury",
    "night",
    "sunset",
    "synthwave",
];
const LIGHT_THEMES: [&str; 19] = [
    "light",
    "acid",
    "autumn",
    "bumblebee",
    "cmyk",
    "corporate",
    "cupcake",
    "cyberpunk",
    "fantasy",
    "emerald",
    "garden",
    "lemonade",
    "lofi",
    "nord",
    "pastel",
    "retro",
    "valentine",
    "winter",
    "wireframe",
];

#[component]
pub fn EditPage() -> impl IntoView {
    let server_action = ServerAction::<AttemptToUpdateWebsite>::new();
    let action_value = server_action.value();
    let error_name = RwSignal::new(None);
    let error_description = RwSignal::new(None);
    let error_light_theme = RwSignal::new(None);
    let error_dark_theme = RwSignal::new(None);
    let error_publish = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_name.set(response.error("name"));
        error_description.set(response.error("description"));
        error_light_theme.set(response.error("light-theme"));
        error_dark_theme.set(response.error("dark-theme"));
        error_publish.set(response.error("publish"));
    });

    view! {
        <MyWebsite children=move |website| {
            let i18n = use_i18n();
            let value_name = RwSignal::new(website.name.clone());
            let value_description = RwSignal::new(website.description.clone());
            let value_icon_image_blob = RwSignal::new(website.icon_image_blob.clone());
            let value_cover_image_blob = RwSignal::new(website.cover_image_blob.clone());
            let value_publish = RwSignal::new(website.is_published);
            let value_light_theme = RwSignal::new(website.light_theme.clone());
            let value_dark_theme = RwSignal::new(website.dark_theme.clone());

            view! {
                <h2 class="h2">{t!(i18n, studio.edit)}</h2>

                <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                    <ActionFormAlert
                        action_value=action_value
                        error_message=move || t!(i18n, studio.failed_to_update_website)
                        redirect_to="/"
                        success_message=move || t!(i18n, studio.website_updated_successfully)
                    />

                    <input type="hidden" name="id" value=website.id.clone() />

                    <TextField label=move || t!(i18n, studio.name) name="name" error=error_name value=value_name />

                    <TextareaField
                        label=move || t!(i18n, studio.description)
                        name="description"
                        error=error_description
                        value=value_description
                    />

                    <ImageUploadField
                        label=move || t!(i18n, studio.icon_image)
                        id="icon_image_blob_id"
                        name="icon_image_blob_id"
                        value=value_icon_image_blob
                        website_id=website.id.clone()
                    />

                    <ImageUploadField
                        label=move || t!(i18n, studio.cover_image)
                        id="cover_image_blob_id"
                        name="cover_image_blob_id"
                        width=288
                        value=value_cover_image_blob
                    />

                    <ThemeSelectorField
                        label=move || t!(i18n, studio.light_theme)
                        name="light_theme"
                        options=LIGHT_THEMES.to_vec()
                        value=value_light_theme
                        error=error_light_theme
                        website=website.clone()
                    />

                    <ThemeSelectorField
                        label=move || t!(i18n, studio.dark_theme)
                        name="dark_theme"
                        options=DARK_THEMES.to_vec()
                        value=value_dark_theme
                        error=error_dark_theme
                        website=website.clone()
                    />

                    <SwitchField
                        label=t!(i18n, studio.publish)
                        name="publish"
                        error=error_publish
                        is_checked=value_publish
                    />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>
            }
        } />
    }
}
