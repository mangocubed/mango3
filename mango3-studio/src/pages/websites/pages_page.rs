use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{ConfirmationDialog, InfiniteScroll, PageCard};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::PlusOutlined;
use mango3_leptos_utils::models::PagePreviewResp;

use crate::context::param_website_id;
use crate::server_functions::{get_my_pages, AttemptToDeletePage};

#[component]
pub fn PagesPage() -> impl IntoView {
    let params_map = use_params_map();
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let my_pages_resource = Resource::new_blocking(
        move || (param_website_id(params_map).unwrap_or_default(), after.get()),
        |(website_id, after)| async { get_my_pages(website_id, after).await },
    );
    let pages = RwSignal::new(vec![]);
    let server_action = ServerAction::<AttemptToDeletePage>::new();
    let delete_page = RwSignal::new(None);
    let show_delete_confirmation = RwSignal::new(false);

    view! {
        <ConfirmationDialog
            is_open=show_delete_confirmation
            on_accept=move || {
                let id = delete_page.get().map(|p: PagePreviewResp| p.id).unwrap();
                server_action
                    .dispatch(AttemptToDeletePage {
                        website_id: param_website_id(params_map).unwrap_or_default(),
                        id: id.clone(),
                    });
                pages
                    .update(|p| {
                        p.retain(|p: &PagePreviewResp| p.id != id);
                    });
                delete_page.set(None);
            }
        >
            {t!(i18n, studio.are_you_sure_you_want_to_delete_this_page)}
        </ConfirmationDialog>

        <h2 class="h2">{t!(i18n, studio.pages)}</h2>

        <section class="flex justify-end max-w-[640px] w-full mb-5 mx-auto">
            <a
                class="btn btn-outline"
                href=move || format!("/websites/{}/page/new", param_website_id(params_map).unwrap_or_default())
            >
                <PlusOutlined />
                {t!(i18n, studio.new_page)}
            </a>
        </section>

        <section class="max-w-[640px] w-full mx-auto">
            <InfiniteScroll
                after=after
                key=|page: &PagePreviewResp| page.id.clone()
                resource=my_pages_resource
                nodes=pages
                children=move |page| {
                    view! {
                        <PageCard
                            page=page.clone()
                            actions=move || {
                                let page = page.clone();
                                view! {
                                    <a
                                        class="btn btn-ghost font-bold"
                                        href={
                                            let page_id = page.id.clone();
                                            move || {
                                                format!(
                                                    "/websites/{}/pages/{}/edit",
                                                    param_website_id(params_map).unwrap_or_default(),
                                                    page_id,
                                                )
                                            }
                                        }
                                    >
                                        {t!(i18n, studio.edit)}
                                    </a>

                                    <button
                                        class="btn btn-ghost font-bold"
                                        on:click=move |_| {
                                            delete_page.set(Some(page.clone()));
                                            show_delete_confirmation.set(true);
                                        }
                                    >
                                        {t!(i18n, studio.delete)}
                                    </button>
                                }
                            }
                        />
                    }
                }
            />
        </section>
    }
}
