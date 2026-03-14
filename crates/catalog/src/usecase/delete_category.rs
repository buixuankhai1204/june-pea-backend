use std::sync::Arc;
use uuid::Uuid;
use crate::domain::catalog_repository::CatalogRepository;
use shared::AppError;

pub struct DeleteCategoryUsecase {
    repo: Arc<dyn CatalogRepository>,
}

impl DeleteCategoryUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_category(id).await
    }
}
