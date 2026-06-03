# E-Shop — Microservices E-Commerce Platform

A production-style e-commerce backend built entirely in **Rust**, following a microservices architecture. Features an API Gateway, event-driven communication via **Kafka**, JWT authentication, and PostgreSQL + Redis persistence.

---

## Architecture Overview

```
                          ┌──────────────────────────────────────────────────┐
                          │                  CLIENT (HTTP)                   │
                          └─────────────────────┬────────────────────────────┘
                                                 │ :8080
                          ┌──────────────────────▼────────────────────────────┐
                          │               API GATEWAY (Axum)                  │
                          │   • JWT validation                                 │
                          │   • Request routing                                │
                          │   • x-user-id header injection                    │
                          └──────┬──────────┬──────────┬──────────┬───────────┘
                                 │          │          │          │
                    ┌────────────▼─┐  ┌─────▼──────┐  │  ┌───────▼──────────┐
                    │   Catalog    │  │   Basket   │  │  │     Identity     │
                    │  Service     │  │  Service   │  │  │     Service      │
                    │  :3000       │  │  :3001     │  │  │     :3003        │
                    │  PostgreSQL  │  │  Redis     │  │  │  PostgreSQL      │
                    └──────┬───────┘  └──────┬─────┘  │  └──────────────────┘
                           │                 │         │
                           │   ┌─────────────▼──────┐  │
                           │   │  Ordering Service  ◄──┘
                           │   │  :3002             │
                           │   │  PostgreSQL        │
                           │   └─────────┬──────────┘
                           │             │  Publishes: order-created
                           │             ▼
                           │   ┌─────────────────────┐
                           └───►       KAFKA          │
                               │  Topic: order-created│
                               └─────────┬────────────┘
                                         │ Consumes
                               ┌─────────┴──────────┐
                               │                    │
                        Catalog Service       Basket Service
                        (decrement stock)   (clear basket)
```

### Services

| Service | Port | Responsibility |
|---|---|---|
| **API Gateway** | `8080` | Single entry point — routes requests, validates JWT, injects user context |
| **Catalog Service** | `3000` | Products & categories CRUD; consumes `order-created` to decrement stock |
| **Basket Service** | `3001` | Shopping cart backed by Redis; consumes `order-created` to clear cart on checkout |
| **Ordering Service** | `3002` | Order creation & management; publishes `order-created` Kafka events |
| **Identity Service** | `3003` | User registration, login, JWT issuance; Argon2 password hashing |

### Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (2021/2024 edition) |
| Web Framework | Axum 0.7 + Tokio |
| Databases | PostgreSQL 16, Redis 7 |
| Message Broker | Apache Kafka |
| Kafka Client | rdkafka |
| Auth | JWT (jsonwebtoken 9), Argon2 |
| DB Driver | sqlx (async, compile-time checked) |
| Containerization | Docker + Docker Compose |

---

## API Reference

All requests go through the **Gateway at `http://localhost:8080`**. Protected routes require `Authorization: Bearer <token>` header.

### Authentication

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `POST` | `/api/auth/register` | Public | Register a new user |
| `POST` | `/api/auth/login` | Public | Login and receive JWT |
| `GET` | `/api/auth/me` | JWT | Get current user info |

### Catalog

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `GET` | `/api/products` | Public | List all products |
| `GET` | `/api/products/:id` | Public | Get product by ID |
| `POST` | `/api/products` | JWT | Create product |
| `PUT` | `/api/products/:id` | JWT | Update product |
| `DELETE` | `/api/products/:id` | JWT | Delete product |
| `GET` | `/api/categories` | Public | List all categories |
| `POST` | `/api/categories` | JWT | Create category |

### Basket

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `GET` | `/api/basket` | JWT | Get current user's basket |
| `POST` | `/api/basket/items` | JWT | Add item to basket |
| `PUT` | `/api/basket/items/:id` | JWT | Update item quantity |
| `DELETE` | `/api/basket/items/:id` | JWT | Remove item from basket |
| `DELETE` | `/api/basket` | JWT | Clear entire basket |

