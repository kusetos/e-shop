CREATE TYPE order_status AS ENUM (
    'Pending',
    'Confirmed',
    'Cancelled'
);

CREATE TABLE orders (
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    status      order_status NOT NULL DEFAULT 'Pending',
    total       DECIMAL(10, 2) NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
