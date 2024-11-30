use std::collections::HashMap;

use regex::Regex;

use crate::enums::{Input, InputError};
use crate::locales::I18n;

pub type ValidationErrors = HashMap<Input, InputError>;

impl InputError {
    pub fn text(&self, i18n: &I18n) -> String {
        match self {
            InputError::InvalidLength(min, max) => {
                let mut text_args = HashMap::new();

                text_args.insert("min".to_owned(), min.unwrap_or_default().into());
                text_args.insert("max".to_owned(), max.unwrap_or_default().into());

                i18n.text_with_args(&self.to_string(), &text_args)
            }
            _ => i18n.text(&self.to_string()),
        }
    }
}

pub struct Validator {
    pub errors: ValidationErrors,
    pub is_valid: bool,
}

impl Default for Validator {
    fn default() -> Self {
        Self {
            is_valid: true,
            errors: HashMap::default(),
        }
    }
}

impl Validator {
    fn add_error(&mut self, input: Input, error: InputError) {
        self.is_valid = false;
        self.errors.insert(input, error);
    }

    pub fn custom_validation(&mut self, input: Input, error: InputError, validation: &dyn Fn() -> bool) -> bool {
        if !validation() {
            self.add_error(input, error);
            return false;
        }

        true
    }
}

pub trait ValidatorTrait<T> {
    fn validate_absence(&mut self, input: Input, value: T) -> bool;

    fn validate_format(&mut self, input: Input, value: T, with: &Regex) -> bool;

    fn validate_length(&mut self, input: Input, value: T, min: Option<u32>, max: Option<u32>) -> bool;

    fn validate_numericality(&mut self, input: Input, value: T, min: Option<T>, max: Option<T>) -> bool;

    fn validate_presence(&mut self, input: Input, value: T) -> bool;
}

impl ValidatorTrait<&str> for Validator {
    fn validate_absence(&mut self, input: Input, value: &str) -> bool {
        if !value.is_empty() {
            self.add_error(input, InputError::CantBePresent);
            return false;
        }

        true
    }

    fn validate_format(&mut self, input: Input, value: &str, with: &Regex) -> bool {
        if !with.is_match(value) {
            self.add_error(input, InputError::InvalidFormat);
            return false;
        }

        true
    }

    fn validate_length(&mut self, input: Input, value: &str, min: Option<u32>, max: Option<u32>) -> bool {
        let Ok(value_length) = TryInto::<u32>::try_into(value.trim().len()) else {
            self.add_error(input, InputError::InvalidLength(min, Some(u32::MAX)));
            return false;
        };

        if (min.is_some() && value_length < min.unwrap()) || (max.is_some() && value_length > max.unwrap()) {
            self.add_error(input, InputError::InvalidLength(min, max));
            return false;
        }

        true
    }

    fn validate_numericality(&mut self, input: Input, _value: &str, _min: Option<&str>, _max: Option<&str>) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_presence(&mut self, input: Input, value: &str) -> bool {
        if value.is_empty() {
            self.add_error(input, InputError::CantBeBlank);
            return false;
        }

        true
    }
}

impl ValidatorTrait<String> for Validator {
    fn validate_absence(&mut self, input: Input, value: String) -> bool {
        self.validate_absence(input, value.as_str())
    }

    fn validate_format(&mut self, input: Input, value: String, with: &Regex) -> bool {
        self.validate_format(input, value.as_str(), with)
    }

    fn validate_length(&mut self, input: Input, value: String, min: Option<u32>, max: Option<u32>) -> bool {
        self.validate_length(input, value.as_str(), min, max)
    }

    fn validate_numericality(&mut self, input: Input, value: String, min: Option<String>, max: Option<String>) -> bool {
        self.validate_numericality(input, value.as_str(), min.as_deref(), max.as_deref())
    }

    fn validate_presence(&mut self, input: Input, value: String) -> bool {
        self.validate_presence(input, value.as_str())
    }
}

impl ValidatorTrait<&String> for Validator {
    fn validate_absence(&mut self, input: Input, value: &String) -> bool {
        self.validate_absence(input, value.as_str())
    }

    fn validate_format(&mut self, input: Input, value: &String, with: &Regex) -> bool {
        self.validate_format(input, value.as_str(), with)
    }

    fn validate_length(&mut self, input: Input, value: &String, min: Option<u32>, max: Option<u32>) -> bool {
        self.validate_length(input, value.as_str(), min, max)
    }

    fn validate_numericality(
        &mut self,
        input: Input,
        value: &String,
        min: Option<&String>,
        max: Option<&String>,
    ) -> bool {
        self.validate_numericality(input, value.as_str(), min.map(|v| v.as_str()), max.map(|v| v.as_str()))
    }

    fn validate_presence(&mut self, input: Input, value: &String) -> bool {
        self.validate_presence(input, value.as_str())
    }
}

impl ValidatorTrait<bool> for Validator {
    fn validate_absence(&mut self, input: Input, value: bool) -> bool {
        if value {
            self.add_error(input, InputError::CantBePresent);
            return false;
        }

        true
    }

    fn validate_format(&mut self, input: Input, _value: bool, _with: &Regex) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_length(&mut self, input: Input, _value: bool, _min: Option<u32>, _max: Option<u32>) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_numericality(&mut self, input: Input, _value: bool, _min: Option<bool>, _max: Option<bool>) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_presence(&mut self, input: Input, value: bool) -> bool {
        if !value {
            self.add_error(input, InputError::CantBeBlank);
            return false;
        }

        true
    }
}

impl<T> ValidatorTrait<Option<T>> for Validator {
    fn validate_absence(&mut self, input: Input, value: Option<T>) -> bool {
        if value.is_some() {
            self.add_error(input, InputError::CantBePresent);
            return false;
        }

        true
    }

    fn validate_format(&mut self, input: Input, _value: Option<T>, _with: &Regex) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_length(&mut self, input: Input, _value: Option<T>, _min: Option<u32>, _max: Option<u32>) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_numericality(
        &mut self,
        input: Input,
        _value: Option<T>,
        _min: Option<Option<T>>,
        _max: Option<Option<T>>,
    ) -> bool {
        self.add_error(input, InputError::IsInvalid);
        false
    }

    fn validate_presence(&mut self, input: Input, value: Option<T>) -> bool {
        if value.is_none() {
            self.add_error(input, InputError::CantBeBlank);
            return false;
        }

        true
    }
}
