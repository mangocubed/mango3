use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, WebsiteCard,
};
use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::pages::Page;
use mango3_web_utils::presenters::WebsiteMinPresenter;
use mango3_web_utils::utils::ToSignalTrait;

use crate::server_functions::get_websites;

#[component]
pub fn WebsitesPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollResourceController::new(|after| {
        Resource::new_blocking(
            move || after.get(),
            |after| async move { get_websites(10, after).await },
        )
    });

    let text_title = async_t_string!(i18n, home.websites).to_signal();

    view! {
        <Page title=text_title>
            <h1 class="h1">{move || text_title.get()}</h1>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll controller=controller key=|website: &WebsiteMinPresenter| website.id let:website>
                    <WebsiteCard website=website />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
