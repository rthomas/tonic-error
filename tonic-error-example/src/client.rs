mod common;

use common::maths_client::MathsClient;
use common::{DivRequest, MathsError};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Code;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect().await?;

    let res = client.div(10, 2).await;
    println!("RESPONSE={:?}", res);

    let res = client.div(10, 0).await;
    println!("RESPONSE={:?}", res);

    Ok(())
}

struct Client {
    client: MathsClient<Channel>,
}

impl Client {
    pub async fn connect() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: MathsClient::connect("http://[::1]:50051").await?,
        })
    }

    pub async fn div(&mut self, a: i32, b: i32) -> Result<f64, MathsError> {
        let req = Request::new(DivRequest { a, b });

        let resp = match self.client.div(req).await {
            Ok(r) => r,
            Err(e) => match e.code() {
                Code::Internal => {
                    return Err(e.try_into().expect("could not convert status to error"))
                }
                _ => panic!("error making rpc call: {e}"),
            },
        };

        Ok(resp.into_inner().result)
    }
}
