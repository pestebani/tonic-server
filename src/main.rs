use tonic::transport::Server;
use tokio::{time::Duration, time};
use crate::agenda::agenda_service_server::AgendaServiceServer;

mod service;

pub mod agenda {
    tonic::include_proto!("agenda.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let agenda_service = service::CustomAgendaService::new().await;

    Server::builder()
        .add_service(AgendaServiceServer::new(agenda_service))
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
            time::sleep(Duration::from_secs(1)).await;
        })
        .await?;

    Ok(())
}
