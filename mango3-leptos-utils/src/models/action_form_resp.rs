use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;

#[cfg(feature = "ssr")]
use mango3_core::locales::I18n;
#[cfg(feature = "ssr")]
use mango3_core::validator::ValidationErrors;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ActionFormResp<D = ()> {
    pub success: Option<bool>,
    pub errors: Option<HashMap<String, String>>,
    pub data: Option<D>,
}

impl<D> ActionFormResp<D> {
    pub fn error(&self, key: &'static str) -> Option<String> {
        self.errors.as_ref().and_then(|e| e.get(key)).cloned()
    }

    pub fn is_invalid(&self) -> bool {
        self.success == Some(false)
    }

    pub fn is_success(&self) -> bool {
        self.success == Some(true)
    }

    #[cfg(feature = "ssr")]
    pub fn new<T>(i18n: &I18n, result: Result<T, ValidationErrors>) -> Result<Self, ServerFnError> {
        Ok(match result {
            Ok(_) => Self {
                success: Some(true),
                errors: None,
                data: None,
            },
            Err(errors) => Self {
                success: Some(false),
                errors: Some(
                    errors
                        .iter()
                        .map(|(input, input_error)| (input.to_string(), input_error.text(&i18n)))
                        .collect(),
                ),
                data: None,
            },
        })
    }

    #[cfg(feature = "ssr")]
    pub fn new_with_data<T>(i18n: &I18n, result: Result<T, ValidationErrors>, data: D) -> Result<Self, ServerFnError> {
        Ok(match result {
            Ok(_) => Self {
                success: Some(true),
                errors: None,
                data: Some(data),
            },
            Err(errors) => Self {
                success: Some(false),
                errors: Some(
                    errors
                        .iter()
                        .map(|(input, input_error)| (input.to_string(), input_error.text(&i18n)))
                        .collect(),
                ),
                data: Some(data),
            },
        })
    }

    #[cfg(feature = "ssr")]
    pub fn new_with_error(i18n: &I18n) -> Result<Self, ServerFnError> {
        Self::new::<()>(i18n, Err(ValidationErrors::new()))
    }
}

impl<D> From<RwSignal<Option<Result<ActionFormResp<D>, ServerFnError<NoCustomError>>>>> for ActionFormResp<D>
where
    D: Default + Send + Sync + Clone + 'static,
{
    fn from(action: RwSignal<Option<Result<ActionFormResp<D>, ServerFnError<NoCustomError>>>>) -> Self {
        action.with(|resp| resp.clone().and_then(|result| result.ok()).unwrap_or_default())
    }
}
