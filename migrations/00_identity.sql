CREATE SCHEMA IF NOT EXISTS public.identify;

CREATE TABLE identify.users (
                                id UUID PRIMARY KEY,
                                email TEXT NOT NULL UNIQUE,
                                password_hash TEXT NOT NULL,
                                role TEXT NOT NULL DEFAULT 'customer',
                                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);