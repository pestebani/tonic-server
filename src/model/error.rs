use std::error::Error;
use std::fmt;
use tonic::{Code, Status};

#[derive(Debug, Clone, PartialEq)]
pub enum ModelError{
    EmptyInput,
    UnknownError{error: String},
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelError::EmptyInput => write!(f, "missing agenda object in input"),
            ModelError::UnknownError{error} => write!(f, "internal error: {}", error),
        }
    }
}


// Implement this in order to be able to return an agenda error from any error with '?' operator
impl<T> From<T> for ModelError
where
    T: Error + 'static,
{
    fn from(err: T) -> Self {
        ModelError::UnknownError{error: err.to_string()}
    }
}


// Implement this trait in order to be a tonic response if error
impl From<ModelError> for Status {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::UnknownError {error} => Status::new(Code::Internal, error),
            ModelError::EmptyInput => Status::new(Code::InvalidArgument, err.to_string()),
        }
    }
}
