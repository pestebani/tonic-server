use std::env;
use std::error::Error;
use crate::database::Database;
use crate::database::postgres::PostgresDB;

#[derive(Debug, Clone)]
pub enum DBLayers {
    Postgres(PostgresDB),
}


impl DBLayers {
    pub async fn new_db_handler() -> Result<Self, Box<dyn Error>> {
        let db_type = env::var("DATABASE_TYPE")?;
        match db_type.as_str() {
            "postgres" => Ok(DBLayers::Postgres(PostgresDB::new().await?)),
            _ => Err(Box::<dyn Error>::from(format!("unknown database type {db_type}"))),
        }
    }
    
    pub fn get_db_handler(&self) -> &impl Database {
        match self {
            DBLayers::Postgres(db) => db,
        }
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
