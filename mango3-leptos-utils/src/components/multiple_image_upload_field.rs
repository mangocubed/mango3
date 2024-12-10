use leptos::prelude::*;
use leptos::text_prop::TextProp;

use crate::models::BlobResp;

use super::ImageUploadField;

#[component]
pub fn MultipleImageUploadField(
    #[prop(into)] id: &'static str,
    #[prop(into, optional)] label: TextProp,
    #[prop(into)] name: &'static str,
    #[prop(into, optional)] value: RwSignal<Vec<BlobResp>>,
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
        <ImageUploadField id=id label=label name=format!("{name}[]") value=uploaded_blob />

        <div class="form-control">
            <ForEnumerate
                each=move || value.get()
                key=|blob| blob.id.clone()
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
                        <ImageUploadField id=format!("{}_{}", id, index.get()) name=format!("{name}[]") value=blob />
                    }
                }
            />
        </div>
    }
}
