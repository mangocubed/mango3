use leptos::prelude::*;

use mango3_leptos_utils::components::{PostCard, WebsiteCard};
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::pages::Page;

use crate::server_functions::{get_posts, get_websites};

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let posts_resource = Resource::new_blocking(move || (), |_| async move { get_posts(5, None).await });
    let websites_resource = Resource::new_blocking(move || (), |_| async move { get_websites(5, None).await });

    view! {
        <Page title=move || t_string!(i18n, shared.home)>
            <div class="flex flex-wrap gap-6 justify-center w-full">
                <section class="flex-1 sm:min-w-[480px] max-w-[640px] w-full">
                    <h2 class="h2">{t!(i18n, home.recent_posts)}</h2>

                    <Suspense>
                        {move || Suspend::new(async move {
                            posts_resource
                                .get()
                                .and_then(|result| result.ok())
                                .map(|cursor_page| {
                                    view! {
                                        <For each=move || cursor_page.nodes.clone() key=|post| post.id.clone() let:post>
                                            <PostCard post=post show_host=true />
                                        </For>
                                    }
                                })
                        })}
                    </Suspense>

                    <a class="btn btn-block ml-auto mr-auto mt-2" href="/posts">
                        {t!(i18n, shared.view_more)}
                    </a>
                </section>

                <section class="flex-1 sm:min-w-[480px] max-w-[640px] w-full">
                    <h2 class="h2">{t!(i18n, home.recent_websites)}</h2>

                    <Suspense>
                        {move || Suspend::new(async move {
                            websites_resource
                                .get()
                                .and_then(|result| result.ok())
                                .map(|cursor_page| {
                                    view! {
                                        <For
                                            each=move || cursor_page.nodes.clone()
                                            key=|website| website.id.clone()
                                            let:website
                                        >
                                            <WebsiteCard website=website />
                                        </For>
                                    }
                                })
                        })}
                    </Suspense>

                    <a class="btn btn-block ml-auto mr-auto mt-2" href="/websites">
                        {t!(i18n, shared.view_more)}
                    </a>
                </section>
            </div>
        </Page>
    }
}
