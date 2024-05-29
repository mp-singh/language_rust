use crate::add_numbers::{AddNumbersRequest, AddNumbersResponse};
use crate::base::root_server::Root;
use crate::db::DB;
use crate::hello::{HelloRequest, HelloResponse};
use crate::language::{GetTranslationByIdRequest, Translation, TranslationsResponse};
use log::{error, warn};
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct MyServer {
    pub db: DB,
}

impl MyServer {
    pub fn new(db: DB) -> Self {
        MyServer { db }
    }
}

#[tonic::async_trait]
impl Root for MyServer {
    async fn hello_world(
        &self,
        _: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse {
            message: "Hello, World!".to_string(),
        };
        Ok(Response::new(response))
    }
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
    async fn get_all_translations(
        &self,
        _request: Request<()>,
    ) -> Result<Response<TranslationsResponse>, Status> {
        let response = TranslationsResponse {
            translations: self.db.get_all_translations().await.map_err(|e| {
                warn!("failed to get all translations: {e}");
                Status::internal("failed to get all translations")
            })?,
        };
        Ok(Response::new(response))
    }
    async fn get_translation_by_id(
        &self,
        request: Request<GetTranslationByIdRequest>,
    ) -> Result<Response<Translation>, Status> {
        let id = request.into_inner().id;

        let response = self.db.get_translation_by_id(id).await.map_err(|e| {
            error!("failed to get translation by id [{id}]: {e}");
            Status::not_found("not found buddy")
        })?;
        Ok(Response::new(response))
    }
}
