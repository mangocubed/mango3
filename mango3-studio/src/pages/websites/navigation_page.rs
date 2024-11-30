use leptos::ev::MouseEvent;
use leptos::prelude::*;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton};
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::icons::{PlusOutlined, TrashOutlined};
use mango3_leptos_utils::models::NavigationItemResp;

use crate::context::use_website_id_param;
use crate::server_functions::{get_all_my_navigation_items, AttemptToSaveNavigation};

#[component]
pub fn NavigationPage() -> impl IntoView {
    let i18n = use_i18n();
    let website_id = use_website_id_param();
    let items_resource = Resource::new_blocking(
        move || website_id.get().unwrap_or_default(),
        get_all_my_navigation_items,
    );
    let server_action = ServerAction::<AttemptToSaveNavigation>::new();
    let action_value = server_action.value();
    let items = RwSignal::new(vec![]);
    let mut temp_id = 1;

    Effect::new(move || {
        if let Some(Ok(nav_items)) = items_resource.get() {
            items.set(nav_items);
        }
    });

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
        <h2 class="h2">{t!(i18n, studio.navigation)}</h2>

        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
            <ActionFormAlert
                action_value=action_value
                error_message=move || { t_string!(i18n, studio.failed_to_save_navigation) }
                redirect_to=format!("/websites/{}", website_id.get().unwrap_or_default())
                success_message=move || { t_string!(i18n, studio.navigation_saved_successfully) }
            />

            <input type="hidden" name="website_id" value=website_id />

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
                                        <div class="form-control w-full">
                                            <input
                                                class="input input-bordered"
                                                name=move || format!("items[{}][title]", index.get())
                                                type="text"
                                                value=item.title
                                            />
                                        </div>
                                    </td>

                                    <td class="p-2">
                                        <div class="form-control w-full">
                                            <input
                                                class="input input-bordered"
                                                name=move || format!("items[{}][url]", index.get())
                                                type="text"
                                                value=item.url
                                            />
                                        </div>
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
    }
}
