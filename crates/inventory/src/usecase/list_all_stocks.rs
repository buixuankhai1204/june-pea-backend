use crate::domain::model::Stock;
use crate::domain::repository::InventoryRepository;
use shared::{database::UnitOfWork, error::AppError};
use std::sync::Arc;

pub struct ListAllStocksUsecase {
    repo: Arc<dyn InventoryRepository>,
    uow: Arc<dyn UnitOfWork>,
}

impl ListAllStocksUsecase {
    pub fn new(repo: Arc<dyn InventoryRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self { repo, uow }
    }

    pub async fn execute(&self) -> Result<Vec<Stock>, AppError> {
        let stocks = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let stocks_clone = stocks.clone();
        let repo = self.repo.clone();

        UnitOfWork::run_read_only(
            &*self.uow,
            Box::new(move |exec| {
                Box::pin(async move {
                    let mut guard = stocks_clone.lock().await;
                    *guard = repo.list_all_stocks(exec).await?;
                    Ok(())
                })
            }),
        )
        .await?;

        let guard = stocks.lock().await;
        Ok(guard.clone())
    }
}
