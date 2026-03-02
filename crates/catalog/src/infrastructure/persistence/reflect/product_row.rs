use crate::domain::model::Product;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Uuid,
}

impl From<ProductRow> for Product {
    fn from(row: ProductRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            category_id: row.category_id,
        }
    }
}
