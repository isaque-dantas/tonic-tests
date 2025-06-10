use temporal_data::{TemporalDataRequest};
use crate::temporal_data::temporal_data_client::TemporalDataClient;
use crate::temporal_data::TemporalDataMessage;

pub mod temporal_data {
    tonic::include_proto!("temporaldata");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemporalDataClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TemporalDataRequest {
        data: vec!(
            TemporalDataMessage {timestamp: String::from("2020-04-14T12:25:44.323"), value: 4.9},
            TemporalDataMessage {timestamp: String::from("2020-04-14T12:26:43.701"), value: 7.3},
            TemporalDataMessage {timestamp: String::from("2020-04-14T12:24:26.421"), value: 6.45},
            TemporalDataMessage {timestamp: String::from("2020-04-14T12:26:56.998"), value: 3.6},
        )
    });

    let response = client.get_data(request).await?;
    let response_data = response.into_inner().data;
    
    println!("{}", response_data.len());
    
    for message in response_data {
        println!("{} - {}", message.timestamp, message.value);
    }

    Ok(())
}
