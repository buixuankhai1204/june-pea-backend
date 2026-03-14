use std::sync::Arc;
use uuid::Uuid;
use crate::domain::catalog_repository::CatalogRepository;
use shared::AppError;

pub struct DeleteProductUsecase {
    repo: Arc<dyn CatalogRepository>,
}

impl DeleteProductUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_product(id).await
    }
}
