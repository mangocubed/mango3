use leptos::either::Either;
use leptos::prelude::*;

use crate::context::use_current_user_resource;
use crate::models::UserResp;

#[component]
pub fn CurrentUserResource<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(Option<UserResp>) -> IV + Send + Sync + 'static,
{
    let current_user_resource = use_current_user_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Transition>
            {move || Suspend::new(async move {
                match current_user_resource.get() {
                    Some(Ok(user_opt)) => Either::Left(children_store.with_value(|store| store(user_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Transition>
    }
}
