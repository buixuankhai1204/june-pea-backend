use crate::domain::cache::CatalogCache;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::{PaginatedProducts, ProductWithVariants};
use crate::usecase::list_products::ListProductsUsecase;
use crate::usecase::product_details::GetProductUsecase;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use shared::AppError;
use std::sync::Arc;

#[derive( Clone)]
pub struct CatalogUsecase {
    get_product_usecase: Arc<GetProductUsecase>,
    list_products_usecase: Arc<ListProductsUsecase>,
}

impl CatalogUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>, cache: Arc<dyn CatalogCache>) -> Self {
        Self {
            list_products_usecase: Arc::new(ListProductsUsecase::new(repo.clone())),
            get_product_usecase: Arc::new(GetProductUsecase::new(repo, cache)),
        }
    }

    pub fn get_product_usecase(&self) -> Arc<GetProductUsecase> {
        self.get_product_usecase.clone()
    }

    pub fn list_products_usecase(&self) -> Arc<ListProductsUsecase> {
        self.list_products_usecase.clone()
    }
}

#[derive(Debug, Deserialize)]
struct ListProductsQuery {
    page: Option<i64>,
    page_size: Option<i64>,
}

pub fn init() -> Router<CatalogUsecase>
{
    Router::new()
        .route("/products", get(list_products_handler))
        .route("/products/{slug}", get(get_product_handler))
}

async fn list_products_handler(
    State(state): State<CatalogUsecase>,
    Query(params): Query<ListProductsQuery>,
) -> Result<Json<PaginatedProducts>, AppError> {
    let usecase = state.list_products_usecase();
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let result = usecase.execute(page, page_size).await?;
    Ok(Json(result))
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