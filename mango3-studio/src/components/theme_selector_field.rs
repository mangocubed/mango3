use leptos::prelude::*;

use mango3_leptos_utils::components::TopBar;
use mango3_leptos_utils::models::WebsiteResp;

#[component]
pub fn ThemeSelectorField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into)] label: String,
    name: &'static str,
    options: Vec<&'static str>,
    value: RwSignal<String>,
    website: WebsiteResp,
) -> impl IntoView {
    let website_name = website.name.clone();
    let website_image_blob_url = website
        .icon_image_blob
        .as_ref()
        .map(|blob| blob.variant_url(42, 42, true));
    let options_store = StoredValue::new(options);

    view! {
        <div class="form-control">
            <input type="hidden" name=name value=value />

            <label class="label">
                <span class="label-text">{label.clone()}</span>
            </label>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <For
                    each=move || options_store.read_value().clone()
                    key=|key| key.to_owned()
                    children=move |key| {
                        let website_name = website_name.clone();
                        let website_icon_image_blob_url = website_image_blob_url.clone();
                        let is_selected = move || value.get() == key;
                        view! {
                            <div
                                class="card card-compact card-bordered cursor-pointer hover:bg-base-100"
                                class:bg-base-100=is_selected
                                on:click=move |_| value.set(key.to_owned())
                            >
                                <div class="card-body">
                                    <div class="card-title font-normal text-base">
                                        <input type="radio" class="radio" checked=is_selected />
                                        {key}
                                    </div>

                                    <div class="relative zoom-75" data-theme=key>
                                        <TopBar
                                            brand=move || {
                                                view! {
                                                    <a class="btn btn-ghost text-xl pl-1 pr-2">
                                                        <img
                                                            alt=website_name.clone()
                                                            class="rounded w-[42px] h-[42px]"
                                                            src=website_icon_image_blob_url
                                                        />
                                                        {website_name.clone()}
                                                    </a>
                                                }
                                            }
                                            class="bg-base-100"
                                            show_user_menu=false
                                        />

                                        <div class="m-6 max-w-[640px]">
                                            <div class="card card-compact bg-base-100 shadow-xl">
                                                <div class="card-body">
                                                    <div class="card-title">{"Lorem ipsum."}</div>

                                                    <div>{"Lorem ipsum odor amet, consectetuer adipiscing elit."}</div>
                                                </div>
                                            </div>

                                        </div>

                                        <div class="absolute inset-0" />
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
