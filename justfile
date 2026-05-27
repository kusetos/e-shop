catalog-db-reload:
    docker-compose stop postgres
    docker-compose up -d postgres
    until docker-compose exec postgres pg_isready -U eshop -d catalog; do sleep 1; done
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx database drop -y
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx database create
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx migrate run --source bins/catalog-service/migrations

ordering-db-reload:
    docker-compose stop postgres
    docker-compose up -d postgres
    until docker-compose exec postgres pg_isready -U eshop -d catalog; do sleep 1; done
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx database drop -y
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx database create
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx migrate run --source bins/ordering-service/migrations

catalog-migrate:
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog sqlx migrate run --source bins/catalog-service/migrations

catalog-seed:
    docker-compose exec -T postgres psql -U eshop -d catalog < bins/catalog-service/seeds/seed_catalog.sql

ordering-migrate:
    DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering sqlx migrate run --source bins/ordering-service/migrations

catalog-sqlx-prepare:
    cd bins/catalog-service && DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog cargo sqlx prepare

ordering-sqlx-prepare:
    cd bins/ordering-service && DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering cargo sqlx prepare

check:
    SQLX_OFFLINE=true cargo check --workspace

catalog-db-open:
    harlequin -a postgres "postgres://eshop:eshop@localhost:5432/catalog"

ordering-db-open:
    harlequin -a postgres "postgres://eshop:eshop@localhost:5432/ordering"

db-stop:
    docker-compose stop postgres

redis-run:
    docker-compose up -d redis

redis-stop:
    docker-compose stop redis