### Orders

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `GET` | `/api/orders` | JWT | List user's orders |
| `GET` | `/api/orders/:id` | JWT | Get order details |
| `POST` | `/api/orders` | JWT | Create order from basket |

---

## Event Flow

```
POST /api/orders
       │
       ▼
Ordering Service
  ├── Validates products via HTTP → Catalog Service
  ├── Persists order to PostgreSQL
  └── Publishes → Kafka topic: order-created
                        │
           ┌────────────┴──────────────┐
           ▼                           ▼
   Catalog Service              Basket Service
  (decrement stock)           (clear user basket)
```

---

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose v2
- [Rust toolchain](https://rustup.rs/) (for local development only)
- [just](https://github.com/casey/just) (optional, for dev tasks)

---

### Running with Docker Compose

This is the recommended way to run the entire stack with a single command.

**1. Clone the repository**

```bash
git clone https://github.com/your-username/e-shop.git
cd e-shop
```

**2. Start all services**

```bash
docker compose up --build
```

This will spin up:
- PostgreSQL 16 (port `5432`)
- Redis 7 (port `6379`)
- Zookeeper + Kafka (port `9092`)
- All 5 application services

Wait for all containers to be healthy (takes ~60s on first run due to Rust compilation).

**3. Verify everything is up**

```bash
docker compose ps
```

All services should show `healthy` or `running`.

**4. Test the API**

Register a user:
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secret123"}'
```

Login and get JWT:
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secret123"}'
```

Use the token for protected routes:
```bash
TOKEN="<paste your token here>"

# List products
curl http://localhost:8080/api/products

# View basket
curl http://localhost:8080/api/basket \
  -H "Authorization: Bearer $TOKEN"
```

---

### Running Locally (without Docker)

Requires PostgreSQL, Redis, and Kafka running locally or via Docker.

**1. Start infrastructure only**

```bash
docker compose up postgres redis zookeeper kafka -d
```

**2. Set up environment**

Copy and configure environment variables:
```bash
cp .env.example .env   # edit values if needed
```

Default `.env`:
```env
CATALOG_DATABASE_URL=postgres://eshop:eshop@localhost:5432/catalog
ORDERING_DATABASE_URL=postgres://eshop:eshop@localhost:5432/ordering
IDENTITY_DATABASE_URL=postgres://eshop:eshop@localhost:5432/identity
REDIS_URL=redis://:secret@localhost:6379
KAFKA_BROKERS=localhost:9092
JWT_SECRET=change-this-in-production
CATALOG_SERVICE_URL=http://localhost:3000
BASKET_SERVICE_URL=http://localhost:3001
ORDERING_SERVICE_URL=http://localhost:3002
IDENTITY_SERVICE_URL=http://localhost:3003
```

**3. Run database migrations**

```bash
just catalog-db-reload
just ordering-db-reload
just identity-db-reload
```

**4. Start each service in a separate terminal**

```bash
# Terminal 1
cargo run --bin catalog-service

# Terminal 2
cargo run --bin basket-service

# Terminal 3
cargo run --bin ordering-service

# Terminal 4
cargo run --bin identity-service

# Terminal 5
cargo run --bin gateway
```

---

## Project Structure

```
e-shop/
├── bins/
│   ├── gateway/              # API Gateway (routing, JWT validation)
│   ├── catalog-service/      # Products & categories
│   ├── basket-service/       # Shopping cart (Redis)
│   ├── ordering-service/     # Order management + Kafka producer
│   └── identity-service/     # Auth, JWT, user management
├── scripts/
│   └── init.sql              # Database schema + seed data
├── docker-compose.yml
├── justfile                  # Dev task runner
└── Cargo.toml                # Workspace definition
```

---

## Development

### Useful `just` commands

```bash
just catalog-db-reload    # Drop and recreate catalog DB
just ordering-db-reload   # Drop and recreate ordering DB
just identity-db-reload   # Drop and recreate identity DB
just sqlx-prepare         # Regenerate sqlx offline query cache
just check                # cargo check with SQLX_OFFLINE=true
```

### Stopping the stack

```bash
docker compose down          # Stop containers
docker compose down -v       # Stop and remove volumes (wipes data)
```

---

## License

MIT
