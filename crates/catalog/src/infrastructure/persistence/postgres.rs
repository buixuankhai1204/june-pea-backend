use std::sync::Arc;
use async_trait::async_trait;
use shared::AppError;
use uuid::Uuid;
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

    async fn create_category(&self, id: Uuid, name: &str, slug: &str, parent_id: Option<Uuid>) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO catalog.categories (id, name, slug, parent_id) VALUES ($1, $2, $3, $4)"
        )
            .bind(id)
            .bind(name)
            .bind(slug)
            .bind(parent_id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<(Uuid, String, String, Option<Uuid>)>, AppError> {
        let rows = sqlx::query_as::<_, (Uuid, String, String, Option<Uuid>)>(
            "SELECT id, name, slug, parent_id FROM catalog.categories ORDER BY name"
        )
            .fetch_all(&*self.pool)
            .await?;
        Ok(rows)
    }

    async fn create_product(&self, id: Uuid, category_id: Uuid, name: &str, slug: &str, description: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO catalog.products (id, category_id, name, slug, description) VALUES ($1, $2, $3, $4, $5)"
        )
            .bind(id)
            .bind(category_id)
            .bind(name)
            .bind(slug)
            .bind(description)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn update_product(&self, id: Uuid, category_id: Uuid, name: &str, slug: &str, description: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE catalog.products SET category_id = $1, name = $2, slug = $3, description = $4 WHERE id = $5"
        )
            .bind(category_id)
            .bind(name)
            .bind(slug)
            .bind(description)
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn delete_product(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM catalog.products WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn delete_category(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM catalog.categories WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}