# E-Shop — Microservices E-Commerce Platform

A production-style e-commerce backend built entirely in **Rust**, following a microservices architecture. Features an API Gateway, event-driven communication via **Kafka**, JWT authentication, and PostgreSQL + Redis persistence. Deployable via Docker Compose or Kubernetes (Minikube).

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
                    └──────┬───────┘  └────────────┘  │  └──────────────────┘
                           │                           │
                           │   ┌───────────────────────▼───┐
                           │   │      Ordering Service     │
                           │   │      :3002                │
                           │   │      PostgreSQL           │
                           │   └─────────┬─────────────────┘
                           │             │  Publishes: order-created
                           │             ▼
                           │   ┌─────────────────────┐
                           └───►       KAFKA          │
                               │  Topic: order-created│
                               └─────────┬────────────┘
                                         │ Consumes
                                         ▼
                                  Catalog Service
                                 (decrement stock)
```

### Services

| Service | Port | Responsibility |
|---|---|---|
| **API Gateway** | `8080` | Single entry point — routes requests, validates JWT, injects `x-user-id` header |
| **Catalog Service** | `3000` | Products & categories CRUD; consumes `order-created` to decrement stock |
| **Basket Service** | `3001` | Shopping cart backed by Redis |
| **Ordering Service** | `3002` | Order creation & status management; publishes `order-created` Kafka events |
| **Identity Service** | `3003` | User registration, login, JWT issuance; Argon2 password hashing |

### Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (edition 2021) |
| Web Framework | Axum 0.7 + Tokio |
| Databases | PostgreSQL 16, Redis 7 |
| Message Broker | Apache Kafka (KRaft mode) |
| Kafka Client | rdkafka |
| Auth | JWT (jsonwebtoken 9), Argon2 |
| DB Driver | sqlx 0.7 (async, compile-time checked) |
| Containerization | Docker + Docker Compose |
| Orchestration | Kubernetes (Minikube) |

---

## API Reference

All requests go through the **Gateway at `http://localhost:8080`**. Protected routes require `Authorization: Bearer <token>`.

### Authentication

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `POST` | `/api/auth/register` | Public | Register a new user |
| `POST` | `/api/auth/login` | Public | Login and receive JWT |
| `GET` | `/api/auth/me` | JWT | Get current user profile |

### Catalog

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `GET` | `/api/products` | Public | List all products (optional `?category_id=`) |
| `GET` | `/api/products/:id` | Public | Get product by ID |
| `POST` | `/api/products` | JWT | Create product |
| `PUT` | `/api/products/:id` | JWT | Update product |
| `DELETE` | `/api/products/:id` | JWT | Delete product |
| `GET` | `/api/categories` | Public | List all categories |
| `POST` | `/api/categories` | JWT | Create category |

### Basket

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `GET` | `/api/basket/:user_id` | JWT | Get basket for a user |
| `POST` | `/api/basket/:user_id` | JWT | Add item to basket |
| `DELETE` | `/api/basket/:user_id` | JWT | Clear entire basket |
| `DELETE` | `/api/basket/:user_id/:product_id` | JWT | Remove specific item |

### Orders

| Method | Endpoint | Auth | Description |
|---|---|---|---|
| `POST` | `/api/orders` | JWT | Create order (prices fetched from catalog) |
| `GET` | `/api/orders/:id` | JWT | Get order with items |
| `GET` | `/api/orders/user/:user_id` | JWT | List all orders for a user |
| `PUT` | `/api/orders/:id/status` | JWT | Update order status |

**Order status transitions:** `Pending → Confirmed`, `Pending → Cancelled`, `Confirmed → Cancelled`

---

## Event Flow

```
POST /api/orders
       │
       ▼
Ordering Service
  ├── Fetches prices via HTTP → Catalog Service (client cannot set prices)
  ├── Validates stock availability
  ├── Persists order + items to PostgreSQL (single transaction)
  └── Publishes → Kafka topic: order-created
                        │
                        ▼
                 Catalog Service
                (decrements stock)
```

