use crate::domain::cache::CatalogCache;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::{PaginatedProducts, ProductWithVariants};
use crate::usecase::{
    list_products::ListProductsUsecase,
    product_details::GetProductUsecase,
    create_category::CreateCategoryUsecase,
    create_product::CreateProductUsecase,
    update_product::UpdateProductUsecase,
    list_categories::ListCategoriesUsecase,
    delete_product::DeleteProductUsecase,
    delete_category::DeleteCategoryUsecase,
};
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;
use serde::Deserialize;
use shared::AppError;
use std::sync::Arc;

#[derive( Clone)]
pub struct CatalogUsecase {
    get_product_usecase: Arc<GetProductUsecase>,
    list_products_usecase: Arc<ListProductsUsecase>,
    create_category_usecase: Arc<CreateCategoryUsecase>,
    create_product_usecase: Arc<CreateProductUsecase>,
    update_product_usecase: Arc<UpdateProductUsecase>,
    list_categories_usecase: Arc<ListCategoriesUsecase>,
    delete_product_usecase: Arc<DeleteProductUsecase>,
    delete_category_usecase: Arc<DeleteCategoryUsecase>,
}

impl CatalogUsecase {
    pub fn new(repo: Arc<dyn CatalogRepository>, cache: Arc<dyn CatalogCache>) -> Self {
        Self {
            list_products_usecase: Arc::new(ListProductsUsecase::new(repo.clone())),
            get_product_usecase: Arc::new(GetProductUsecase::new(repo.clone(), cache)),
            create_category_usecase: Arc::new(CreateCategoryUsecase::new(repo.clone())),
            create_product_usecase: Arc::new(CreateProductUsecase::new(repo.clone())),
            update_product_usecase: Arc::new(UpdateProductUsecase::new(repo.clone())),
            list_categories_usecase: Arc::new(ListCategoriesUsecase::new(repo.clone())),
            delete_product_usecase: Arc::new(DeleteProductUsecase::new(repo.clone())),
            delete_category_usecase: Arc::new(DeleteCategoryUsecase::new(repo)),
        }
    }

    pub fn get_product_usecase(&self) -> Arc<GetProductUsecase> {
        self.get_product_usecase.clone()
    }

    pub fn list_products_usecase(&self) -> Arc<ListProductsUsecase> {
        self.list_products_usecase.clone()
    }

    pub fn create_category_usecase(&self) -> Arc<CreateCategoryUsecase> {
        self.create_category_usecase.clone()
    }

    pub fn create_product_usecase(&self) -> Arc<CreateProductUsecase> {
        self.create_product_usecase.clone()
    }

    pub fn update_product_usecase(&self) -> Arc<UpdateProductUsecase> {
        self.update_product_usecase.clone()
    }

    pub fn list_categories_usecase(&self) -> Arc<ListCategoriesUsecase> {
        self.list_categories_usecase.clone()
    }

    pub fn delete_product_usecase(&self) -> Arc<DeleteProductUsecase> {
        self.delete_product_usecase.clone()
    }

    pub fn delete_category_usecase(&self) -> Arc<DeleteCategoryUsecase> {
        self.delete_category_usecase.clone()
    }
}

#[derive(Debug, Deserialize)]
struct ListProductsQuery {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CreateCategoryRequest {
    name: String,
    slug: Option<String>,
    parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
struct CreateProductRequest {
    category_id: Uuid,
    name: String,
    slug: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateProductRequest {
    category_id: Uuid,
    name: String,
    slug: Option<String>,
    description: Option<String>,
}

pub fn init() -> Router<CatalogUsecase>
{
    Router::new()
        .route("/products", get(list_products_handler))
        .route("/products", axum::routing::post(create_product_handler))
        .route("/products/{id}", axum::routing::patch(update_product_handler))
        .route("/products/{id}", axum::routing::delete(delete_product_handler))
        .route("/products/slug/{slug}", get(get_product_handler))
        .route("/categories", get(list_categories_handler))
        .route("/categories", axum::routing::post(create_category_handler))
        .route("/categories/{id}", axum::routing::delete(delete_category_handler))
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

async fn list_categories_handler(
    State(state): State<CatalogUsecase>,
) -> Result<Json<Vec<(Uuid, String, String, Option<Uuid>)>>, AppError> {
    let usecase = state.list_categories_usecase();
    let result = usecase.execute().await?;
    Ok(Json(result))
}

async fn create_category_handler(
    State(state): State<CatalogUsecase>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.create_category_usecase();
    usecase.execute(body.name, body.slug, body.parent_id).await?;
    Ok(Json(true))
}

async fn create_product_handler(
    State(state): State<CatalogUsecase>,
    Json(body): Json<CreateProductRequest>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.create_product_usecase();
    usecase.execute(body.name, body.slug, body.category_id, body.description).await?;
    Ok(Json(true))
}

async fn update_product_handler(
    State(state): State<CatalogUsecase>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.update_product_usecase();
    usecase.execute(id, body.name, body.slug, body.category_id, body.description).await?;
    Ok(Json(true))
}

async fn delete_product_handler(
    State(state): State<CatalogUsecase>,
    Path(id): Path<Uuid>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.delete_product_usecase();
    usecase.execute(id).await?;
    Ok(Json(true))
}

async fn delete_category_handler(
    State(state): State<CatalogUsecase>,
    Path(id): Path<Uuid>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.delete_category_usecase();
    usecase.execute(id).await?;
    Ok(Json(true))
}