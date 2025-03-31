use super::ValidationErrors;

pub type MutResult<T> = Result<Success<T>, MutError>;

pub struct MutSuccess<T = ()> {
    message: String,
    data: T,
}

pub struct MutError {
    message: String,
    errors: ValidationErrors,
}