---

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose v2
- [Rust toolchain](https://rustup.rs/) (for local development)
- [just](https://github.com/casey/just) (task runner)

---

### Running with Docker Compose

**1. Clone and configure**

```bash
git clone https://github.com/your-username/e-shop.git
cd e-shop
cp .env.example .env
```

**2. Start all services**

```bash
docker compose up --build
```

Starts PostgreSQL, Redis, Kafka, and all 5 application services. First build takes 5–15 minutes (Rust + rdkafka compilation). Subsequent builds use cache.

**3. Run migrations and seed data**

```bash
just catalog-migrate && just catalog-seed
just ordering-migrate
just identity-migrate
```

**4. Run the test suite**

```bash
just test
```

---

### Running on Kubernetes (Minikube)

**Prerequisites:** [minikube](https://minikube.sigs.k8s.io/docs/start/), [kubectl](https://kubernetes.io/docs/tasks/tools/)

**1. Start the cluster**

```bash
minikube start --driver=docker
```

**2. Build images and load into Minikube**

```bash
docker compose build
minikube image load e-shop-catalog-service:latest
minikube image load e-shop-basket-service:latest
minikube image load e-shop-ordering-service:latest
minikube image load e-shop-identity-service:latest
minikube image load e-shop-gateway:latest
```

**3. Deploy everything**

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/postgres/
kubectl apply -f k8s/redis/
kubectl apply -f k8s/kafka/
kubectl apply -f k8s/catalog/
kubectl apply -f k8s/basket/
kubectl apply -f k8s/ordering/
kubectl apply -f k8s/identity/
kubectl apply -f k8s/gateway/
```

**4. Run migrations**

```bash
kubectl exec -n e-shop deploy/postgres -- psql -U eshop -d catalog \
  -f /dev/stdin < bins/catalog-service/migrations/...
# or apply SQL directly — see k8s/README notes
```

**5. Access the gateway**

```bash
echo "http://$(minikube ip):30080"
```

**6. Run the test suite against Kubernetes**

```bash
just test-k8s
```

---

### Running Services Locally (no Docker)

**1. Start infrastructure**

```bash
docker compose up postgres redis kafka -d
```

**2. Configure environment**

```bash
cp .env.example .env   # values already correct for local dev
```

**3. Run migrations**

```bash
just catalog-migrate && just catalog-seed
just ordering-migrate
just identity-migrate
```

**4. Start services**

```bash
cargo run --bin catalog-service   # terminal 1
cargo run --bin basket-service    # terminal 2
cargo run --bin ordering-service  # terminal 3
cargo run --bin identity-service  # terminal 4
cargo run --bin gateway           # terminal 5
```

---

## Project Structure

```
e-shop/
├── bins/
│   ├── gateway/              # API Gateway — routing, JWT validation, reverse proxy
│   │   └── src/
│   │       ├── auth.rs       # JWT middleware
│   │       ├── proxy.rs      # HTTP reverse proxy
│   │       └── config.rs     # Upstream URL config
│   ├── catalog-service/      # Products & categories + Kafka consumer
│   ├── basket-service/       # Shopping cart (Redis) + Kafka consumer
│   ├── ordering-service/     # Order management + Kafka producer
│   └── identity-service/     # Auth, JWT, user management
│       └── src/
│           └── jwt/mod.rs    # Claims + Axum extractor
├── k8s/                      # Kubernetes manifests
│   ├── namespace.yaml
│   ├── configmap.yaml
│   ├── secret.yaml
│   ├── postgres/
│   ├── redis/
│   ├── kafka/
│   ├── catalog/
│   ├── basket/
│   ├── ordering/
│   ├── identity/
│   └── gateway/
├── scripts/
│   ├── init.sql              # Creates ordering + identity databases
│   └── test.sh               # End-to-end test suite
├── .cargo/config.toml        # SQLX_OFFLINE=true for IDE support
├── docker-compose.yml
├── justfile                  # Dev task runner
└── Cargo.toml                # Workspace definition
```

---

## Development

### `just` commands

```bash
# Database
just catalog-migrate          # Apply catalog migrations
just ordering-migrate         # Apply ordering migrations
just identity-migrate         # Apply identity migrations
just catalog-seed             # Seed catalog with test products
just catalog-db-reload        # Drop + recreate + migrate catalog DB
just ordering-db-reload       # Drop + recreate + migrate ordering DB
just identity-db-reload       # Drop + recreate + migrate identity DB

# SQLx
just sqlx-prepare             # Regenerate offline query cache (all services)
just check                    # cargo check --workspace (offline mode)

# Testing
just test                     # Run test suite against Docker Compose stack
just test-k8s                 # Run test suite against Kubernetes cluster

# Infrastructure
just redis-run                # Start Redis only
just db-stop                  # Stop PostgreSQL
```

### After changing a SQL query

```bash
just sqlx-prepare   # updates .sqlx/ cache for IDE + Docker builds
```

### Stopping

```bash
# Docker Compose
docker compose down        # stop containers, keep data
docker compose down -v     # stop containers, wipe all data

# Kubernetes
kubectl delete namespace e-shop   # removes everything in the namespace
minikube stop                     # pause the cluster
minikube delete                   # destroy the cluster entirely
```

---

## License

MIT
