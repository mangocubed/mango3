use codee::string::FromToStringCodec;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use leptos_use::core::{ElementMaybeSignal, IntoElementMaybeSignal, MaybeRwSignal};
use leptos_use::*;
use std::marker::PhantomData;
use std::sync::Arc;

use super::use_basic_config;

pub fn use_color_mode() -> UseColorModeReturn {
    let UseColorModeOptions {
        target,
        initial_value,
        on_changed,
        _marker,
    } = UseColorModeOptions::default();

    let preferred_dark = use_preferred_dark();

    let system = Signal::derive(move || {
        if preferred_dark.get() {
            ColorMode::Dark
        } else {
            ColorMode::Light
        }
    });

    let (store, set_store) = initial_value.into_signal();

    let (cookie, set_cookie) = get_cookie_signal();

    let _ = sync_signal_with_options(
        (cookie, set_cookie),
        (store, set_store),
        SyncSignalOptions::with_assigns(
            move |store: &mut ColorMode, cookie: &Option<ColorMode>| {
                if let Some(cookie) = cookie {
                    *store = cookie.clone();
                }
            },
            move |cookie: &mut Option<ColorMode>, store: &ColorMode| *cookie = Some(store.clone()),
        ),
    );

    let state = Signal::derive(move || {
        let value = store.get();
        if value == ColorMode::Auto {
            system.get()
        } else {
            value
        }
    });

    let target = target.into_element_maybe_signal();

    let update_html_attrs = {
        move |target: ElementMaybeSignal<web_sys::Element>, value: ColorMode| {
            let el = target.get_untracked();

            if let Some(el) = el {
                let _ = el.set_attribute("data-theme", &value.to_string());
            }
        }
    };

    let default_on_changed = move |mode: ColorMode| {
        update_html_attrs(target.clone(), mode);
    };

    let on_changed = move |mode: ColorMode| {
        on_changed(mode, Arc::new(default_on_changed.clone()));
    };

    Effect::new({
        let on_changed = on_changed.clone();

        move |_| {
            on_changed.clone()(state.get());
        }
    });

    on_cleanup(move || {
        on_changed(state.get());
    });

    let mode = Signal::derive(move || store.get());

    UseColorModeReturn {
        mode,
        set_mode: set_store,
        store,
        set_store,
        system,
        state,
    }
}

fn get_cookie_signal() -> (Signal<Option<ColorMode>>, WriteSignal<Option<ColorMode>>) {
    let basic_config = use_basic_config();
    use_cookie_with_options::<ColorMode, FromToStringCodec>(
        "_mango3_color_mode",
        UseCookieOptions::default()
            .domain(basic_config.domain.clone())
            .same_site(SameSite::Strict),
    )
}

#[derive(DefaultBuilder)]
pub struct UseColorModeOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
    M: ?Sized,
{
    /// Element that the color mode will be applied to. Defaults to `"html"`.
    target: El,

    /// Initial value of the color mode. Defaults to `"Auto"`.
    #[builder(into)]
    initial_value: MaybeRwSignal<ColorMode>,

    /// Custom handler that is called on updates.
    /// If specified this will override the default behavior.
    /// To get the default behaviour back you can call the provided `default_handler` function.
    /// It takes two parameters:
    ///     - `mode: ColorMode`: The color mode to change to.
    ///     -`default_handler: Arc<dyn Fn(ColorMode)>`: The default handler that would have been called if the `on_changed` handler had not been specified.
    on_changed: OnChangedFn,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

type OnChangedFn = Arc<dyn Fn(ColorMode, Arc<dyn Fn(ColorMode) + Send + Sync>) + Send + Sync>;

impl Default for UseColorModeOptions<&'static str, str> {
    fn default() -> Self {
        Self {
            target: "html",
            initial_value: ColorMode::Auto.into(),
            on_changed: Arc::new(move |mode, default_handler| (default_handler)(mode)),
            _marker: PhantomData,
        }
    }
}
