mod common;

use common::maths_client::MathsClient;
use common::{DivRequest, MathsError};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;
use tonic_error::TonicError;

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
        let resp = self
            .client
            .div(req)
            .await
            .map_err(|s| TonicError::from_status(&s).unwrap())?;

        Ok(resp.into_inner().result)
    }
}
