use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::*;
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::pages::{LoginPage, RegisterPage, ResetPasswordPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            {move || {
                let i18n = use_i18n();
                let text_suffix = async_t_string!(i18n, accounts.accounts).to_signal();
                view! {
                    <FaviconLink />

                    <AppTitle suffix=text_suffix />

                    <Router>
                        <TopBar
                            brand=move || view! { <Brand href="/login" suffix=text_suffix /> }
                            right_items=move || view! { <GoToMango3 /> }
                            show_user_menu=false
                        />

                        <main class="grow md:m-6 m-4">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("login") view=LoginPage />
                                <Route path=StaticSegment("register") view=RegisterPage />
                                <Route path=StaticSegment("reset-password") view=ResetPasswordPage />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>

                    <LoadingOverlay />
                }
            }}
        </AppProvider>
    }
}
