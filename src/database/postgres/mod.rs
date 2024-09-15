use std::env;
use std::env::VarError;
use std::error::Error;
use sqlx::{Pool, Row};
use sqlx_postgres::{PgQueryResult, PgRow, Postgres};
use tonic::async_trait;
use tracing::instrument;
use crate::database::Database;
use crate::database::error::DatabaseError;
use crate::model::AgendaModel;
use crate::trace_and_handle_error_database;

#[derive(Debug, Clone)]
pub struct PostgresDB {
    pool: sqlx::PgPool,
}


// This cannot return an error if we want to start the server
impl PostgresDB {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conn_url = match env::var("DATABASE_URL") {
            Ok(conn) => conn,
            Err(VarError::NotPresent) => "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            Err(_) => return Err(Box::<dyn Error>::from("error retrieving from env variable")),
        };

        let pool: Pool<Postgres> = sqlx::PgPool::connect(&conn_url).await?;

        Ok(PostgresDB{pool})
    }
}


fn convert_postgres_result_to_database_result<T>(result: Result<T, sqlx::error::Error>, id: Option<i64>, agenda: Option<AgendaModel>) -> Result<T, DatabaseError> {
    let agenda_name = agenda.map(|a| a.name).unwrap_or("".to_string());
    match result {
        Ok(m) => Ok(m),
        Err(sqlx::error::Error::RowNotFound) => Err(DatabaseError::NotFoundError {id: id.unwrap_or(0)}),
        Err(sqlx::error::Error::Database(error)) => {
            match error.constraint(){
                Some("my_table_pk_1") => Err(DatabaseError::AlreadyExists {error: format!("It already exists an entry with name {agenda_name}")}),
                Some(_) => Err(DatabaseError::AlreadyExists {error: error.to_string()}),
                None => Err(DatabaseError::UnknownError {error: error.to_string()})
            }
        }
        Err(error) => Err(DatabaseError::UnknownError {error: error.to_string()}),
    }
}


macro_rules! execute_query_return_agenda {
    ($query:expr, $pool:expr) => {
        $query.map(
            |row: PgRow| AgendaModel {
                id: row.get::<i64, &str>("id") as i64,
                name: row.get("name"),
                phone: row.get("phone"),
                email: row.get("email")
            }
        )
            .fetch_one($pool)
            .await
    }
}


#[async_trait]
impl Database for PostgresDB {
    // Returns OK because we don't want to return an error when starting the server
    async fn init_database(&self) -> Result<(), Box<dyn Error>>{
        let query_table_exists = "CREATE TABLE IF NOT EXISTS my_table (
	id BIGSERIAL PRIMARY KEY,
	name varchar NOT NULL,
    phone varchar NOT NULL,
    email varchar NOT NULL,
	CONSTRAINT my_table_pk_1 UNIQUE (name)
);";
        sqlx::query(query_table_exists)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    #[instrument(level = "info")]
    async fn retrieve_from_id(&self, id: i64) -> Result<AgendaModel, DatabaseError> {
        trace_and_handle_error_database!({
            let query = "SELECT id, name, phone, email FROM my_table WHERE id=$1";
            let select_query = sqlx::query(query)
                .bind(id);
    
            let res_model = execute_query_return_agenda!(select_query, &self.pool);
    
            convert_postgres_result_to_database_result(res_model, Some(id), None)
        })
    }

    #[instrument(level = "info")]
    async fn retrieve_all(&self, page: i64, items: i64) -> Result<(Vec<AgendaModel>, i64, i64), DatabaseError> {
        trace_and_handle_error_database!({
            let query = "SELECT id, name, phone, email, (SELECT COUNT(*) FROM my_table) AS total_count FROM my_table ORDER BY id LIMIT $1 OFFSET $2";
            let select_query = sqlx::query(query)
                .bind(items)
                .bind((page - 1) * items);
    
            let mut total_count: i64 = 0;
    
            let return_function = |row: PgRow| {
                total_count = row.get("total_count");
                AgendaModel {
                    id: row.get("id"),
                    name: row.get("name"),
                    phone: row.get("phone"),
                    email: row.get("email")
                }
            };
    
            let agenda_models = select_query.map(return_function)
                .fetch_all(&self.pool)
                .await?;
    
            Ok(
                (
                    agenda_models,
                    if (total_count as u64).div_ceil(items as u64) as i64 > page { page + 1 } else { 0 },
                    total_count,
                )
            )
        })
    }

