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
                current_user_resource
                    .get()
                    .and_then(|result| result.ok())
                    .map(|user| children_store.with_value(|store| store(user)))
            })}
        </Transition>
    }
}
