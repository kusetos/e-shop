hi:
    echo asfdklj

db-reload:
    docker-compose stop postgres
    docker-compose up postgres
    sqlx database drop
    sqlx database create
    sqlx migrate run

db-open:
    harlequin -a postgres "postgres://eshop:eshop@localhost:5432/catalog"

db-stop:
    docker-compose stop postgres

redis-run:
    docker-compose up -d redis

redis-stop:
    docker-compose stop redis
