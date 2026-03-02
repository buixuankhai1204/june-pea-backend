use crate::domain::{model::Order, repository::OrderRepository};
use shared::error::AppError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetOrderUsecase {
    pool: PgPool,
    repo: Arc<dyn OrderRepository>,
}

impl GetOrderUsecase {
    pub fn new(pool: PgPool, repo: Arc<dyn OrderRepository>) -> Self {
        Self { pool, repo }
    }

    pub async fn execute(&self, order_id: Uuid) -> Result<Order, AppError> {
        // Read-only: start a lightweight transaction to satisfy DbExecutor API
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|_| AppError::InternalServerError)?;

        use shared::infrastructure::postgres::SqlxExecutor;
        let mut executor = SqlxExecutor { tx };

        let result = self.repo.get_order_by_id(&mut executor, order_id).await;

        // Always rollback — this was read-only
        let _ = executor.tx.rollback().await;

        result
    }
}
