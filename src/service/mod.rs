use tonic::{Request, Response, Status};
use crate::agenda::{PingRequest, PingResponse, CreateAgendaRequest, CreateAgendaResponse, GetAgendaRequest, GetAgendaResponse, UpdateAgendaRequest, UpdateAgendaResponse, DeleteAgendaRequest, DeleteAgendaResponse, GetAgendasRequest, GetAgendasResponse};
use crate::agenda::agenda_service_server::{AgendaService};
use crate::database::database_object::DBLayers;
use crate::model::AgendaModel;
use crate::database::Database;

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


#[tonic::async_trait]
impl AgendaService for CustomAgendaService {
    async fn ping(
        &self,
        _request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse {
            response: "pong".into(),
        };

        Ok(Response::new(reply))
    }

    async fn create_agenda(
        &self,
        request: Request<CreateAgendaRequest>,
    ) -> Result<Response<CreateAgendaResponse>, Status> {
        let new_agenda: AgendaModel = self.database
            .clone()
            .get_db_handler()
            .unwrap()
            .create_agenda(AgendaModel::from_proto(request.into_inner().agenda.unwrap()))
            .await?;

        Ok(Response::new(CreateAgendaResponse {
            agenda: Some(new_agenda.to_proto())
        }))
    }

    async fn get_agenda(
        &self,
        request: Request<GetAgendaRequest>,
    ) -> Result<Response<GetAgendaResponse>, Status> {
        let new_agenda: AgendaModel = self.database
            .clone()
            .get_db_handler()
            .unwrap()
            .retrieve_from_id(request.into_inner().id)
            .await?;

        Ok(Response::new(GetAgendaResponse {
            agenda: Some(new_agenda.to_proto())
        }))
    }

    async fn get_agendas(
        &self,
        request: Request<GetAgendasRequest>
    ) -> Result<Response<GetAgendasResponse>, Status> {
        let message :GetAgendasRequest = request.into_inner().clone();
        let (agendas, page, items) = self.database
            .clone()
            .get_db_handler()
            .unwrap()
            .retrieve_all(message.page, message.items)
            .await?;

        Ok(Response::new(GetAgendasResponse {
            agendas: agendas.into_iter().map(|agenda| agenda.to_proto()).collect(),
            next_page: page,
            total: items,
        }))
    }

    async fn update_agenda(
        &self,
        request: Request<UpdateAgendaRequest>,
    ) -> Result<Response<UpdateAgendaResponse>, Status> {
        let message :UpdateAgendaRequest = request.into_inner().clone();
        let new_agenda: AgendaModel = self.database
            .clone()
            .get_db_handler()
            .unwrap()
            .update_agenda(message.id, AgendaModel::from_proto(message.agenda.unwrap()))
            .await?;
        Ok(Response::new(UpdateAgendaResponse {
            agenda: Some(new_agenda.to_proto())
        }))
    }

    async fn delete_agenda(
        &self,
        request: Request<DeleteAgendaRequest>,
    ) -> Result<Response<DeleteAgendaResponse>, Status> {
        self.database
            .clone()
            .get_db_handler()
            .unwrap()
            .delete_agenda(request.into_inner().id)
            .await?;
        Ok(Response::new(DeleteAgendaResponse {}))
    }
}
