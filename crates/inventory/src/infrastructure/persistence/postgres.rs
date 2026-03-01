use async_trait::async_trait;
use uuid::Uuid;
use shared::{database::DbExecutor, error::AppError, infrastructure::postgres::SqlxExecutor};
use crate::domain::repository::InventoryRepository;

pub struct PostgresInventoryRepository;

#[async_trait]
impl InventoryRepository for PostgresInventoryRepository {
    async fn get_stock_for_update(&self, exec: &mut dyn DbExecutor, id: Uuid) -> Result<i32, AppError> {
        // Safe downcast to our Shared SQLx Wrapper
        let executor = SqlxExecutor::from_executor(exec);

        let row = sqlx::query!(
            "SELECT quantity FROM inventory.stocks WHERE variant_id = $1 FOR UPDATE",
            id
        )
            .fetch_one(&mut *executor.tx)
            .await
            .map_err(|e| AppError::InternalServerError)?;

        Ok(row.quantity)
    }

    async fn update_stock(&self, exec: &mut dyn DbExecutor, id: Uuid, quantity: i32) -> Result<(), AppError> {
        let executor = SqlxExecutor::from_executor(exec);

        sqlx::query!(
            "UPDATE inventory.stocks SET quantity = $1 WHERE variant_id = $2",
            quantity, id
        )
            .execute(&mut *executor.tx)
            .await
            .map_err(|e| AppError::InternalServerError)?;

        Ok(())
    }
}