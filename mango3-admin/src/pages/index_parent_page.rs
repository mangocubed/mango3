use leptos::prelude::*;
use leptos_router::components::Outlet;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{Menu, MenuItem};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::icons::{HomeOutlined, UsersOutlined};

#[component]
pub fn IndexParentPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="flex grow gap-4">
            <Menu>
                <MenuItem href="/" icon=HomeOutlined label=async_t_string!(i18n, shared.home) />
                <MenuItem href="/users" icon=UsersOutlined label=async_t_string!(i18n, admin.users) />
            </Menu>

            <div class="grow ml-4">
                <Outlet />
            </div>
        </div>
    }
}
