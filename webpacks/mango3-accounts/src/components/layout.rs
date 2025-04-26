use mango3_web_utils::components::{BottomBar, Brand, GoToMango3, TopBar};
use mango3_web_utils::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Layout() -> Element {
    rsx! {
        TopBar {
            brand: rsx! { Brand { href: "/login", { t!("accounts") } } },
            right_items: |_| rsx! { GoToMango3 {} }
        }

        main {
            class: "grow md:m-6 m-4",
            Outlet::<Routes> {}
        }

        BottomBar {}
    }
}
