use tonic::{Request, Response, Status};
use crate::agenda::{PingRequest, PingResponse, CreateAgendaRequest, CreateAgendaResponse, GetAgendaRequest, GetAgendaResponse, UpdateAgendaRequest, UpdateAgendaResponse, DeleteAgendaRequest, DeleteAgendaResponse, GetAgendasRequest, GetAgendasResponse};
use crate::agenda::agenda_service_server::{AgendaService};



pub struct CustomAgendaService {}


impl CustomAgendaService {
    pub async fn new() -> Self {
        CustomAgendaService {}
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
        _request: Request<CreateAgendaRequest>,
    ) -> Result<Response<CreateAgendaResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_agenda(
        &self,
        _request: Request<GetAgendaRequest>,
    ) -> Result<Response<GetAgendaResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_agendas(
        &self,
        _request: Request<GetAgendasRequest>
    ) -> Result<Response<GetAgendasResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn update_agenda(
        &self,
        _request: Request<UpdateAgendaRequest>,
    ) -> Result<Response<UpdateAgendaResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn delete_agenda(
        &self,
        _request: Request<DeleteAgendaRequest>,
    ) -> Result<Response<DeleteAgendaResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}
