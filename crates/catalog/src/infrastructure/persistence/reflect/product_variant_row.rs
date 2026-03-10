use crate::domain::model::ProductVariant;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProductVariantRow {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub base_price: Decimal,
    pub sale_price: Option<Decimal>,
    pub attributes: serde_json::Value,
}

impl From<ProductVariantRow> for ProductVariant {
    fn from(row: ProductVariantRow) -> Self {
        Self {
            id: row.id,
            product_id: row.product_id,
            sku: row.sku,
            name: row.name,
            base_price: row.base_price,
            sale_price: row.sale_price,
            attributes: row.attributes,
        }
    }
}
