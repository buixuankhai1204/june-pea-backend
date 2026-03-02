use crate::domain::{model::OrderStatus, repository::OrderRepository};
use shared::{database::UnitOfWork, error::AppError};
use std::sync::Arc;
use uuid::Uuid;

pub struct CancelOrderUsecase {
    repo: Arc<dyn OrderRepository>,
    uow: Arc<dyn UnitOfWork>,
}

impl CancelOrderUsecase {
    pub fn new(repo: Arc<dyn OrderRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self { repo, uow }
    }

    pub async fn execute(&self, order_id: Uuid) -> Result<(), AppError> {
        let repo = self.repo.clone();

        self.uow
            .run_atomic(Box::new(move |exec| {
                Box::pin(async move {
                    let mut order = repo.get_order_by_id(exec, order_id).await?;

                    // Domain mutator validates the cancellation rule internally
                    order.cancel()?;

                    repo.update_order_status(exec, order_id, OrderStatus::Cancelled)
                        .await
                })
            }))
            .await
    }
}
