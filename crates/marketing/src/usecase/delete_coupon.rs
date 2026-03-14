use std::sync::Arc;
use uuid::Uuid;
use shared::{database::UnitOfWork, error::AppError};
use crate::domain::repository::CouponRepository;

pub struct DeleteCouponUsecase {
    repo: Arc<dyn CouponRepository>,
    uow: Arc<dyn UnitOfWork>,
}

impl DeleteCouponUsecase {
    pub fn new(repo: Arc<dyn CouponRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self { repo, uow }
    }

    pub async fn execute(&self, coupon_id: Uuid) -> Result<(), AppError> {
        let repo = self.repo.clone();
        self.uow.run_atomic(Box::new(move |exec| {
            Box::pin(async move {
                repo.delete_coupon(exec, coupon_id).await
            })
        })).await
    }
}
