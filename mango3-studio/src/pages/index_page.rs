use leptos::either::Either;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_use::use_element_visibility;

use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::WebsiteResp;
use mango3_leptos_utils::pages::AuthenticatedPage;

use crate::server_functions::get_my_websites;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let my_websites_resource = Resource::new_blocking(move || after.get(), get_my_websites);
    let websites: RwSignal<Vec<WebsiteResp>> = RwSignal::new(vec![]);
    let has_more = RwSignal::new(true);
    let last_id = Memo::new(move |_| websites.get().last().map(|w| w.id.clone()));
    let node_ref = NodeRef::<Div>::new();
    let bottom_is_visible = use_element_visibility(node_ref);
    let is_loading_more = Memo::new(move |_| bottom_is_visible.get() && has_more.get());

    Effect::new(move || {
        if is_loading_more.get() {
            after.set(last_id.get());

            my_websites_resource.refetch();
        }
    });

    Effect::new(move || {
        if let Some(Ok(more_websites)) = my_websites_resource.get() {
            websites.update(|w| {
                let ids: Vec<String> = w.iter().map(|w| w.id.clone()).collect();
                let mut filtered_websites: Vec<WebsiteResp> = more_websites
                    .clone()
                    .iter()
                    .filter(|mw| !ids.contains(&mw.id))
                    .cloned()
                    .collect();

                has_more.set(!filtered_websites.is_empty());

                w.append(&mut filtered_websites);
            });
        }
    });

    view! {
        <AuthenticatedPage title=move || t_string!(i18n, shared.home)>
            <section class="max-w-[640px] m-auto">
                <h2 class="text-xl font-bold mb-4">{t!(i18n, studio.my_websites)}</h2>
                <For each=move || websites.get() key=|website| website.id.clone() let:website>
                    <div class="card card-compact bg-base-100 shadow-xl">
                        <div class="card-body">
                            <div class="flex gap-3">
                                {
                                    let website_name = website.name.clone();
                                    move || {
                                        if let Some(icon_image_blob) = website
                                            .icon_image_blob
                                            .as_ref()
                                        {
                                            Either::Left(
                                                view! {
                                                    <div class="avatar">
                                                        <div class="w-[32px] rounded">
                                                            <img
                                                                alt=website_name.clone()
                                                                class="rounded"
                                                                width=32
                                                                height=32
                                                                src=icon_image_blob.variant_url(32, 32, true)
                                                            />
                                                        </div>
                                                    </div>
                                                },
                                            )
                                        } else {
                                            Either::Right(
                                                view! {
                                                    <div class="avatar placeholder">
                                                        <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                                            <span class="text-xs">{website.initials.clone()}</span>
                                                        </div>
                                                    </div>
                                                },
                                            )
                                        }
                                    }
                                } <h3 class="card-title">{website.name.clone()}</h3>
                            </div>

                            <p>{website.description}</p>

                            <div class="card-actions justify-end">
                                <Show when=move || website.is_published>
                                    <a class="btn btn-ghost font-bold" href=website.url.clone()>
                                        {t!(i18n, studio.go_to_website)}
                                    </a>
                                </Show>

                                <a
                                    class="btn btn-ghost font-bold"
                                    href=format!("/websites/{}", website.id)
                                >
                                    {t!(i18n, studio.view_more)}
                                </a>
                            </div>
                        </div>
                    </div>
                </For>

                <div node_ref=node_ref class="flex mt-4">
                    <span
                        class="loading loading-spinner loading-lg m-auto"
                        class:hidden=move || !is_loading_more.get()
                    ></span>
                </div>
            </section>
        </AuthenticatedPage>
    }
}
