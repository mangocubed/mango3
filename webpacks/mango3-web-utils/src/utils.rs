use leptos::prelude::{AsyncDerived, Get, Signal};

pub trait ToSignalTrait<T>
where
    T: Clone + Default + Send + Sync + 'static,
{
    fn to_signal(self) -> Signal<T>;
}

impl<T> ToSignalTrait<T> for AsyncDerived<T>
where
    T: Clone + Default + Send + Sync + 'static,
{
    fn to_signal(self) -> Signal<T> {
        Signal::derive(move || self.get().unwrap_or_default())
    }
}
