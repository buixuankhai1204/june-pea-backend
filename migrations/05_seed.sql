-- Disable foreign key checks for truncation (PostgreSQL handles this via CASCADE)
TRUNCATE TABLE ordering.order_items CASCADE;
TRUNCATE TABLE ordering.orders CASCADE;
TRUNCATE TABLE inventory.stock CASCADE;
TRUNCATE TABLE catalog.product_variants CASCADE;
TRUNCATE TABLE catalog.products CASCADE;
TRUNCATE TABLE catalog.categories CASCADE;
TRUNCATE TABLE marketing.coupons CASCADE;
TRUNCATE TABLE identify.users CASCADE;

-- Identity Users
INSERT INTO identify.users (id, email, password_hash, role) VALUES
('11111111-1111-1111-1111-111111111111', 'admin@junepea.com', 'hashed_pw_here', 'admin'),
('22222222-2222-2222-2222-222222222222', 'customer1@example.com', 'hashed_pw_here', 'customer'),
('33333333-3333-3333-3333-333333333333', 'customer2@example.com', 'hashed_pw_here', 'customer');

-- Catalog Categories
INSERT INTO catalog.categories (id, name, slug, parent_id) VALUES
('44444444-4444-4444-4444-444444444444', 'Clothing', 'clothing', NULL),
('55555555-5555-5555-5555-555555555555', 'Electronics', 'electronics', NULL);

INSERT INTO catalog.categories (id, name, slug, parent_id) VALUES
('66666666-6666-6666-6666-666666666666', 'T-Shirts', 't-shirts', '44444444-4444-4444-4444-444444444444'),
('77777777-7777-7777-7777-777777777777', 'Laptops', 'laptops', '55555555-5555-5555-5555-555555555555');

-- Catalog Products
INSERT INTO catalog.products (id, category_id, name, slug, description) VALUES
('88888888-8888-8888-8888-888888888888', '66666666-6666-6666-6666-666666666666', 'Basic T-Shirt', 'basic-t-shirt', 'A comfortable basic t-shirt'),
('99999999-9999-9999-9999-999999999999', '77777777-7777-7777-7777-777777777777', 'Pro Laptop', 'pro-laptop', 'High performance laptop');

-- Catalog Product Variants
INSERT INTO catalog.product_variants (id, product_id, sku, name, attributes, base_price, sale_price) VALUES
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '88888888-8888-8888-8888-888888888888', 'TSHIRT-BLK-L', 'Basic T-Shirt - Black - L', '{"color": "black", "size": "L"}', 2000, 1500),
('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '88888888-8888-8888-8888-888888888888', 'TSHIRT-WHT-M', 'Basic T-Shirt - White - M', '{"color": "white", "size": "M"}', 2000, NULL),
('cccccccc-cccc-cccc-cccc-cccccccccccc', '99999999-9999-9999-9999-999999999999', 'LAPTOP-PRO-16', 'Pro Laptop 16 inch', '{"ram": "16GB", "storage": "512GB"}', 150000, 140000);

-- Inventory Stock
INSERT INTO inventory.stock (variant_id, quantity) VALUES
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 100),
('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 50),
('cccccccc-cccc-cccc-cccc-cccccccccccc', 10);

-- Marketing Coupons
INSERT INTO marketing.coupons (id, code, discount_amount, max_uses, current_uses, is_active) VALUES
('dddddddd-dddd-dddd-dddd-dddddddddddd', 'WELCOME10', 1000, 100, 0, TRUE),
('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'SUMMER20', 2000, 50, 50, FALSE);

-- Ordering
INSERT INTO ordering.orders (id, customer_id, status, total) VALUES
('ffffffff-ffff-ffff-ffff-ffffffffffff', '22222222-2222-2222-2222-222222222222', 'completed', 3000),
('00000000-0000-0000-0000-000000000000', '33333333-3333-3333-3333-333333333333', 'pending', 140000);

-- Ordering Items
INSERT INTO ordering.order_items (id, order_id, variant_id, quantity, unit_price) VALUES
('11a111a1-1a1a-1111-1a1a-1a1a1a1a1a1a', 'ffffffff-ffff-ffff-ffff-ffffffffffff', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 2, 1500),
('22b222b2-2b2b-2222-2b2b-2b2b2b2b2b2b', '00000000-0000-0000-0000-000000000000', 'cccccccc-cccc-cccc-cccc-cccccccccccc', 1, 140000);
