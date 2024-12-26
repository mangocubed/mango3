use leptos::prelude::*;
use leptos_router::params::ParamsMap;

pub fn param_query(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get("q").unwrap_or_default())
}
