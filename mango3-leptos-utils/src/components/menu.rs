use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Menu(children: Children) -> impl IntoView {
    view! { <ul class="menu bg-base-200 rounded-box md:w-56">{children()}</ul> }
}

#[component]
pub fn MenuItem(
    #[prop(into)] href: &'static str,
    #[prop(into)] icon: ViewFnOnce,
    #[prop(into)] label: AsyncDerived<&'static str>,
) -> impl IntoView {
    let location = use_location();
    let is_active = move || location.pathname.get() == href;
    let label_text = move || label.get();

    view! {
        <li>
            <a class:menu-active=is_active href=href title=label_text>
                {icon.run()}
                <span class="md:inline hidden">{label_text}</span>
            </a>
        </li>
    }
}
