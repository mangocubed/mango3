use leptos::prelude::*;

use crate::components::forms::FormField;
use crate::i18n::{t, use_i18n};
use crate::presenters::MutPresenterActionValue;

#[server]
pub async fn get_country_options() -> Result<Vec<(String, String)>, ServerFnError> {
    Ok(rust_iso3166::ALL
        .iter()
        .map(|country| (country.name.to_owned(), country.alpha2.to_owned()))
        .collect())
}

#[component]
pub fn CountryField(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let options_resource = Resource::new_blocking(|| (), |_| get_country_options());

    let has_error = move || error.get().is_some();

    view! {
        <FormField action_value=action_value error=error id=id label=label name=name>

            <select class="select w-full" class:select-error=has_error id=id name=name>
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
        </FormField>
    }
}
