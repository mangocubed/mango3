use super::ValidationErrors;

#[macro_export]
macro_rules! mut_result {
    ($result:expr) => {
        match $result {
            Ok(data) => $crate::mut_success!(data),
            Err(_) => $crate::mut_error!(),
        }
    };
}

#[macro_export]
macro_rules! mut_success {
    () => {
        Ok($crate::utils::MutSuccess {
            data: (),
            message: "Record saved successfully.".to_owned(),
        })
    };
    ($data:expr) => {
        Ok($crate::utils::MutSuccess {
            data: $data,
            message: "Record saved successfully.".to_owned(),
        })
    };
}

#[macro_export]
macro_rules! mut_error {
    () => {
        Err($crate::utils::MutError {
            errors: $crate::utils::ValidationErrors::default(),
            message: "Failed to save record.".to_owned(),
        })
    };
    ($errors:expr) => {
        Err($crate::utils::MutError {
            errors: $errors,
            message: "Failed to save record.".to_owned(),
        })
    };
}

pub type MutResult<T = ()> = Result<MutSuccess<T>, MutError>;

pub struct MutSuccess<T> {
    pub data: T,
    pub message: String,
}

#[derive(Debug, Default)]
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

impl From<std::io::Error> for MutError {
    fn from(error: std::io::Error) -> Self {
        Self {
            errors: ValidationErrors::default(),
            message: format!("Failed to save record: {}.", error),
        }
    }
}
