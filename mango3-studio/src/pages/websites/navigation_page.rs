use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use mango3_leptos_utils::components::forms::{FormErrorAlert, FormSuccessModal, SubmitButton};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::{PlusOutlined, TrashOutlined};
use mango3_leptos_utils::models::NavigationItemResp;

use crate::components::MyWebsitePageWrapper;
use crate::server_functions::{get_all_my_navigation_items, AttemptToSaveNavigation};

#[component]
pub fn NavigationPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToSaveNavigation>::new();
    let action_value = server_action.value();
    let items = RwSignal::new(vec![]);
    let mut temp_id = 1;

    let add_item = move |event: MouseEvent| {
        event.prevent_default();

        items.update(|items| {
            items.push(NavigationItemResp {
                id: temp_id.to_string(),
                position: 0,
                title: String::new(),
                url: "/".to_owned(),
            });
        });

        temp_id += 1;
    };

    view! {
        <MyWebsitePageWrapper children=move |website| {
            let navigate = use_navigate();
            let website_id = website.id.clone();
            let website_id_clone = website_id.clone();
            let items_resource = LocalResource::new({
                let website_id = website_id.clone();
                move || get_all_my_navigation_items(website_id.clone())
            });
            Effect::new(move || {
                if let Some(Ok(nav_items)) = items_resource.get().map(|resource| resource.take()) {
                    items.set(nav_items);
                }
            });
            view! {
                <h2 class="h2">{t!(i18n, studio.navigation)}</h2>

                <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                    <FormErrorAlert
                        action_value=action_value
                        message=move || t!(i18n, studio.failed_to_save_navigation)
                    />

                    <input type="hidden" name="website_id" value=website_id_clone />

                    <table class="table">
                        <thead>
                            <tr>
                                <th class="p-2">{t!(i18n, studio.title)}</th>
                                <th class="p-2">{t!(i18n, studio.url)}</th>
                                <th class="p-2" />
                            </tr>
                        </thead>

                        <tbody>
                            <ForEnumerate
                                each=move || items.get()
                                key=|item| item.id.clone()
                                children=move |index, item| {
                                    view! {
                                        <tr>
                                            <input
                                                type="hidden"
                                                name=move || format!("items[{}][id]", index.get())
                                                value=item.id
                                            />

                                            <td class="p-2">
                                                <fieldset class="fieldset">
                                                    <input
                                                        class="input w-full"
                                                        name=move || format!("items[{}][title]", index.get())
                                                        type="text"
                                                        value=item.title
                                                    />
                                                </fieldset>
                                            </td>

                                            <td class="p-2">
                                                <fieldset class="fieldset w-full">
                                                    <input
                                                        class="input w-full"
                                                        name=move || format!("items[{}][url]", index.get())
                                                        type="text"
                                                        value=item.url
                                                    />
                                                </fieldset>
                                            </td>

                                            <td class="p-2">
                                                <button
                                                    class="btn btn-outline"
                                                    on:click=move |event| {
                                                        event.prevent_default();
                                                        items
                                                            .update(|items| {
                                                                items.remove(index.get());
                                                            });
                                                    }
                                                >
                                                    <TrashOutlined />
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>
                    </table>

                    <button class="btn btn-ghost btn-block my-4" on:click=add_item>
                        <PlusOutlined />
                        {t!(i18n, studio.add_item)}
                    </button>

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>

                <FormSuccessModal
                    action_value=action_value
                    message=move || t!(i18n, studio.navigation_saved_successfully)
                    on_close=move || {
                        navigate(&format!("/websites/{}", &website_id), Default::default());
                    }
                />
            }
        } />
    }
}
