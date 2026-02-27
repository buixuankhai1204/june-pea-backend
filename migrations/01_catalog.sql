CREATE SCHEMA IF NOT EXISTS catalog;

CREATE TABLE catalog.categories (
                                    id UUID PRIMARY KEY,
                                    name TEXT NOT NULL,
                                    slug TEXT NOT NULL UNIQUE,
                                    parent_id UUID REFERENCES catalog.categories(id)
);

CREATE TABLE catalog.products (
                                  id UUID PRIMARY KEY,
                                  category_id UUID NOT NULL REFERENCES catalog.categories(id),
                                  name TEXT NOT NULL,
                                  slug TEXT NOT NULL UNIQUE,
                                  description TEXT,
                                  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE catalog.product_variants (
                                          id UUID PRIMARY KEY,
                                          product_id UUID NOT NULL REFERENCES catalog.products(id) ON DELETE CASCADE,
                                          sku TEXT NOT NULL UNIQUE,
                                          name TEXT NOT NULL, -- Ví dụ: "Áo thun Yame - Size L - Đen"
                                          attributes JSONB NOT NULL, -- Lưu {"color": "black", "size": "L"}
                                          base_price DECIMAL(12,2) NOT NULL,
                                          sale_price DECIMAL(12,2)
);