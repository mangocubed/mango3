use leptos::prelude::*;

use crate::i18n::{t, use_i18n};

#[server]
pub async fn get_country_options() -> Result<Vec<(String, String)>, ServerFnError> {
    Ok(rust_iso3166::ALL
        .iter()
        .map(|country| (country.name.to_owned(), country.alpha2.to_owned()))
        .collect())
}

#[component]
pub fn CountryField(
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: ViewFn,
    name: &'static str,
    #[prop(optional, into)] value: Signal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let options_resource = Resource::new_blocking(|| (), |_| get_country_options());

    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

    let has_error = move || error.get().is_some();

    view! {
        <fieldset class="fieldset">
            <label class="fieldset-label" for=field_id>
                {label.run()}
            </label>

            <select class="select w-full" class:select-error=has_error id=field_id name=name>
                <option value="">{t!(i18n, shared.select)}</option>
                <Suspense>
                    {move || Suspend::new(async move {
                        options_resource
                            .get()
                            .and_then(|result| result.ok())
                            .map(|options| {
                                view! {
                                    <For
                                        each=move || options.clone()
                                        key=|(_, alpha2)| alpha2.clone()
                                        children=move |(name, alpha2)| {
                                            view! {
                                                <option value=alpha2.clone() selected=move || value.get() == alpha2>
                                                    {name}
                                                </option>
                                            }
                                        }
                                    />
                                }
                            })
                    })}
                </Suspense>
            </select>

            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
