use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use shared::AppError;
use std::sync::Arc;
use uuid::Uuid;
use shared::database::UnitOfWork;
use crate::domain::repository::InventoryRepository;
use crate::usecase::decrease_stock::DecreaseStockUsecase;

#[derive( Clone)]
pub struct InventoryUsecase {
    decrease_stock_usecase: Arc<DecreaseStockUsecase>
}

impl InventoryUsecase {
    pub fn new(repo: Arc<dyn InventoryRepository>, uow: Arc<dyn UnitOfWork>) -> Self {
        Self {
            decrease_stock_usecase: Arc::new(DecreaseStockUsecase::new(repo, uow))
        }
    }

    pub fn decrease_stock_usecase(&self) -> Arc<DecreaseStockUsecase> {
        self.decrease_stock_usecase.clone()
    }
}

#[derive(Debug, Deserialize)]
struct DecreaseStockRequest {
    variant_id: Uuid,
    amount: i32,
}

pub fn init() -> Router<InventoryUsecase>
{
    Router::new().route("/decrease-stock", post(decrease_stock_handler))
}

async fn decrease_stock_handler(
    State(state): State<InventoryUsecase>,
    Json(body): Json<DecreaseStockRequest>,
) -> Result<Json<bool>, AppError>
{
    if body.amount <= 0 {
        return Err(AppError::Validation("Amount must be positive".into()));
    }
    let usecase = state.decrease_stock_usecase();
    usecase.execute(body.variant_id, body.amount).await?;
    Ok(Json(true))
}