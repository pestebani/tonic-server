use std::error::Error;
use tonic::async_trait;
use crate::database::Database;
use crate::database::error::DatabaseError;
use crate::model::AgendaModel;

#[derive(Debug, Clone)]
pub struct PostgresDB {}


// This cannot return an error if we want to start the server
impl PostgresDB {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(PostgresDB{})
    }
}


#[async_trait]
impl Database for PostgresDB {
    // Returns OK because we don't want to return an error when starting the server
    async fn init_database(self) -> Result<(), Box<dyn Error>>{
        Ok(())
    }
    async fn retrieve_from_id(self, _id: i64) -> Result<AgendaModel, DatabaseError> {
        Err(DatabaseError::UnimplementedError)
    }

    async fn retrieve_all(self, _page: i64, _items: i64) -> Result<(Vec<AgendaModel>, i64, i64), DatabaseError> {
        Err(DatabaseError::UnimplementedError)
    }

    async fn create_agenda(self, _agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        Err(DatabaseError::UnimplementedError)
    }

    async fn update_agenda(self, _id: i64, _agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        Err(DatabaseError::UnimplementedError)
    }

    async fn delete_agenda(self, _id: i64) -> Result<(), DatabaseError> {
        Err(DatabaseError::UnimplementedError)
    }
}
