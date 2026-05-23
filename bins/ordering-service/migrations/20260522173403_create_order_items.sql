CREATE TABLE order_items (
    id          SERIAL PRIMARY KEY,
    order_id    INTEGER NOT NULL REFERENCES orders(id),
    product_id  INTEGER NOT NULL,
    name        TEXT NOT NULL,
    price       DECIMAL(10, 2) NOT NULL CHECK (price > 0),
    quantity    INTEGER NOT NULL CHECK (quantity > 0)
);
