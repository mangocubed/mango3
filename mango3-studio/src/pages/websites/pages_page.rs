use leptos::prelude::*;

use mango3_leptos_utils::components::{ConfirmationDialog, InfiniteScroll, PageCard};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::PageResp;

use crate::context::use_website_id_param;
use crate::server_functions::{get_my_pages, AttemptToDeletePage};

#[component]
pub fn PagesPage() -> impl IntoView {
    let website_id = use_website_id_param();
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let my_pages_resource = Resource::new_blocking(
        move || (website_id.get().unwrap_or_default(), after.get()),
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
                let id = delete_page.get().map(|p: PageResp| p.id).unwrap();
                server_action
                    .dispatch(AttemptToDeletePage {
                        website_id: website_id.get().unwrap_or_default(),
                        id: id.clone(),
                    });
                pages
                    .update(|p| {
                        p.retain(|p: &PageResp| p.id != id);
                    });
                delete_page.set(None);
            }
        >
            {t!(i18n, studio.are_you_sure_you_want_to_delete_this_page)}
        </ConfirmationDialog>

        <h2 class="h2">{t!(i18n, studio.pages)}</h2>

        <section class="max-w-[640px] w-full ml-auto mr-auto">
            <InfiniteScroll
                after=after
                key=|page: &PageResp| page.id.clone()
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
                                        href=format!(
                                            "/websites/{}/pages/{}/edit",
                                            website_id.get().unwrap_or_default(),
                                            &page.id,
                                        )
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
