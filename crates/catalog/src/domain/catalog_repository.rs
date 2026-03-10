use async_trait::async_trait;
use crate::domain::model::{Product, ProductWithVariants};
use shared::AppError;

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProductWithVariants>, AppError>;
    async fn list_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, AppError>;
    async fn count_all(&self) -> Result<i64, AppError>;
}