use std::error::Error;
use std::fmt;
use tonic::{Status, Code};


#[derive(Debug, Clone, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_error_display() {
        let error = DatabaseError::NotFoundError{id: 1};
        assert_eq!(error.to_string(), "the element with id 1 does not exists");
    }

    #[tokio::test]
    async fn test_database_error_from() {
        let error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let db_error = DatabaseError::from(error);
        assert_eq!(db_error, DatabaseError::UnknownError{error: "test".to_string()});
    }

    #[tokio::test]
    async fn test_database_error_into() {
        let error = DatabaseError::NotFoundError{id: 1};
        let status = Status::from(error);
        assert_eq!(status.code(), Code::NotFound);
        assert_eq!(status.message(), "the element with id 1 does not exists");
    }
    
    #[tokio::test]
    async fn test_database_error_into_unimplemented() {
        let error = DatabaseError::UnimplementedError;
        let status = Status::from(error);
        assert_eq!(status.code(), Code::Unimplemented);
        assert_eq!(status.message(), "unimplemented error");
    }
    
    #[tokio::test]
    async fn test_database_error_into_connection() {
        let error = DatabaseError::ConnectionError;
        let status = Status::from(error);
        assert_eq!(status.code(), Code::Unavailable);
        assert_eq!(status.message(), "connection error with database");
    }
    
    #[tokio::test]
    async fn test_database_error_into_already_exists() {
        let error = DatabaseError::AlreadyExists{error: "error".to_string()};
        let status = Status::from(error);
        assert_eq!(status.code(), Code::AlreadyExists);
        assert_eq!(status.message(), "error");
    }
    
    #[tokio::test]
    async fn test_database_error_into_unknown() {
        let error = DatabaseError::UnknownError{error: "error".to_string()};
        let status = Status::from(error);
        assert_eq!(status.code(), Code::Internal);
        assert_eq!(status.message(), "error");
    }
    
    #[tokio::test]
    async fn test_database_error_into_not_found() {
        let error = DatabaseError::NotFoundError{id: 0};
        let status = Status::from(error);
        assert_eq!(status.code(), Code::NotFound);
        assert_eq!(status.message(), "the element with id 0 does not exists");
    }
}
