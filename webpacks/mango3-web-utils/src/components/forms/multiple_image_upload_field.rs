use leptos::prelude::*;
use leptos::text_prop::TextProp;

use crate::presenters::BlobPresenter;

use super::ImageUploadField;

#[component]
pub fn MultipleImageUploadField(
    #[prop(into, optional)] id: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
    #[prop(into, optional)] value: RwSignal<Vec<BlobPresenter>>,
    #[prop(into, optional)] website_id: TextProp,
) -> impl IntoView {
    let uploaded_blob = RwSignal::new(None);

    Effect::new(move || {
        if let Some(blob) = uploaded_blob.get() {
            value.update(|blobs| {
                blobs.insert(0, blob);
            });
            uploaded_blob.set(None);
        }
    });

    view! {
        <ImageUploadField id=id label=label name=&format!("{name}[]") website_id=website_id value=uploaded_blob />

        <fieldset class="fieldset">
            <ForEnumerate
                each=move || value.get()
                key=|blob| blob.id
                children=move |index, blob| {
                    let blob = RwSignal::new(Some(blob));
                    Effect::new(move || {
                        if blob.get().is_none() {
                            value
                                .update(|blobs| {
                                    blobs.remove(index.get());
                                });
                        }
                    });

                    view! {
                        <ImageUploadField id=&format!("{}_{}", id, index.get()) name=&format!("{name}[]") value=blob />
                    }
                }
            />
        </fieldset>
    }
}
