-- Make customer_id optional to allow guest checkout
ALTER TABLE ordering.orders ALTER COLUMN customer_id DROP NOT NULL;

-- Since guest users won't have an ID in the identify.users table,
-- we must drop the foreign key constraint that requires customer_id to exist in identify.users.
-- NULL values naturally bypass FKs, but if someone tries to use a random non-registered UUID as guest, it would fail.
-- Keeping the FK is actually fine if guest = NULL.
-- Wait, the implementation plan said to drop the FK in case of anything, but actually NULL handles it fine.
-- Let's just drop the NOT NULL constraint.
-- If customer_id is NULL it will be a guest checkout.

-- We don't need to drop the FK constraint as long as we use NULL, 
-- but if we plan to use random session UUIDs in the future, we would drop it.
-- We are just using NULL for guests, so we just drop NOT NULL.
