use std::error::Error;
use std::fmt;
use tonic::{Status, Code};


#[derive(Debug, Clone)]
pub enum DatabaseError {
    ConnectionError,
    NotFoundError{id: i64},
    AlreadyExists{error: String},
    UnimplementedError,
    UnknownError{error: String},
}



// Implement fmt::Display trait in order to be a std::error::Error type
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::NotFoundError{id} => write!(f, "the element with id {} does not exists", id),
            DatabaseError::AlreadyExists{error} => write!(f, "the element with id {} already exists", error),
            DatabaseError::UnknownError{error} => write!(f, "internal error: {}", error),
            DatabaseError::ConnectionError => write!(f, "connection error with database"),
            DatabaseError::UnimplementedError => write!(f, "unimplemented error"),
        }
    }
}


// Implement this in order to be able to return an agenda error from any error with '?' operator
impl<T> From<T> for DatabaseError
where
    T: Error + 'static,
{
    fn from(err: T) -> Self {
        DatabaseError::UnknownError{error: err.to_string()}
    }
}



// Implement this trait in order to be a tonic response if error
impl From<DatabaseError> for Status {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFoundError {id: _id} => Status::new(Code::NotFound, err.to_string()),
            DatabaseError::AlreadyExists {error} => Status::new(Code::AlreadyExists, error),
            DatabaseError::UnknownError {error} => Status::new(Code::Internal, error),
            DatabaseError::ConnectionError => Status::new(Code::Unavailable, err.to_string()),
            DatabaseError::UnimplementedError => Status::new(Code::Unimplemented, err.to_string()),
        }
    }
}
