use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[server]
pub async fn get_country_options() -> Result<Vec<(String, String)>, ServerFnError> {
    let mut countries: Vec<(String, String)> = rust_iso3166::ALL
        .iter()
        .map(|country| (country.name.to_owned(), country.alpha2.to_owned()))
        .collect();

    countries.insert(0, ("Select".to_owned(), "".to_owned()));

    Ok(countries)
}

#[component]
pub fn CountryField(
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: TextProp,
    name: &'static str,
) -> impl IntoView {
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
        <div class="form-control w-full">
            <label class="label" for=field_id>
                <span class="label-text">{move || label.get()}</span>
            </label>
            <select
                class="select select-bordered"
                class:select-error=has_error
                id=field_id
                name=name
            >
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
                                            view! { <option value=alpha2>{name}</option> }
                                        }
                                    />
                                }
                            })
                    })}
                </Suspense>
            </select>
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
