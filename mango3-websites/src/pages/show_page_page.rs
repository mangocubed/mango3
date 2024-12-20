use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_meta::Meta;

use mango3_leptos_utils::components::{LoadingSpinner, TimeAgo};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::components::{HighLightCode, MetaDateTime};
use crate::context::use_slug_param;
use crate::server_functions::get_page;

#[component]
pub fn ShowPagePage() -> impl IntoView {
    let i18n = use_i18n();
    let slug = use_slug_param();
    let page_resource = Resource::new_blocking(move || slug.get(), get_page);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match page_resource.get() {
                    Some(Ok(Some(page))) => {
                        EitherOf3::A(
                            view! {
                                <Meta name="description" content=page.title.clone() />

                                <Page class="max-w-[1200px] w-full ml-auto mr-auto" title=page.title.clone()>
                                    {move || {
                                        page.published_at
                                            .map(|published_at| {
                                                view! {
                                                    <MetaDateTime
                                                        property="article:published_time"
                                                        content=published_at
                                                    />
                                                }
                                            })
                                    }}

                                    {move || {
                                        page.updated_at
                                            .map(|updated_at| {
                                                view! {
                                                    <MetaDateTime property="article:modified_time" content=updated_at />
                                                }
                                            })
                                    }}

                                    <div class="card card-compact bg-base-200 shadow-xl mb-4">
                                        {
                                            let page_title = page.title.clone();
                                            move || {
                                                page.cover_image_blob
                                                    .clone()
                                                    .map(|cover_image_blob| {
                                                        let cover_image_url = cover_image_blob
                                                            .variant_url(1200, 200, true);
                                                        view! {
                                                            <Meta property="og:image" content=cover_image_url.clone() />
                                                            <figure>
                                                                <img src=cover_image_url alt=page_title.clone() />
                                                            </figure>
                                                        }
                                                    })
                                            }
                                        } <div class="card-body">
                                            <h1 class="card-title h1 mb-6">{page.title}</h1>

                                            <div class="self-end text-right  my-4">
                                                <TimeAgo value=page.created_at />

                                                {move || {
                                                    page.updated_at
                                                        .map(|update_at| {
                                                            view! {
                                                                " ("
                                                                {t!(i18n, shared.edited)}
                                                                " "
                                                                <TimeAgo value=update_at />
                                                                ")"
                                                            }
                                                        })
                                                }}
                                            </div>

                                            <div
                                                class="prose max-w-none break-words"
                                                inner_html=page.content_html.clone()
                                            />
                                        </div>
                                    </div>

                                    <HighLightCode content=page.content_html />
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(view! { <div /> }),
                }
            })}
        </Suspense>
    }
}
