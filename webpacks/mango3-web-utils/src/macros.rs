#[macro_export]
macro_rules! async_t_string {
    ($($tt:tt)*) => {
        leptos::prelude::AsyncDerived::new(move || leptos_i18n::t_string!($($tt)*))
    }
}
