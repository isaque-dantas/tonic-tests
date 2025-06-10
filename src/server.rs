use crate::temporal_data::temporal_data_server::{TemporalData, TemporalDataServer};
use crate::temporal_data::{TemporalDataRequest, TemporalDataResponse};
use tonic::{Code, Request, Response, Status, transport::Server};

pub mod temporal_data {
    tonic::include_proto!("temporaldata");
}

mod models;
pub mod schema;
mod temporaldata_repository;

#[derive(Debug, Default)]
pub struct MyTemporalData {}

#[tonic::async_trait]
impl TemporalData for MyTemporalData {
    async fn send_data(
        &self,
        request: Request<TemporalDataRequest>,
    ) -> Result<Response<TemporalDataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let inner_request = request.into_inner();

        let saved_data = temporaldata_repository::save_data(inner_request);
        match saved_data {
            Some(saved_data) => {
                let response: TemporalDataResponse = TemporalDataResponse {
                    data: saved_data.iter().map(|d| d.to_message()).collect(),
                };

                Ok(Response::new(response))
            }
            None => Err(Status::new(Code::Unknown, "An unknown error happened")),
        }
    }

    async fn get_data(
        &self,
        _request: Request<TemporalDataRequest>,
    ) -> Result<Response<TemporalDataResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let temporal_data = MyTemporalData::default();

    Server::builder()
        .add_service(TemporalDataServer::new(temporal_data))
        .serve(addr)
        .await?;

    Ok(())
}
