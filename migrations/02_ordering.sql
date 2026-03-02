CREATE SCHEMA IF NOT EXISTS ordering;

CREATE TABLE ordering.orders (
    id          UUID        PRIMARY KEY,
    customer_id UUID        NOT NULL REFERENCES identify.users(id),
    status      TEXT        NOT NULL DEFAULT 'pending'
                                CHECK (status IN ('pending', 'cancelled', 'completed')),
    total       BIGINT      NOT NULL CHECK (total >= 0),  -- stored in cents
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ordering.order_items (
    id          UUID    PRIMARY KEY,
    order_id    UUID    NOT NULL REFERENCES ordering.orders(id) ON DELETE CASCADE,
    variant_id  UUID    NOT NULL REFERENCES catalog.product_variants(id),
    quantity    INTEGER NOT NULL CHECK (quantity > 0),
    unit_price  BIGINT  NOT NULL CHECK (unit_price >= 0)  -- stored in cents
);

CREATE INDEX idx_orders_customer_id ON ordering.orders (customer_id);
CREATE INDEX idx_order_items_order_id ON ordering.order_items (order_id);
