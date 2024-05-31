use crate::base::translations_server::Translations;
use crate::db::DB;
use crate::language::{GetTranslationByIdRequest, Translation, TranslationsResponse};
use log::{error, warn};
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct TranslationsServer {
    pub db: DB,
}

impl TranslationsServer {
    pub fn new(db: DB) -> Self {
        TranslationsServer { db }
    }
}

#[tonic::async_trait]
impl Translations for TranslationsServer {
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
