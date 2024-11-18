use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;

use leptos_meta::{Link, Title};
use mango3_leptos_utils::components::{AppTitle, Brand, GoToMango3, LoadingSpinner, TopBar};
use mango3_leptos_utils::context::{use_basic_config, use_page_title};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_current_website_resource;

#[component]
pub fn CurrentWebsiteResource<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(Option<WebsiteResp>) -> IV + Send + Sync + 'static,
{
    let current_website_resource = use_current_website_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match current_website_resource.get() {
                    Some(Ok(website_opt)) => Either::Left(children_store.with_value(|store| store(website_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn WebsiteTopBar() -> impl IntoView {
    let var_name = view! {
        <TopBar right_items=move || view! { <GoToMango3 /> }>
            <CurrentWebsiteResource children=move |website| {
                let i18n = use_i18n();
                let basic_config = use_basic_config();
                let page_title = use_page_title();
                match website {
                    Some(website) => {
                        let website_name = website.name.clone();
                        let title_text = move || {
                            let mut text = "".to_owned();
                            if let Some(page_title) = page_title.value.get() {
                                text += &format!("{page_title} | ");
                            }
                            text
                                + &format!(
                                    "{} ({})",
                                    website_name.clone(),
                                    t_string!(i18n, websites.powered_by_title, title = basic_config.title.clone()),
                                )
                        };
                        Either::Left(
                            view! {
                                <Title text=title_text />

                                {
                                    let website_icon_image_blob = website.icon_image_blob.clone();
                                    move || {
                                        website_icon_image_blob
                                            .clone()
                                            .map(|blob| {
                                                view! { <Link rel="icon" href=blob.variant_url(32, 32, true) /> }
                                            })
                                    }
                                }

                                <a class="btn btn-ghost text-xl" href="/">
                                    <img
                                        alt=website.name.clone()
                                        class="rounded"
                                        src=website.icon_image_blob.map(|blob| blob.variant_url(42, 42, true))
                                    />
                                    {website.name}
                                </a>
                            },
                        )
                    }
                    None => {
                        Either::Right(move || {
                            let home_url = basic_config.home_url.clone();
                            view! {
                                <AppTitle />

                                <Brand href=home_url.clone() />
                            }
                        })
                    }
                }
            } />
        </TopBar>
    };
    var_name
}
