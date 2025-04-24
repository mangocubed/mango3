use mango3_web_utils::components::{AppProvider, FaviconLink};
use mango3_web_utils::prelude::*;

pub fn app() -> Element {
    rsx! {
        document::Meta { name: "robots", content: "noindex, nofollow" }

        AppProvider {
            class: "dark:bg-neutral-950 bg-slate-50",
            FaviconLink {}
        }
    }
}

// pub fn App() -> impl IntoView {
//     view! {
//         <AppProvider>
//             {move || {
//                 let i18n = use_i18n();
//                 let text_suffix = async_t_string!(i18n, accounts.accounts).to_signal();
//                 view! {
//                     <FaviconLink />

//                     <AppTitle suffix=text_suffix />

//                     <Router>
//                         <TopBar
//                             brand=move || view! { <Brand href="/login" suffix=text_suffix /> }
//                             right_items=move |_| view! { <GoToMango3 /> }
//                             show_user_menu=false
//                         />

//                         <main class="grow md:m-6 m-4">
//                             <Routes fallback=NotFoundPage>
//                                 <Route path=StaticSegment("login") view=LoginPage />
//                                 <Route path=StaticSegment("register") view=RegisterPage />
//                                 <Route path=StaticSegment("reset-password") view=ResetPasswordPage />
//                             </Routes>
//                         </main>

//                         <BottomBar />
//                     </Router>

//                     <LoadingOverlay />
//                 }
//             }}
//         </AppProvider>
//     }
// }
