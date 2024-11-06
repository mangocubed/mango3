use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;

#[cfg(feature = "ssr")]
use mango3_core::locales::I18n;
#[cfg(feature = "ssr")]
use mango3_core::validator::ValidationErrors;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ActionFormResp {
    pub success: Option<bool>,
    pub errors: Option<HashMap<String, String>>,
}

impl ActionFormResp {
    pub fn error(&self, key: &'static str) -> Option<String> {
        self.errors.as_ref().and_then(|e| e.get(key)).cloned()
    }

    #[cfg(feature = "ssr")]
    pub fn new<T>(i18n: &I18n, result: Result<T, ValidationErrors>) -> Result<Self, ServerFnError> {
        Ok(match result {
            Ok(_) => Self {
                success: Some(true),
                errors: None,
            },
            Err(errors) => Self {
                success: Some(false),
                errors: Some(
                    errors
                        .iter()
                        .map(|(input, input_error)| (input.to_string(), input_error.text(&i18n)))
                        .collect(),
                ),
            },
        })
    }

    #[cfg(feature = "ssr")]
    pub fn new_with_error(i18n: &I18n) -> Result<Self, ServerFnError> {
        Self::new::<()>(i18n, Err(ValidationErrors::new()))
    }

    #[cfg(feature = "ssr")]
    pub fn new_with_redirect<T>(
        i18n: &I18n,
        result: Result<T, ValidationErrors>,
        path: &str,
    ) -> Result<Self, ServerFnError> {
        if let Ok(_) = result {
            leptos_axum::redirect(path);
        }

        Self::new(i18n, result)
    }
}

impl From<RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>> for ActionFormResp {
    fn from(action: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>) -> Self {
        action.get().and_then(|v| v.ok()).unwrap_or_default()
    }
}
