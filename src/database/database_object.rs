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
