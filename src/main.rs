//! signing client
use std::error::Error;
// use std::pin::Pin;
use dotenv::dotenv;
use lazy_static::lazy_static;
// use futures::Stream;
use log::{info, trace};
// use tokio::sync::mpsc;
// use tokio_stream::StreamExt;
// use tonic::{server::ClientStreamingService, transport::Server, Request, Response, Status};
use entropy_grpc as proto;
use proto::{entropy_client::EntropyClient, GetPartyRequest};
use tonic::{Request};

lazy_static! {
    // todo: There's definitely a better way to get my IP address
    static ref MY_IP_ADDRESS: String =
        std::env::var("MY_IP_ADDRESS").expect("Can't read my address");
    static ref GRPC_SERVER_ADDRESS: String =
        std::env::var("GRPC_SERVER_ADDRESS").expect("Can't read gRPC server address");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    let mut client = create_grpc_client().await?;
    let request = Request::new(GetPartyRequest {
        address: MY_IP_ADDRESS.to_string(),
    });
    let mut stream = client.get_party(request).await?.into_inner();
    // todo: why is this Vec<Vec<String>> not Vec<String>
    let addresses = {
        let mut address_list = vec![];
        while let Some(node_address) = stream.message().await? {
            trace!("🔨 Client: got node address: {:?}", node_address);
            address_list.push(node_address.addresses);
        }
        address_list
    };

    info!("🎉 Finish Client: got addresses: {:?}", addresses);
    Ok(())
}

async fn create_grpc_client(
) -> Result<EntropyClient<tonic::transport::Channel>, Box<dyn std::error::Error>> {
    info!("✨ Client: initiating...");
    let channel = tonic::transport::Channel::from_static(&GRPC_SERVER_ADDRESS)
        .connect()
        .await?;
    Ok(EntropyClient::new(channel))
}
