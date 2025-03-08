use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_leptos_utils::components::forms::{
    FormErrorAlert, FormSuccessModal, ImageUploadField, MarkdownEditorField, SubmitButton, SwitchField, TextField,
};
use mango3_leptos_utils::i18n::{t, use_i18n};

use crate::components::{MyWebsite, ThemeSelectorField};
use crate::server_functions::AttemptToUpdateWebsite;

const DARK_THEMES: [&str; 14] = [
    "dark",
    "abyss",
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
const LIGHT_THEMES: [&str; 21] = [
    "light",
    "acid",
    "autumn",
    "bumblebee",
    "caramellatte",
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
    "silk",
    "valentine",
    "winter",
    "wireframe",
];

#[component]
pub fn EditPage() -> impl IntoView {
    let server_action = ServerAction::<AttemptToUpdateWebsite>::new();
    let action_value = server_action.value();

    view! {
        <MyWebsite children=move |website| {
            let navigate = use_navigate();
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
                    <FormErrorAlert
                        action_value=action_value
                        message=move || t!(i18n, studio.failed_to_update_website)
                    />

                    <input type="hidden" name="id" value=website.id.clone() />

                    <TextField
                        action_value=action_value
                        id="name"
                        label=move || t!(i18n, studio.name)
                        name="name"
                        value=value_name
                    />

                    <MarkdownEditorField
                        action_value=action_value
                        id="description"
                        label=move || t!(i18n, studio.description)
                        name="description"
                        value=value_description
                    />

                    <ImageUploadField
                        action_value=action_value
                        id="icon_image_blob_id"
                        label=move || t!(i18n, studio.icon_image)
                        name="icon_image_blob_id"
                        value=value_icon_image_blob
                        website_id=website.id.clone()
                    />

                    <ImageUploadField
                        action_value=action_value
                        id="cover_image_blob_id"
                        label=move || t!(i18n, studio.cover_image)
                        name="cover_image_blob_id"
                        width=288
                        value=value_cover_image_blob
                    />

                    <ThemeSelectorField
                        action_value=action_value
                        id="light_theme"
                        label=move || t!(i18n, studio.light_theme)
                        name="light_theme"
                        options=LIGHT_THEMES.to_vec()
                        value=value_light_theme
                        website=website.clone()
                    />

                    <ThemeSelectorField
                        action_value=action_value
                        id="dark_theme"
                        label=move || t!(i18n, studio.dark_theme)
                        name="dark_theme"
                        options=DARK_THEMES.to_vec()
                        value=value_dark_theme
                        website=website.clone()
                    />

                    <SwitchField
                        action_value=action_value
                        id="publish"
                        label=move || t!(i18n, studio.publish)
                        name="publish"
                        is_checked=value_publish
                    />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>

                <FormSuccessModal
                    action_value=action_value
                    message=move || t!(i18n, studio.website_updated_successfully)
                    on_close=move || {
                        navigate("/", Default::default());
                    }
                />
            }
        } />
    }
}
