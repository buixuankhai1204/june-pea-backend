use crate::domain::{model::Coupon, repository::CouponRepository};
use shared::{database::UnitOfWork, error::AppError};
use std::sync::Arc;

pub struct CreateCouponUsecase {
    repo: Arc<dyn CouponRepository>,
    uow: Arc<dyn UnitOfWork>,
}

impl CreateCouponUsecase {
    pub fn new(repo: Arc<dyn CouponRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self { repo, uow }
    }

    pub async fn execute(
        &self,
        code: String,
        discount_amount: i64,
        max_uses: i32,
    ) -> Result<Coupon, AppError> {
        let coupon = Coupon::new(code, discount_amount, max_uses)?;

        let repo = self.repo.clone();
        let coupon_clone = coupon.clone();

        self.uow
            .run_atomic(Box::new(move |exec| {
                Box::pin(async move { repo.create_coupon(exec, &coupon_clone).await })
            }))
            .await?;

        Ok(coupon)
    }
}
