use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::utils::*;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromModel;

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! mut_presenter_error_result {
    () => {
        Ok($crate::presenters::MutPresenter::new_with_error())
    };
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! mut_presenter_result {
    ($core_context:expr, $i18n:expr, $result:expr) => {
        Ok($crate::presenters::MutPresenter::new($core_context, $i18n, $result).await)
    };
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct MutPresenter<T = ()> {
    pub success: Option<bool>,
    pub errors: Option<HashMap<String, String>>,
    pub data: Option<T>,
    pub message: Option<String>,
}

pub type MutPresenterResult<T = ()> = Result<MutPresenter<T>, ServerFnError>;

pub type MutPresenterActionValue<T = ()> = RwSignal<Option<MutPresenterResult<T>>>;

impl<T> MutPresenter<T> {
    pub fn error(&self, key: String) -> Option<String> {
        self.errors.as_ref().and_then(|e| e.get(&key)).cloned()
    }

    pub fn is_invalid(&self) -> bool {
        self.success == Some(false)
    }

    pub fn is_success(&self) -> bool {
        self.success == Some(true)
    }

    #[cfg(feature = "ssr")]
    pub async fn new<M>(core_context: &CoreContext, i18n: &I18n, result: MutResult<M>) -> Self
    where
        T: FromModel<M>,
    {
        match result {
            Ok(success) => Self {
                success: Some(true),
                errors: None,
                data: Some(T::from_model(core_context, &success.data).await),
                message: Some(success.message),
            },
            Err(error) => Self {
                success: Some(false),
                errors: Some(
                    error
                        .errors
                        .iter()
                        .map(|(input, input_error)| (input.to_string(), input_error.text(&i18n)))
                        .collect(),
                ),
                data: None,
                message: Some(error.message),
            },
        }
    }

    #[cfg(feature = "ssr")]
    pub fn new_with_error() -> Self {
        Self {
            success: Some(false),
            errors: None,
            data: None,
            message: None,
        }
    }
}

impl<T> From<MutPresenterActionValue<T>> for MutPresenter<T>
where
    T: Default + Send + Sync + Clone + 'static,
{
    fn from(action: MutPresenterActionValue<T>) -> Self {
        action.with(|resp| resp.clone().and_then(|result| result.ok()).unwrap_or_default())
    }
}
