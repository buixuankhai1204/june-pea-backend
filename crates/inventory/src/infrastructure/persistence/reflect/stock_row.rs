use crate::domain::model::Stock;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct StockRow {
    pub variant_id: Uuid,
    pub quantity: i32,
}

impl From<StockRow> for Stock {
    fn from(row: StockRow) -> Self {
        Self {
            variant_id: row.variant_id,
            quantity: row.quantity,
        }
    }
}
