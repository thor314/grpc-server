use std::error::Error;

use entropy_grpc as proto;
use proto::{ GetPartyRequest, GetPartyResponse, entropy_server::Entropy};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request,Response, Status};

#[derive(Default,Debug)]
pub struct EntropyService{}

#[tonic::async_trait]
impl Entropy for EntropyService{
    type GetPartyStream = ReceiverStream<Result<GetPartyResponse,Status>>;
    async fn get_party(&self, request: Request<GetPartyRequest>)-> Result<Response<Self::GetPartyStream>,Status>{
        todo!();
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}