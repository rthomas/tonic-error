mod common;

use common::maths_server::{Maths, MathsServer};
use common::MathsError;
use common::{DivRequest, DivResponse};
use tonic::{transport::Server, Request, Response, Status};
use tonic_error::TonicError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let maths_service = MathsService::default();

    Server::builder()
        .add_service(MathsServer::new(maths_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
struct MathsService {}

#[tonic::async_trait]
impl Maths for MathsService {
    async fn div(&self, req: Request<DivRequest>) -> Result<Response<DivResponse>, Status> {
        let req = req.into_inner();
        if req.b == 0 {
            return Err(MathsError::DivByZero(req.a, req.b).to_status());
        }
        let result = req.a as f64 / req.b as f64;
        Ok(Response::new(DivResponse { result }))
    }
}
