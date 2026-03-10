use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared::AppError;
use std::sync::Arc;
use uuid::Uuid;
use shared::database::UnitOfWork;
use crate::domain::repository::InventoryRepository;
use crate::usecase::decrease_stock::DecreaseStockUsecase;

#[derive( Clone)]
pub struct InventoryUsecase {
    descrease_stock_usecase: Arc<DecreaseStockUsecase>
}

impl InventoryUsecase {
    pub fn new(repo: Arc<dyn InventoryRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self {
            descrease_stock_usecase: Arc::new(DecreaseStockUsecase::new(repo, uow))
        }
    }

    pub fn descrease_stock_usecase(&self) -> Arc<DecreaseStockUsecase> {
        self.descrease_stock_usecase.clone()
    }
}
pub fn init() -> Router<InventoryUsecase>
{
    Router::new().route("/inventory/decrease-stock", get(descrease_stock_handler))
}

async fn descrease_stock_handler(
    State(state): State<InventoryUsecase>,
    Path(_slug): Path<String>,
) -> Result<Json<bool>, AppError>

{
    let usecase = state.descrease_stock_usecase();
    usecase.execute(Uuid::parse_str("asd").unwrap(),0).await?;

    Ok(Json(true))
}