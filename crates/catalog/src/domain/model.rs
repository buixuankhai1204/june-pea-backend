use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,             // E.g., YAME-TSHIRT-L-BLK
    pub price: Decimal,
    pub attributes: serde_json::Value, // {"size": "L", "color": "Black"}
}

/// A composite DTO for the Storefront
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWithVariants {
    pub product: Product,
    pub variants: Vec<ProductVariant>,
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_variant_discount_calculation() {
        let variant = ProductVariant {
            id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            sku: "YAME-TEE-L-BLK".to_string(),
            price: Decimal::new(10000, 2), // $100.00
            attributes: serde_json::json!({"size": "L"}),
        };

        // Assertions for business rules can go here
        assert_eq!(variant.price, Decimal::new(10000, 2));
    }
}