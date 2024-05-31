use crate::base::health_server::Health;
use crate::health::{HealthCheckResponse, ServingStatus};
use log::debug;
use tonic::{Request, Response, Status};

#[derive(Default, Debug, Clone)]
pub struct HealthServer {}

#[tonic::async_trait]
impl Health for HealthServer {
    async fn check(&self, _: Request<()>) -> Result<Response<HealthCheckResponse>, Status> {
        debug!("health check");
        let status = ServingStatus::Good;
        let response = HealthCheckResponse {
            status: status as i32,
        };
        debug!("health check response: {:?}", response);
        Ok(Response::new(response))
    }
}
