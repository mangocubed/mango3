use leptos::prelude::*;

use crate::context::use_basic_config_resource;
use crate::models::BasicConfigResp;

#[component]
pub fn BasicConfigResource<IV, VF>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(BasicConfigResp) -> IV + Send + Sync + 'static,
{
    let basic_config_resource = use_basic_config_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                basic_config_resource
                    .get()
                    .and_then(|result| result.ok())
                    .map(|basic_config| children_store.with_value(|store| store(basic_config)))
            })}
        </Suspense>
    }
}
