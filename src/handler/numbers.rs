use crate::{add_numbers::AddNumbersRequest, base::numbers_server::Numbers};

use crate::add_numbers::AddNumbersResponse;
use tonic::{Request, Response, Status};

#[derive(Default, Debug, Clone)]
pub struct NumbersServer {}

#[tonic::async_trait]
impl Numbers for NumbersServer {
    async fn add_numbers(
        &self,
        req: Request<AddNumbersRequest>,
    ) -> Result<Response<AddNumbersResponse>, Status> {
        let request = req.into_inner();
        let response = AddNumbersResponse {
            message: request.a + request.b,
        };
        Ok(Response::new(response))
    }
}
