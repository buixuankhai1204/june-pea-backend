use async_trait::async_trait;
use crate::domain::model::{Product, ProductWithVariants};
use uuid::Uuid;
use shared::AppError;

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProductWithVariants>, AppError>;
    async fn list_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, AppError>;
    async fn count_all(&self) -> Result<i64, AppError>;
    async fn create_category(&self, id: Uuid, name: &str, slug: &str, parent_id: Option<Uuid>) -> Result<(), AppError>;
    async fn list_categories(&self) -> Result<Vec<(Uuid, String, String, Option<Uuid>)>, AppError>;
    async fn create_product(&self, id: Uuid, category_id: Uuid, name: &str, slug: &str, description: Option<&str>) -> Result<(), AppError>;
    async fn update_product(&self, id: Uuid, category_id: Uuid, name: &str, slug: &str, description: Option<&str>) -> Result<(), AppError>;
    async fn delete_product(&self, id: Uuid) -> Result<(), AppError>;
    async fn delete_category(&self, id: Uuid) -> Result<(), AppError>;
}