CREATE TABLE products (
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    price       DECIMAL(10, 2) NOT NULL,
    stock       INTEGER NOT NULL DEFAULT 0,
    image_url   TEXT NOT NULL,
    category_id INTEGER NOT NULL REFERENCES categories(id),
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
