use leptos::prelude::*;

use mango3_web_utils::components::forms::FormField;
use mango3_web_utils::components::{TopBar, WebsiteIcon};
use mango3_web_utils::presenters::{MutPresenterActionValue, WebsiteMinPresenter};

#[component]
pub fn ThemeSelectorField(
    action_value: MutPresenterActionValue,
    id: &'static str,
    #[prop(into)] label: ViewFn,
    name: &'static str,
    options: Vec<&'static str>,
    value: RwSignal<String>,
    #[prop(into)] website: WebsiteMinPresenter,
) -> impl IntoView {
    let options_store = StoredValue::new(options);

    view! {
        <FormField action_value=action_value id=id label=label name=name>
            <input type="hidden" name=name value=value />

            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <For
                    each=move || options_store.read_value().clone()
                    key=|key| key.to_owned()
                    children=move |key| {
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
                                                let website_name = website.name.clone();
                                                move || {
                                                    view! {
                                                        <a class="btn btn-ghost text-xl pl-1 pr-2">
                                                            <WebsiteIcon website=website size=42 />

                                                            {website_name}
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
        </FormField>
    }
}
