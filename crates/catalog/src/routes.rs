use std::format;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use tokio::spawn;
use shared::AppError;
use crate::domain::cache::CatalogCache;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::ProductWithVariants;
use crate::usecase::product_details::GetProductUsecase;

pub trait CatalogState: Send + Sync {
    fn catalog_repository(&self) -> Arc<dyn CatalogRepository>;
}
pub fn init<S>() -> Router<S>
where S: CatalogState + Clone + Send + Sync + 'static
{
    Router::new().route("/products/:slug", get(get_product_handler::<S>))
}

async fn get_product_handler<S>(
    State(state): State<GetProductUsecase>,
    Path(slug): Path<String>,
) -> Result<Json<ProductWithVariants>, AppError>
where
    S: CatalogState,
{
    let usecase = state.get_product_usecase();
    let product = usecase.execute(&slug).await?;

    Ok(Json(product))
}