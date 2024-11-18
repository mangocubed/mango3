use leptos::ev::Event;
use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::ActionFormResp;

use crate::components::MyWebsiteResource;
use crate::server_functions::AttemptToCreatePost;

#[component]
pub fn NewPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToCreatePost>::new();
    let action_value = server_action.value();
    let error_title = RwSignal::new(None);
    let error_slug = RwSignal::new(None);
    let error_content = RwSignal::new(None);
    let error_publish = RwSignal::new(None);
    let value_slug = RwSignal::new("".to_owned());

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_title.set(response.error("title"));
        error_slug.set(response.error("slug"));
        error_content.set(response.error("content"));
    });

    let title_on_input = move |event: Event| {
        value_slug.set(slug::slugify(event_target_value(&event)));
    };

    view! {
        <MyWebsiteResource children=move |website| {
            website
                .map(|website| {
                    view! {
                        <ActionForm
                            action=server_action
                            attr:autocomplete="off"
                            attr:novalidate="true"
                            attr:class="form"
                        >
                            <ActionFormAlert
                                action_value=action_value
                                error_message=move || {
                                    t_string!(i18n, studio.failed_to_create_post)
                                }
                                redirect_to=format!("/websites/{}", website.id)
                                success_message=move || {
                                    t_string!(i18n, studio.post_created_successfully)
                                }
                            />

                            <input type="hidden" name="website_id" value=website.id />

                            <TextField
                                label=move || t_string!(i18n, studio.title)
                                name="title"
                                error=error_title
                                on_input=title_on_input
                            />

                            <TextField
                                label=move || t_string!(i18n, studio.slug)
                                name="slug"
                                value=value_slug
                                error=error_slug
                            />

                            <TextareaField
                                label=move || t_string!(i18n, studio.content)
                                name="content"
                                rows=8
                                error=error_content
                            />

                            <ImageUploadField
                                label=move || t_string!(i18n, studio.cover_image)
                                name="cover_image_blob_id"
                                width=288
                            />

                            <SwitchField
                                label=move || t_string!(i18n, studio.publish)
                                name="publish"
                                error=error_publish
                            />

                            <SubmitButton is_loading=server_action.pending() />
                        </ActionForm>
                    }
                })
        } />
    }
}
