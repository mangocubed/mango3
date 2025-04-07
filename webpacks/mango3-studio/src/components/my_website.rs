use leptos::either::Either;
use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::LoadingSpinner;
use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::pages::Page;
use mango3_web_utils::presenters::WebsitePresenter;

use crate::context::use_my_website_resource;

#[component]
pub fn MyWebsiteOpt<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(Option<WebsitePresenter>) -> IV + Send + Sync + 'static,
{
    let my_website_resource = use_my_website_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Transition fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match my_website_resource.get().map(|resource| resource.take()) {
                    Some(Ok(website_opt)) => Either::Left(children_store.with_value(|store| store(website_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Transition>
    }
}

#[component]
pub fn MyWebsite<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsitePresenter) -> IV + Send + Sync + 'static,
{
    view! { <MyWebsiteOpt children=move |website_opt| { website_opt.map(&children) } /> }
}

#[component]
pub fn MyWebsitePageWrapper<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsitePresenter) -> IV + Copy + Send + Sync + 'static,
{
    let i18n = use_i18n();
    let text_my_websites = async_t_string!(i18n, studio.my_websites);

    view! {
        <MyWebsiteOpt children=move |website_opt| {
            website_opt
                .map(|website| {
                    let website_name = website.name.clone();
                    let text_title = Signal::derive(move || {
                        format!(
                            "{} > {}",
                            text_my_websites.with(|value| value.unwrap_or("My websites")),
                            website_name.clone(),
                        )
                    });
                    view! { <Page title=text_title>{children(website)}</Page> }
                })
        } />
    }
}
