use super::ValidationErrors;

#[macro_export]
macro_rules! mut_success_result {
    () => {
        Ok(crate::utils::MutSuccess {
            data: (),
            message: "Record saved successfully.".to_owned(),
        })
    };
    ($data:expr) => {
        Ok(crate::utils::MutSuccess {
            data: $data,
            message: "Record saved successfully.".to_owned(),
        })
    };
    ($data:expr, $message:expr) => {
        Ok(crate::utils::MutSuccess {
            data: $data,
            message: $message,
        })
    };
}

#[macro_export]
macro_rules! mut_error_result {
    () => {
        Err(crate::utils::MutError {
            errors: crate::utils::ValidationErrors::default(),
            message: "Failed to save record.".to_owned(),
        })
    };
    ($errors:expr) => {
        Err(crate::utils::MutError {
            errors: $errors,
            message: "Failed to save record.".to_owned(),
        })
    };
    ($errors:expr, $message:expr) => {
        Err(crate::utils::MutError {
            errors: $errors,
            message: $message,
        })
    };
}

pub type MutResult<T = ()> = Result<MutSuccess<T>, MutError>;

pub struct MutSuccess<T> {
    pub data: T,
    pub message: String,
}

#[derive(Debug)]
pub struct MutError {
    pub errors: ValidationErrors,
    pub message: String,
}

impl From<sqlx::Error> for MutError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            errors: ValidationErrors::default(),
            message: format!("Failed to save record: {}.", error),
        }
    }
}