    #[instrument(level = "info")]
    async fn create_agenda(&self, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        trace_and_handle_error_database!({
            let query = "INSERT INTO my_table (name, phone, email) VALUES ($1, $2, $3) RETURNING id, name, phone, email";
            let insert_element_query = sqlx::query(query)
                .bind(agenda.name.clone())
                .bind(agenda.phone.clone())
                .bind(agenda.email.clone());
    
            let res_model = execute_query_return_agenda!(insert_element_query, &self.pool);
            convert_postgres_result_to_database_result(res_model, None, Some(agenda))
        })
    }

    #[instrument(level = "info")]
    async fn update_agenda(&self, id: i64, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        trace_and_handle_error_database!({
            let query = "UPDATE my_table SET name=$1, phone=$2, email=$3 WHERE id=$4 RETURNING id, name, phone, email";
            let updated_elements_query = sqlx::query(query)
                .bind(agenda.name.clone())
                .bind(agenda.phone.clone())
                .bind(agenda.email.clone())
                .bind(id);
    
            let res_model = execute_query_return_agenda!(updated_elements_query, &self.pool);
    
            convert_postgres_result_to_database_result(res_model, Some(id), Some(agenda))
        })
    }

    #[instrument(level = "info")]
    async fn delete_agenda(&self, id: i64) -> Result<(), DatabaseError> {
        trace_and_handle_error_database!({
            let query = "DELETE from my_table WHERE id = $1";
            let deleted_elements_query: PgQueryResult = sqlx::query(query)
                .bind(id)
                .execute(&self.pool)
                .await?;
    
            if deleted_elements_query.rows_affected() < 1 {
                Err(DatabaseError::NotFoundError {id})
            } else {
                Ok(())
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::error::Error;

    async fn empty_database() -> Result<PostgresDB, Box<dyn std::error::Error>> {
        let db = PostgresDB::new().await?;
        let query = "DELETE from my_table";
        sqlx::query(query)
            .execute(&db.pool)
            .await?;
        Ok(db)
    }

    #[tokio::test]
    async fn test_convert_postgres_result_to_database_result_ok() {
        let result = convert_postgres_result_to_database_result(Ok(1), None, None);
        assert_eq!(result, Ok(1));
    }

    #[tokio::test]
    async fn test_convert_postgres_result_to_database_result_row_not_found() {
        let result = convert_postgres_result_to_database_result::<()>(Err(Error::RowNotFound), Some(1), None);
        assert_eq!(result, Err(DatabaseError::NotFoundError {id: 1}));
    }

    #[tokio::test]
    async fn test_convert_postgres_result_to_database_result_unknown_error() {
        let result = convert_postgres_result_to_database_result::<()>(Err(Error::ColumnNotFound("Demo error".to_string())), None, None);
        assert_eq!(result, Err(DatabaseError::UnknownError {error: "no column found for name: Demo error".to_string()}));
    }

    #[tokio::test]
    async fn test_new_postgres_db_success() {
        let result = PostgresDB::new().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new_postgres_db_error() {
        let old_env_var = env::var("DATABASE_URL").unwrap_or("".to_string());
        env::set_var("DATABASE_URL", "invalid_url");

        let result = PostgresDB::new().await;

        assert!(result.is_err());

        env::set_var("DATABASE_URL", old_env_var);
    }

    #[tokio::test]
    async fn test_init_database_success() {
        let db = PostgresDB::new().await.unwrap();

        let result = db.init_database().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_retrieve_from_id_success() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();

        empty_database().await.unwrap();

        let model = AgendaModel {
            id: 0,
            name: "test".to_string(),
            phone: "123456789".to_string(),
            email: "test_email@test.com".to_string()
        };

        let result_id = db.clone()
            .create_agenda(model.clone()).await;

        assert!(result_id.is_ok());
        let model_created = result_id.unwrap();

        assert!(model_created.id > 0);
        assert_eq!(model_created.name, model.name);
        assert_eq!(model_created.phone, model.phone);
        assert_eq!(model_created.email, model.email);

        let result = db.retrieve_from_id(model_created.id).await;

        assert!(result.is_ok());

        let model_retrieved = result.unwrap();

        assert_eq!(model_retrieved, model_created);
    }

    #[tokio::test]
    async fn test_insert_retrieve_all_success() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let model1 = AgendaModel {
            id: 0,
            name: "test_1".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_1@test.com".to_string(),
        };

        let model2 = AgendaModel {
            id: 0,
            name: "test_2".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_2@test.com".to_string(),
        };

        let model3 = AgendaModel {
            id: 0,
            name: "test_3".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_3@test.com".to_string(),
        };

        let result_id1 = db.clone()
            .create_agenda(model1.clone()).await;

        assert!(result_id1.is_ok());

        let result_id2 = db.clone()
            .create_agenda(model2.clone()).await;

        assert!(result_id2.is_ok());

        let result_id3 = db.clone()
            .create_agenda(model3.clone()).await;

        assert!(result_id3.is_ok());

        let result = db.retrieve_all(1, 2).await;
        assert!(result.is_ok());

        let (models, next_page, total_count) = result.unwrap();
        assert_eq!(models.len(), 2);
        assert_eq!(next_page, 2);
        assert_eq!(total_count, 3);
    }

    #[tokio::test]
    async fn test_insert_update_success() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let model = AgendaModel {
            id: 0,
            name: "test".to_string(),
            phone: "123456789".to_string(),
            email: "test_email@test.com".to_string(),
        };

        let result_id = db.clone()
            .create_agenda(model.clone()).await;

        assert!(result_id.is_ok());

        let new_model = AgendaModel {
            id: result_id.unwrap().id,
            name: "new_test".to_string(),
            phone: "987654321".to_string(),
            email: "another_test_email@test.com".to_string(),
        };

        let res_update_model = db.clone()
            .update_agenda(new_model.id, new_model.clone()).await;

        assert!(res_update_model.is_ok());
        let updated_model = res_update_model.unwrap();
        assert_eq!(updated_model, new_model);

    }

    #[tokio::test]
    async fn delete_agenda_success() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let model = AgendaModel {
            id: 0,
            name: "test".to_string(),
            phone: "123456789".to_string(),
            email: "test_email@test.com".to_string(),
        };

        let result_id = db.clone()
            .create_agenda(model.clone()).await;

        assert!(result_id.is_ok());
        let inserted_id = result_id.unwrap().id;

        let result = db.clone()
            .delete_agenda(inserted_id).await;

        assert!(result.is_ok());

        let result_retrieve = db.clone()
            .retrieve_from_id(inserted_id).await;

        assert!(result_retrieve.is_err());
        let _ = result_retrieve.map_err(|e| {
            assert_eq!(e, DatabaseError::NotFoundError {id: inserted_id});
        });
    }
    
    #[tokio::test]
    async fn delete_agenda_not_found() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let result = db.clone()
            .delete_agenda(1).await;

        assert!(result.is_err());
        let _ = result.map_err(|e| {
            assert_eq!(e, DatabaseError::NotFoundError {id: 1});
        });
    }
    
    #[tokio::test]
    async fn test_insert_already_exists() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let model1 = AgendaModel {
            id: 0,
            name: "test".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_1@test.com".to_string(),
        };

        let model2 = AgendaModel {
            id: 0,
            name: "test".to_string(),
            phone: "987654321".to_string(),
            email: "test_email_2@test.com".to_string(),
        };
        
        let result_id1 = db.clone()
            .create_agenda(model1.clone()).await;
        
        assert!(result_id1.is_ok());
        
        let result_id2 = db.clone()
            .create_agenda(model2.clone()).await;
        
        assert!(result_id2.is_err());
        let _ = result_id2.map_err(|e| {
            assert_eq!(e, DatabaseError::AlreadyExists {error: "It already exists an entry with name test".to_string()});
        });
    }
    
    #[tokio::test]
    async fn test_update_already_exists() {
        let db = PostgresDB::new().await.unwrap();
        db.clone().init_database().await.unwrap();
        empty_database().await.unwrap();

        let model1 = AgendaModel {
            id: 0,
            name: "test_1".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_1@test.com".to_string(),
        };

        let model2 = AgendaModel {
            id: 0,
            name: "test_2".to_string(),
            phone: "987654321".to_string(),
            email: "test_email_2@test.com".to_string(),
        };

        let result_id1 = db.clone()
            .create_agenda(model1.clone()).await;

        assert!(result_id1.is_ok());

        let result_id2 = db.clone()
            .create_agenda(model2.clone()).await;

        assert!(result_id2.is_ok());

        let model1_update = AgendaModel {
            id: 0,
            name: "test_2".to_string(),
            phone: "123456789".to_string(),
            email: "test_email_1@test.com".to_string(),
        };
        
        let result_update = db.clone()
            .update_agenda(result_id1.unwrap().id, model1_update.clone()).await;
        
        assert!(result_update.is_err());
        let _ = result_update.map_err(|e| {
            assert_eq!(e, DatabaseError::AlreadyExists {error: "It already exists an entry with name test_2".to_string()});
        });
    }
}
