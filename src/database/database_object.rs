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
