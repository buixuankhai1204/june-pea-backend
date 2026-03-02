use crate::domain::model::OrderItem;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct OrderItemRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub variant_id: Uuid,
    pub quantity: i32,
    pub unit_price: i64,
}

impl From<OrderItemRow> for OrderItem {
    fn from(row: OrderItemRow) -> Self {
        Self {
            id: row.id,
            order_id: row.order_id,
            variant_id: row.variant_id,
            quantity: row.quantity,
            unit_price: row.unit_price,
        }
    }
}
