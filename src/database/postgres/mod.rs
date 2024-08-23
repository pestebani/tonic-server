use std::env;
use std::env::VarError;
use std::error::Error;
use sqlx::{Pool, Row};
use sqlx_postgres::{PgQueryResult, PgRow, Postgres};
use tonic::async_trait;
use crate::database::Database;
use crate::database::error::DatabaseError;
use crate::model::AgendaModel;

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
    async fn init_database(self) -> Result<(), Box<dyn Error>>{
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
    async fn retrieve_from_id(self, id: i64) -> Result<AgendaModel, DatabaseError> {
        let query = "SELECT id, name, phone, email FROM my_table WHERE id=$1";
        let select_query = sqlx::query(query)
            .bind(id);

        let res_model = execute_query_return_agenda!(select_query, &self.pool);

        convert_postgres_result_to_database_result(res_model, Some(id), None)
    }

    async fn retrieve_all(self, page: i64, items: i64) -> Result<(Vec<AgendaModel>, i64, i64), DatabaseError> {
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
    }

    async fn create_agenda(self, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        let query = "INSERT INTO my_table (name, phone, email) VALUES ($1, $2, $3) RETURNING id, name, phone, email";
        let insert_element_query = sqlx::query(query)
            .bind(agenda.name.clone())
            .bind(agenda.phone.clone())
            .bind(agenda.email.clone());


        let res_model = execute_query_return_agenda!(insert_element_query, &self.pool);
        convert_postgres_result_to_database_result(res_model, None, Some(agenda))
    }

    async fn update_agenda(self, id: i64, agenda: AgendaModel) -> Result<AgendaModel, DatabaseError> {
        let query = "UPDATE my_table SET name=$1, phone=$2, email=$3 WHERE id=$4 RETURNING id, name, phone, email";
        let updated_elements_query = sqlx::query(query)
            .bind(agenda.name.clone())
            .bind(agenda.phone.clone())
            .bind(agenda.email.clone())
            .bind(id);

        let res_model = execute_query_return_agenda!(updated_elements_query, &self.pool);

        convert_postgres_result_to_database_result(res_model, Some(id), Some(agenda))
    }

    async fn delete_agenda(self, id: i64) -> Result<(), DatabaseError> {
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
    }
}
