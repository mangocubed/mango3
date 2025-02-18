use leptos::prelude::*;

use mango3_leptos_utils::components::{TopBar, WebsiteIcon};
use mango3_leptos_utils::models::WebsitePreviewResp;

#[component]
pub fn ThemeSelectorField<L>(
    #[prop(into, optional)] error: MaybeProp<String>,
    label: L,
    name: &'static str,
    options: Vec<&'static str>,
    value: RwSignal<String>,
    #[prop(into)] website: WebsitePreviewResp,
) -> impl IntoView
where
    L: IntoView,
{
    let website_name = website.name.clone();
    let options_store = StoredValue::new(options);

    view! {
        <fieldset class="fieldset">
            <input type="hidden" name=name value=value />

            <label class="fieldset-label">{label}</label>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <For
                    each=move || options_store.read_value().clone()
                    key=|key| key.to_owned()
                    children=move |key| {
                        let website_name = website_name.clone();
                        let is_selected = move || value.get() == key;
                        view! {
                            <div
                                class="card card-sm card-border cursor-pointer hover:bg-base-100"
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
                                            brand={
                                                let website = website.clone();
                                                move || {
                                                    view! {
                                                        <a class="btn btn-ghost text-xl pl-1 pr-2">
                                                            <WebsiteIcon website=website size=42 />

                                                            {website_name.clone()}
                                                        </a>
                                                    }
                                                }
                                            }
                                            class="bg-base-200"
                                            show_user_menu=false
                                        />

                                        <div class="m-6 max-w-[640px]">
                                            <div class="card card-sm bg-base-200 shadow-xl">
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

            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
