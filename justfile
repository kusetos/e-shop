catalog-db-reload:
    docker compose stop postgres
    docker compose up -d postgres
    until docker compose exec postgres pg_isready -U eshop -d catalog; do sleep 1; done
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx database drop -y
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx database create
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx migrate run --source bins/catalog-service/migrations

ordering-db-reload:
    docker compose stop postgres
    docker compose up -d postgres
    until docker compose exec postgres pg_isready -U eshop -d catalog; do sleep 1; done
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx database drop -y
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx database create
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx migrate run --source bins/ordering-service/migrations

catalog-migrate:
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx migrate run --source bins/catalog-service/migrations

catalog-seed:
    psql postgres://eshop:eshop@localhost:5432/catalog -f bins/catalog-service/seeds/seed_catalog.sql

ordering-migrate:
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx migrate run --source bins/ordering-service/migrations

catalog-db-open:
    harlequin -a postgres "postgres://eshop:eshop@localhost:5432/catalog"

ordering-db-open:
    harlequin -a postgres "postgres://eshop:eshop@localhost:5432/ordering"

db-stop:
    docker compose stop postgres

redis-run:
    docker compose up -d redis

redis-stop:
    docker compose stop redis
