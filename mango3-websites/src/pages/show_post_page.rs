use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{LoadingSpinner, PostCard};
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_post;

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let params_map = use_params_map();
    let slug = params_map.with(|params| params.get("slug").unwrap_or_default());
    let post_resource = Resource::new_blocking(move || slug.clone(), get_post);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match post_resource.get() {
                    Some(Ok(Some(post))) => {
                        EitherOf3::A(
                            view! {
                                <Page title=post.title.clone()>
                                    <div class="max-w-[1200px] w-full ml-auto mr-auto">
                                        <PostCard post=post show_content=true />
                                    </div>
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(()),
                }
            })}
        </Suspense>
    }
}
