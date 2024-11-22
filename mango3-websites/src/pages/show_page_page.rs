use leptos::either::EitherOf3;
use leptos::prelude::*;

use mango3_leptos_utils::components::{LoadingSpinner, PageCard};
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::context::use_slug_param;
use crate::server_functions::get_page;

#[component]
pub fn ShowPagePage() -> impl IntoView {
    let slug = use_slug_param();
    let page_resource = Resource::new_blocking(move || slug.clone(), get_page);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match page_resource.get() {
                    Some(Ok(Some(page))) => {
                        EitherOf3::A(
                            view! {
                                <Page title=page.title.clone()>
                                    <div class="max-w-[1200px] w-full ml-auto mr-auto">
                                        <PageCard page=page show_content=true />
                                    </div>
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(()),
                }
            })}
        </Suspense>
    }
}
