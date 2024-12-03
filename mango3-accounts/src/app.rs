use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, FaviconLink, GoToMango3, TopBar};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;

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
                let suffix = move || t_string!(i18n, accounts.accounts);
                view! {
                    <FaviconLink />

                    <AppTitle suffix=suffix />

                    <Router>
                        <TopBar right_items=move || view! { <GoToMango3 /> } show_user_menu=false>
                            <Brand href="/login" suffix=suffix />
                        </TopBar>

                        <main class="grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("login") view=LoginPage />
                                <Route path=StaticSegment("register") view=RegisterPage />
                                <Route path=StaticSegment("reset-password") view=ResetPasswordPage />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
