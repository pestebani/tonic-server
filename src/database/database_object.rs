use std::env;
use std::error::Error;
use crate::database::Database;
use crate::database::postgres::PostgresDB;
use crate::database::error::DatabaseError;

#[derive(Debug, Clone)]
pub struct DBLayers {
    postgres_db: Option<PostgresDB>,
}

impl DBLayers {
    fn new() -> Self {
        DBLayers{postgres_db: None}
    }
    
    pub fn get_db_handler(self) -> Result<impl Database, DatabaseError> {
        let db_type = env::var("DATABASE_TYPE")?;
        match db_type.as_str() {
            "postgres" => Ok(self.postgres_db.unwrap()),
            _ => Err(DatabaseError::UnknownError {error: "unknown database type".into()}),
        }
    }

    pub async fn new_db_handler() -> Result<DBLayers, Box<dyn Error>> {
        let db_type = env::var("DATABASE_TYPE")?;
        let mut dbl = DBLayers::new();
        match db_type.as_str() {
            "postgres" => dbl.postgres_db = Some(PostgresDB::new().await?),
            _ => return Err(Box::<dyn Error>::from("unknown database type")),
        };
        
        Ok(dbl)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_new_db_handler_postgres_success() {
        env::set_var("DATABASE_TYPE", "postgres");
        
        let result = DBLayers::new_db_handler().await;

        assert!(result.is_ok());
        let dbl = result.unwrap();
        assert!(dbl.postgres_db.is_some());
        assert!(dbl.get_db_handler().is_ok());
    }

    #[tokio::test]
    async fn test_new_db_handler_unknown_type() {
        env::set_var("DATABASE_TYPE", "invalid_type");

        let result = DBLayers::new_db_handler().await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "unknown database type");
    }
    
    #[tokio::test]
    async fn test_get_db_handler_unknown_type() {
        env::set_var("DATABASE_TYPE", "invalid_type");

        let dbl = DBLayers::new();

        let result = dbl.get_db_handler();

        assert!(result.is_err());
    }
}
