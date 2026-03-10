CREATE SCHEMA IF NOT EXISTS inventory;

CREATE TABLE inventory.stock (
    variant_id UUID PRIMARY KEY REFERENCES catalog.product_variants(id),
    quantity   INTEGER NOT NULL DEFAULT 0 CHECK (quantity >= 0)
);
