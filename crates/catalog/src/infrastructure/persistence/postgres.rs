use std::sync::Arc;
use async_trait::async_trait;
use shared::AppError;
use crate::domain::catalog_repository::CatalogRepository;
use crate::domain::model::{Product, ProductWithVariants};

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
        let row = sqlx::query!(
            r#"
            SELECT p.*, 
                   json_agg(v.*) as "variants!"
            FROM catalog.products p
            LEFT JOIN catalog.product_variants v ON p.id = v.product_id
            WHERE p.slug = $1
            GROUP BY p.id
            "#,
            slug
        )
            .fetch_optional(&*self.pool)
            .await?;
        Ok(None)

        // Logic to map database JSON back to our Domain Structs
        // ... mapping code ...
    }

    async fn list_all(&self, _limit: i64, _offset: i64) -> Result<Vec<Product>, AppError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_create_and_fetch_product(pool: sqlx::PgPool) {
        let repo = PostgresCatalogRepository::new(Arc::new(pool));
        let product_id = uuid::Uuid::new_v4();

        // 1. Setup seed data manually or via repo
        sqlx::query!("INSERT INTO catalog.products (id, name, slug, category_id) VALUES ($1, $2, $3, $4)",
            product_id, "Yame T-Shirt", "yame-t-shirt", uuid::Uuid::new_v4())
            .execute(&repo.pool).await.unwrap();

        // 2. Execution
        let result = repo.get_by_slug("yame-t-shirt").await.unwrap();

        // 3. Validation
        assert!(result.is_some());
        assert_eq!(result.unwrap().product.name, "Yame T-Shirt");
    }
}