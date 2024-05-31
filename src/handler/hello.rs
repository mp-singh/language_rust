use crate::base::hello_server::Hello;
use crate::hello::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

#[derive(Default, Debug, Clone)]
pub struct HelloServer {}

#[tonic::async_trait]
impl Hello for HelloServer {
    async fn hello_world(
        &self,
        _: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse {
            message: "Hello, World!".to_string(),
        };
        Ok(Response::new(response))
    }
}
