use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::Page;

use crate::components::CurrentWebsiteResource;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <Page title=move || t_string!(i18n, shared.home)>
            <div class="max-w-[1200px] w-full ml-auto mr-auto">
                <CurrentWebsiteResource children=move |website| {
                    website
                        .map(|website| {
                            view! {
                                {website
                                    .cover_image_blob
                                    .map(|blob| {
                                        view! {
                                            <img
                                                class="rounded mb-4"
                                                src=blob.variant_url(1200, 200, true)
                                            />
                                        }
                                    })}
                                <h3 class="text-lg font-bold">{website.description}</h3>
                            }
                        })
                } />
            </div>
        </Page>
    }
}
