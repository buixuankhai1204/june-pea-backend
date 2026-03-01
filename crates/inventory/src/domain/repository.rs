use async_trait::async_trait;
use uuid::Uuid;
use shared::{database::DbExecutor, error::AppError};

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn get_stock_for_update(&self, exec: &mut dyn DbExecutor, id: Uuid) -> Result<i32, AppError>;
    async fn update_stock(&self, exec: &mut dyn DbExecutor, id: Uuid, quantity: i32) -> Result<(), AppError>;
}