use std::sync::Arc;
use async_trait::async_trait;
use shared::AppError;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::{Product, ProductVariant, ProductWithVariants};

pub struct PostgresCatalogRepository {
    pool: Arc<sqlx::PgPool>,
}

impl PostgresCatalogRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for PostgresCatalogRepository {
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProductWithVariants>, AppError> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT id, name, slug, description, category_id FROM catalog.products WHERE slug = $1",
        )
            .bind(slug)
            .fetch_optional(&*self.pool)
            .await?;

        let product = match product {
            Some(p) => p,
            None => return Ok(None),
        };

        let variants = sqlx::query_as::<_, ProductVariant>(
            "SELECT id, product_id, sku, name, attributes, base_price, sale_price FROM catalog.product_variants WHERE product_id = $1",
        )
            .bind(product.id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(Some(ProductWithVariants { product, variants }))
    }

    async fn list_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            "SELECT id, name, slug, description, category_id FROM catalog.products ORDER BY name LIMIT $1 OFFSET $2",
        )
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?;

        Ok(products)
    }

    async fn count_all(&self) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM catalog.products")
            .fetch_one(&*self.pool)
            .await?;
        Ok(row.0)
    }
}