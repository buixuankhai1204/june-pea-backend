use std::sync::Arc;
use uuid::Uuid;
use crate::domain::catalog_repository::CatalogRepository;
use shared::AppError;

pub struct ListCategoriesUsecase {
    repo: Arc<dyn CatalogRepository>,
}

impl ListCategoriesUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<(Uuid, String, String, Option<Uuid>)>, AppError> {
        self.repo.list_categories().await
    }
}
