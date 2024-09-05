pub mod database_object;
pub mod error;
mod postgres;

use std::error::Error;
use tonic::async_trait;
use crate::database::error::DatabaseError;
use crate::model::AgendaModel;


#[macro_export]
macro_rules! trace_and_handle_error_database {
    // Match the block of code within the macro invocation
    ({ $($code:tt)* }) => {{
        // Get the current span for tracing
        let current_span = tracing::Span::current();

        // Log the entry into the function
        tracing::info!("Entered function in database module");

        // Execute the provided code within an async block
        let result = async {
            $($code)*
        }.await;

        // Match on the result to handle success and error cases
        match &result {
            Ok(_) => {
                tracing::info!("Function executed successfully");
                current_span.record("success", &true);

            },
            
            Err(DatabaseError::UnknownError{ error: err }) => {
                tracing::error!("Error in function: {}", err);
                current_span.record("error", &format!("{}", err));
            },
            
            Err(err) => {
                tracing::warn!("Error in function: {}", err);
                current_span.record("warn", &format!("{}", err));
            }
        }
        result
    }};
}

#[async_trait]
pub trait Database {
    async fn init_database(self) -> Result<(), Box<dyn Error>>;
    async fn retrieve_from_id(self, id: i64) -> Result<AgendaModel, DatabaseError>;

    async fn retrieve_all(self, page: i64, items: i64) -> Result<(Vec<AgendaModel>, i64, i64), DatabaseError>;

    async fn create_agenda(self, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError>;

    async fn update_agenda(self, id: i64, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError>;

    async fn delete_agenda(self, id: i64) -> Result<(), DatabaseError>;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    // test the trace_and_handle_error_database macro
    #[tokio::test]
    async fn test_trace_and_handle_error_database_ok() {
        let result = trace_and_handle_error_database!({
            Ok::<(), DatabaseError>(())
        });

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trace_and_handle_error_database_unknown_error() {
        let result = trace_and_handle_error_database!({
            Err::<(), DatabaseError>(DatabaseError::UnknownError {error: "error".into()})
        });

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_trace_and_handle_error_database_known_error() {
        let result = trace_and_handle_error_database!({
            Err::<(), DatabaseError>(DatabaseError::UnimplementedError)
        });

        assert!(result.is_err());
    }
}

