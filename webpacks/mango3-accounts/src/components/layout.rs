use mango3_web_utils::components::TopBar;
use mango3_web_utils::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Layout() -> Element {
    rsx! {
        TopBar {}

        Outlet::<Routes> {}
    }
}
