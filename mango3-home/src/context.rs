use leptos::prelude::*;
use leptos_router::params::ParamsMap;

use crate::constants::KEY_PARAM_USERNAME;

pub fn param_username(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get(KEY_PARAM_USERNAME).unwrap_or_default())
}
