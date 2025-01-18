// Based on: https://github.com/Synphonyte/leptos-use/blob/main/src/use_color_mode.rs

use codee::string::FromToStringCodec;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use leptos_use::core::MaybeRwSignal;
use leptos_use::{
    sync_signal_with_options, use_cookie_with_options, use_document, ColorMode, SameSite, SyncSignalOptions,
    UseColorModeReturn, UseCookieOptions,
};
use std::marker::PhantomData;
use std::sync::Arc;

use super::use_basic_config;

fn use_preferred_dark() -> Signal<bool> {
    #[cfg(not(feature = "ssr"))]
    {
        leptos_use::use_media_query("(prefers-color-scheme: dark)")
    }

    #[cfg(feature = "ssr")]
    {
        Signal::derive(move || {
            leptos_use::utils::header(http::HeaderName::from_static("sec-ch-prefers-color-scheme"))
                == Some("dark".to_string())
        })
    }
}

pub fn use_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(UseColorModeOptions::default())
}

pub fn use_color_mode_with_options<M>(options: UseColorModeOptions<M>) -> UseColorModeReturn
where
    M: ?Sized,
{
    let UseColorModeOptions {
        initial_value,
        on_changed,
        light_theme,
        dark_theme,

        _marker,
    } = options;

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

    let default_on_changed = move |mode: ColorMode| {
        let el = use_document().query_selector("html");

        if let Ok(Some(el)) = el {
            let data_theme = match mode {
                ColorMode::Dark => dark_theme.as_str(),
                _ => light_theme.as_str(),
            };

            let _ = el.set_attribute("data-theme", data_theme);
        }
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

    #[cfg(not(feature = "ssr"))]
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
            .path("/")
            .same_site(SameSite::Strict),
    )
}

#[derive(DefaultBuilder)]
pub struct UseColorModeOptions<M>
where
    M: ?Sized,
{
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

    light_theme: String,
    dark_theme: String,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

type OnChangedFn = Arc<dyn Fn(ColorMode, Arc<dyn Fn(ColorMode) + Send + Sync>) + Send + Sync>;

impl Default for UseColorModeOptions<str> {
    fn default() -> Self {
        Self {
            initial_value: ColorMode::Auto.into(),
            on_changed: Arc::new(move |mode, default_handler| (default_handler)(mode)),
            light_theme: "light".to_owned(),
            dark_theme: "dark".to_owned(),
            _marker: PhantomData,
        }
    }
}
