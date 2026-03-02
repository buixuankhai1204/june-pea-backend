use crate::domain::{
    model::{NewOrderItem, Order},
    repository::OrderRepository,
};
use shared::{database::UnitOfWork, error::AppError};
use std::sync::Arc;
use uuid::Uuid;

pub struct PlaceOrderUsecase {
    repo: Arc<dyn OrderRepository>,
    uow: Arc<dyn UnitOfWork>,
}

impl PlaceOrderUsecase {
    pub fn new(repo: Arc<dyn OrderRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self { repo, uow }
    }

    pub async fn execute(
        &self,
        customer_id: Uuid,
        new_items: Vec<NewOrderItem>,
    ) -> Result<Uuid, AppError> {
        // Domain factory validates all business rules internally
        let (order, items) = Order::place(customer_id, new_items)?;
        let order_id = order.id;

        let repo = self.repo.clone();

        self.uow
            .run_atomic(Box::new(move |exec| {
                Box::pin(async move { repo.create_order(exec, &order, &items).await })
            }))
            .await?;

        Ok(order_id)
    }
}
