use dotenv::dotenv;
use entropy_grpc as proto;
use lazy_static::lazy_static;
use log::info;
use proto::{entropy_server::{Entropy, EntropyServer}, GetPartyRequest, GetPartyResponse};
use std::error::Error;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

lazy_static! {
    static ref ALICE_IP_ADDRESS: String =
        std::env::var("ALICE_IP_ADDRESS").expect("Can't read my address");
    static ref TEMP_ADDRESS_LIST: Vec<String> =
        ["1", "2", "3"].into_iter().map(|s| s.to_string()).collect();
}

#[derive(Default, Debug)]
pub struct EntropyService {
    addresses: Vec<String>,
}

#[tonic::async_trait]
impl Entropy for EntropyService {
    type GetPartyStream = ReceiverStream<Result<GetPartyResponse, Status>>;
    async fn get_party(
        &self,
        request: Request<GetPartyRequest>,
    ) -> Result<Response<Self::GetPartyStream>, Status> {

        todo!();
    }
}

impl EntropyService {
    pub(crate) fn new(addresses: Vec<String>) -> Self {
        Self { addresses }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    info!("✨ Starting Server");
    let alice_addr = std::env::var(ALICE_IP_ADDRESS.to_string())?.parse()?;
    let service = EntropyServer::new(EntropyService::new(TEMP_ADDRESS_LIST.to_vec()));
    Server::builder().add_service(service).serve(alice_addr).await?;
    Ok(())
}
