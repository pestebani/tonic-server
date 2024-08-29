use tracing::instrument;
use tonic::{Request, Response, Status, Code};
use crate::agenda::{PingRequest, PingResponse, CreateAgendaRequest, CreateAgendaResponse, GetAgendaRequest, GetAgendaResponse, UpdateAgendaRequest, UpdateAgendaResponse, DeleteAgendaRequest, DeleteAgendaResponse, GetAgendasRequest, GetAgendasResponse};
use crate::agenda::agenda_service_server::{AgendaService};
use crate::database::database_object::DBLayers;
use crate::model::AgendaModel;
use crate::database::Database;

#[derive(Debug)]
pub struct CustomAgendaService {
    pub(crate) database: DBLayers,
}


impl CustomAgendaService {
    pub async fn new() -> Self {
        let database = DBLayers::new_db_handler().await.unwrap();
        database.clone().get_db_handler().unwrap().init_database().await.unwrap();
        CustomAgendaService {
            database: DBLayers::new_db_handler().await.unwrap(),
        }
    }
}

macro_rules! trace_and_handle_error {
    // Match the block of code within the macro invocation
    ({ $($code:tt)* }) => {{
        // Get the current span for tracing
        let current_span = tracing::Span::current();

        // Log the entry into the function
        tracing::info!("Entered function in service module");

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
            
            Err(err) if err.code() == Code::Internal => {
                tracing::error!("Error in function: {}", err.to_string());
                current_span.record("error", &format!("{}", err.to_string()));
            },
            Err(err) => {
                tracing::warn!("Error in function: {}", err.to_string());
                current_span.record("warn", &format!("{}", err.to_string()));
            }
        }
        result
    }};
}


#[tonic::async_trait]
impl AgendaService for CustomAgendaService {
    #[instrument(level = "info", target = "service::ping")]
    async fn ping(
        &self,
        _request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        trace_and_handle_error!({
            let reply = PingResponse {
                response: "pong".into(),
            };
    
            Ok::<Response<PingResponse>, Status>(Response::new(reply))
        })
    }

    #[instrument(level = "info", target = "service::create_agenda")]
    async fn create_agenda(
        &self,
        request: Request<CreateAgendaRequest>,
    ) -> Result<Response<CreateAgendaResponse>, Status> {
        trace_and_handle_error!({
            let new_agenda: AgendaModel = self.database
                .clone()
                .get_db_handler()
                .unwrap()
                .create_agenda(AgendaModel::from_proto(request.into_inner().agenda.unwrap()))
                .await?;
    
            Ok::<Response<CreateAgendaResponse>, Status>(Response::new(CreateAgendaResponse {
                agenda: Some(new_agenda.to_proto())
            }))
        })
    }

    #[instrument(level = "info", target = "service::get_agenda")]
    async fn get_agenda(
        &self,
        request: Request<GetAgendaRequest>,
    ) -> Result<Response<GetAgendaResponse>, Status> {
        trace_and_handle_error!({
            let new_agenda: AgendaModel = self.database
                .clone()
                .get_db_handler()
                .unwrap()
                .retrieve_from_id(request.into_inner().id)
                .await?;
    
            Ok::<Response<GetAgendaResponse>, Status>(Response::new(GetAgendaResponse {
                agenda: Some(new_agenda.to_proto())
            }))
        })
    }

    #[instrument(level = "info", target = "service::get_agendas")]
    async fn get_agendas(
        &self,
        request: Request<GetAgendasRequest>
    ) -> Result<Response<GetAgendasResponse>, Status> {
        trace_and_handle_error!({
            let message :GetAgendasRequest = request.into_inner().clone();
            let (agendas, page, items) = self.database
                .clone()
                .get_db_handler()
                .unwrap()
                .retrieve_all(message.page, message.items)
                .await?;

            Ok::<Response<GetAgendasResponse>, Status>(Response::new(GetAgendasResponse {
                agendas: agendas.into_iter().map(|agenda| agenda.to_proto()).collect(),
                next_page: page,
                total: items,
            }))
        })
    }

    #[instrument(level = "info", target = "service::update_agenda")]
    async fn update_agenda(
        &self,
        request: Request<UpdateAgendaRequest>,
    ) -> Result<Response<UpdateAgendaResponse>, Status> {
        trace_and_handle_error!({
            let message :UpdateAgendaRequest = request.into_inner().clone();
            let new_agenda: AgendaModel = self.database
                .clone()
                .get_db_handler()
                .unwrap()
                .update_agenda(message.id, AgendaModel::from_proto(message.agenda.unwrap()))
                .await?;
            Ok::<Response<UpdateAgendaResponse>, Status>(Response::new(UpdateAgendaResponse {
                agenda: Some(new_agenda.to_proto())
            }))
        })
    }

    #[instrument(level = "info", target = "service::delete_agenda")]
    async fn delete_agenda(
        &self,
        request: Request<DeleteAgendaRequest>,
    ) -> Result<Response<DeleteAgendaResponse>, Status> {
        trace_and_handle_error!({
            self.database
                .clone()
                .get_db_handler()
                .unwrap()
                .delete_agenda(request.into_inner().id)
                .await?;
            Ok::<Response<DeleteAgendaResponse>, Status>(Response::new(DeleteAgendaResponse {}))
        })
    }
}
