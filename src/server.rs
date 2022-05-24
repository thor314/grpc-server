use dotenv::dotenv;
use entropy_grpc as proto;
use lazy_static::lazy_static;
use log::info;
use proto::{
    entropy_server::{Entropy, EntropyServer},
    GetPartyRequest, GetPartyResponse,
};
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

const N_MESSAGES: usize = 1;
lazy_static! {
    static ref ALICE_IP_ADDRESS: String =
        std::env::var("ALICE_IP_ADDRESS").expect("Can't read my address");
    static ref TEMP_ADDRESS_LIST: Vec<String> =
        ["1", "2", "3"].into_iter().map(|s| s.to_string()).collect();
}

#[derive(Default, Debug)]
pub struct EntropyService {}

#[tonic::async_trait]
impl Entropy for EntropyService {
    type GetPartyStream = ReceiverStream<Result<GetPartyResponse, Status>>;
    /// Return the ip addresses of participating signing nodes
    async fn get_party(
        &self,
        _request: Request<GetPartyRequest>,
    ) -> Result<Response<Self::GetPartyStream>, Status> {
        info!("🧑🏻‍🤝‍🧑🏻 Server: getting signer party addresses...");
        let temp_node_addresses = TEMP_ADDRESS_LIST.to_vec();
        let reply = GetPartyResponse {
            addresses: temp_node_addresses,
        };
        let (tx, rx) = mpsc::channel(N_MESSAGES);
        tx.send(Ok(reply)).await.unwrap();
        // spawn threads if more than 1 message
        // let (tx, rx) = mpsc::channel(N_MESSAGES);
        // tokio::spawn(async move {
        //     tx.send(Ok(reply)).await.unwrap();
        // });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    info!("✨ Starting Server...");
    let alice_addr = std::env::var(ALICE_IP_ADDRESS.to_string())?.parse()?;
    let service = EntropyServer::new(EntropyService::default());
    Server::builder()
        .add_service(service)
        .serve(alice_addr)
        .await?;
    Ok(())
}
