use crate::domain::cache::CatalogCache;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::ProductWithVariants;
use crate::usecase::product_details::GetProductUsecase;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared::AppError;
use std::sync::Arc;

#[derive( Clone)]
pub struct CatalogUsecase {
    get_product_usecase: Arc<GetProductUsecase>
}

impl CatalogUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>, cache: Arc<dyn CatalogCache>) -> Self {
        Self {
            get_product_usecase: Arc::new(GetProductUsecase::new(repo, cache))
        }
    }

    pub fn get_product_usecase(&self) -> Arc<GetProductUsecase> {
        self.get_product_usecase.clone()
    }
}
pub fn init() -> Router<CatalogUsecase>
{
    Router::new().route("/products/:slug", get(get_product_handler))
}

async fn get_product_handler(
    State(state): State<CatalogUsecase>,
    Path(slug): Path<String>,
) -> Result<Json<ProductWithVariants>, AppError>

{
    let usecase = state.get_product_usecase();
    let product = usecase.execute(&slug).await?;

    Ok(Json(product))
}