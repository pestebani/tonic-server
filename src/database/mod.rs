pub mod database_object;
pub mod error;
mod postgres;

use std::error::Error;
use tonic::async_trait;
use crate::database::error::DatabaseError;
use crate::model::AgendaModel;

#[async_trait]
pub trait Database {
    async fn init_database(self) -> Result<(), Box<dyn Error>>;
    async fn retrieve_from_id(self, id: i64) -> Result<AgendaModel, DatabaseError>;

    async fn retrieve_all(self, page: i64, items: i64) -> Result<(Vec<AgendaModel>, i64, i64), DatabaseError>;

    async fn create_agenda(self, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError>;

    async fn update_agenda(self, id: i64, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError>;

    async fn delete_agenda(self, id: i64) -> Result<(), DatabaseError>;
}

