# Aerugo

[![CI/CD](https://github.com/AI-Decenter/Aerugo/actions/workflows/ci.yml/badge.svg)](https://github.com/AI-Decenter/Aerugo/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Project Status: Active](https://img.shields.io/badge/Project%20Status-Active-green.svg)](https://github.com/AI-Decenter/Aerugo/)

**Aerugo** is a next-generation, distributed, and multi-tenant container registry built with Rust. It is designed for high performance, security, and scalability, leveraging an S3-compatible object storage backend for infinite scalability of container images.

> **Project Status (September 2025)**: Aerugo is actively under development. Core features including authentication, user management, organization management, and repository management are complete. Registry API implementation is in progress.

## 📋 Table of Contents

- [Core Features](#-core-features)
- [Architecture](#-architecture)
- [API Overview](#-api-overview)
- [Development Setup](#-development-setup)
- [Getting Started](#-getting-started)
- [Project Structure](#-project-structure)
- [Contributing](#-contributing)
- [Roadmap](#-roadmap)
- [License](#-license)

## ✨ Core Features

- **🔄 Distributed & Highly Available:** Designed from the ground up to run in a clustered environment with no single point of failure
- **🏢 Multi-tenancy:** First-class support for individual users and organizations, allowing for the creation and management of private registries with granular access control
- **☁️ S3-Compatible Backend:** Uses any S3-compatible object storage (AWS S3, MinIO, Ceph, etc.) for storing container image layers, ensuring durability and scalability
- **🦀 Written in Rust:** Provides memory safety, concurrency, and performance, making it a secure and efficient core for your registry infrastructure
- **🐳 Docker Registry V2 API Compliant:** Fully compatible with the Docker client and other OCI-compliant tools
- **🚀 Modern Management API:** A separate, clean RESTful API for programmatic management of users, organizations, repositories, and permissions

## 📊 Implementation Status

| Feature | Status | Description |
|---------|--------|-------------|
| Configuration System | ✅ Complete | Environment variables, config files, validation |
| Database Layer | ✅ Complete | Schema design, migrations, models, and query functionality |
| Authentication | ✅ Complete | JWT tokens, API keys, login/registration, permissions system |
| User Management | ✅ Complete | User profiles, password management, search |
| Organization Management | ✅ Complete | Create/update/delete orgs, member management |
| Repository Management | ✅ Complete | Create/update/delete repos, access control |
| **API Key Authentication** | ✅ **NEW!** | **Hỗ trợ API key song song JWT, dual authentication** |
| **Docker Authentication** | ✅ **Complete** | **JWT & Basic auth, permission-based access** |
| Registry API | 🔄 In Progress | Docker Registry V2 API implementation |
| S3 Storage Integration | 🔄 In Progress | Integration with S3-compatible storage |
| Cache System | 📝 Planned | Redis-based caching for performance |
| Metrics & Monitoring | 📝 Planned | Prometheus metrics, health checks, logging |
| Horizontal Scaling | 📝 Planned | Multi-node cluster support |

---

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ and Cargo
- Docker (for development services)
- Git

**That's it!** Our development scripts handle everything else automatically.

### Quick Start
1. **Clone the repository:**
   ```bash
   git clone https://github.com/AI-Decenter/Aerugo.git
   cd Aerugo
   ```

2. **Set up development environment (one command!):**
   ```bash
   ./scripts/dev.sh setup
   ```
   This script will:
   - Check Docker installation
   - Set up PostgreSQL, Redis, and MinIO containers with proper ports
   - Create necessary databases and buckets
   - Configure all services according to your `.env` file

3. **Start developing:**
   ```bash
   ./scripts/dev.sh run
   ```

4. **Run tests (in another terminal):**
   ```bash
   ./runtest.sh
   ```

### Development Commands

The `./scripts/dev.sh` script provides everything you need:

```bash
# Infrastructure management
./scripts/dev.sh setup     # Initial setup (run once)
./scripts/dev.sh start     # Start all services
./scripts/dev.sh stop      # Stop all services  
./scripts/dev.sh restart   # Restart all services
./scripts/dev.sh status    # Check service status
./scripts/dev.sh clean     # Reset everything

# Development workflow
./scripts/dev.sh build     # Build the application
./scripts/dev.sh run       # Run in development mode
./scripts/dev.sh test      # Run Rust tests
./scripts/dev.sh check     # Quick code check
./scripts/dev.sh fmt       # Format code

# Service access
./scripts/dev.sh psql      # Connect to PostgreSQL
./scripts/dev.sh redis-cli # Connect to Redis
./scripts/dev.sh minio     # Open MinIO console
```

### API Documentation
The API documentation is available at `http://localhost:8080/api/docs` when the server is running.

## 🔐 API Key Authentication (Tiếng Việt)

Aerugo bây giờ hỗ trợ **hệ thống API key song song với JWT authentication**, cho phép bạn có thể sử dụng cả hai phương pháp xác thực:

### Cách hoạt động của API Key

1. **Format API Key**: API key có format `ak_<32_ký_tự_ngẫu_nhiên>` (ví dụ: `ak_1234567890abcdef1234567890abcdef`)
2. **Lưu trữ bảo mật**: API key được hash bằng SHA-256 trước khi lưu vào database
3. **Các cách sử dụng**:
   - **Header Authorization**: `Authorization: Bearer ak_your_api_key_here`
   - **Header X-API-Key**: `X-API-Key: ak_your_api_key_here`
4. **Fallback thông minh**: Nếu không có API key hoặc API key không hợp lệ, hệ thống sẽ tự động thử JWT authentication

### Database Schema cho API Keys (Simplified)

```sql
CREATE TABLE api_keys (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    key_hash VARCHAR(128) NOT NULL UNIQUE,      -- SHA-256 hash của API key
    name VARCHAR(64) NOT NULL,                  -- Tên mô tả của key
    expires_at TIMESTAMP,                       -- Thời gian hết hạn (optional)
    last_used_at TIMESTAMP,                     -- Lần cuối sử dụng
    is_active BOOLEAN DEFAULT true,             -- Trạng thái kích hoạt
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### Ví dụ sử dụng API Key

```bash
# Sử dụng với Authorization header
curl -H "Authorization: Bearer ak_1234567890abcdef1234567890abcdef" \
     https://your-aerugo.com/api/v1/repos/repositories

# Sử dụng với X-API-Key header  
curl -H "X-API-Key: ak_1234567890abcdef1234567890abcdef" \
     https://your-aerugo.com/api/v1/organizations

# JWT vẫn hoạt động bình thường
curl -H "Authorization: Bearer <jwt_token>" \
     https://your-aerugo.com/api/v1/repos/repositories
```

### Ưu điểm của API Key (Simplified)

- **Dễ sử dụng**: Không cần refresh token như JWT
- **Bảo mật tốt**: Hash SHA-256, có thể set thời gian hết hạn
- **Cache performance**: API key được cache để tối ưu hiệu suất
- **Tương thích hoàn toàn**: JWT authentication vẫn hoạt động bình thường
- **Không có conflict**: Hai hệ thống hoạt động song song, không xung đột
- **Full quyền**: API key có toàn quyền như JWT, không cần phân quyền phức tạp

### Các API endpoints được hỗ trợ

API key hiện tại hỗ trợ tất cả các protected endpoints:
- ✅ **Authentication APIs**: `/api/v1/auth/*` (trừ login/register)
- ✅ **Organizations APIs**: `/api/v1/organizations/*`
- ✅ **Repositories APIs**: `/api/v1/repos/*`
- ✅ **Storage APIs**: `/api/v1/storage/*` (nếu được protected)

---

## 🔐 Authentication System

Aerugo now supports full Docker Registry V2 authentication! All push/pull operations require proper authentication.

### Quick Start with Docker

```bash
# 1. Start the registry
./scripts/dev.sh run

# 2. Register a new user (via API)
curl -X POST http://localhost:8080/auth/register \
     -H "Content-Type: application/json" \
     -d '{"username":"myuser","password":"mypass","email":"user@example.com"}'

# 3. Login with Docker CLI
docker login localhost:8080
Username: myuser
Password: mypass

# 4. Now you can push/pull images!
docker tag nginx:latest localhost:8080/myorg/nginx:latest
docker push localhost:8080/myorg/nginx:latest
docker pull localhost:8080/myorg/nginx:latest
```

### Authentication Methods

- **🔑 Basic Authentication**: For Docker CLI and container runtimes
- **🎫 JWT Bearer Tokens**: For web applications and API clients
- **🛡️ Permission-Based Access**: Pull, push, and delete permissions per repository
- **👥 Organization-Level Control**: Team-based access management

See [Docker Authentication Guide](docs/DOCKER_AUTHENTICATION.md) for detailed documentation.

### Test Authentication

```bash
# Run comprehensive authentication tests
./test_docker_auth.sh
```

## 🏛️ Architecture

Aerugo operates on a shared-nothing, stateless node architecture. This allows for effortless horizontal scaling by simply adding more nodes behind a load balancer. The state is managed externally in a dedicated metadata store and the S3 backend.

### High-Level Architecture Diagram

```
        ┌─────────────────────────────────┐
        │   Docker Client / Admin Client  │
        └────────────────┬────────────────┘
                         │
           ┌─────────────┴─────────────┐
           │ HTTPS (Registry & Mgmt API) │
           ▼                             ▼
┌───────────────────────────────────────────────────┐
│                  Load Balancer                    │
└───────────────────────────────────────────────────┘
           │              │              │
           ▼              ▼              ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ Aerugo Node  │ │ Aerugo Node  │ │ Aerugo Node  │
    │   (Rust)     │ │   (Rust)     │ │   (Rust)     │  ◀── Stateless, Scalable Service
    └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
           │              │              │
           │       ┌──────┴──────┐       │
           │       │             │       │
           ▼       ▼             ▼       ▼
┌─────────────────────┐     ┌─────────────────────┐
│   Metadata Store    │◀────│    Cache Layer      │
│ (e.g., PostgreSQL,  │     │   (e.g., Redis)     │
│     CockroachDB)    │     └─────────────────────┘
└─────────────────────┘
           ▲
           │ (Manages users, orgs, permissions, manifests, tags)
           │
           └─────────────────────────────────────────────────────┐
                                                                 │
                                                                 ▼ (Generates presigned URLs for blobs)
                                               ┌─────────────────────────┐
                                               │      S3-Compatible      │
                                               │      Object Storage     │
                                               └─────────────────────────┘
                                                         ▲
                                                         │
                                                         │ (Direct Blob Upload/Download)
                                                         └──────────────────────▶ Docker Client
```

### Component Breakdown

#### Load Balancer
The entry point for all traffic. It distributes requests across the available Aerugo nodes. It should handle TLS termination.

#### Aerugo Nodes
These are the stateless, core application instances written in Rust.

- They handle all API logic for both the Docker Registry V2 API and the Management API
- They authenticate and authorize requests by querying the Metadata Store
- For blob operations (pushes/pulls), they do not proxy the data. Instead, they generate pre-signed URLs for the client to interact directly with the S3-compatible backend. This drastically reduces the load on the registry nodes and improves performance

#### Metadata Store
A transactional, persistent database (e.g., PostgreSQL, CockroachDB) that stores all non-blob data:

- User and Organization accounts
- Repository information and permissions
- Image manifests and tags
- Authentication tokens and API keys

#### S3-Compatible Object Storage
This is the storage layer for the actual content of the container images (the layers, or "blobs"). By offloading this to an S3-compatible service, Aerugo can scale its storage capacity independently and benefit from the durability features of these systems.

#### Cache Layer
A distributed cache (e.g., Redis) is used to cache frequently accessed metadata, such as manifest data and authorization decisions, to reduce latency and load on the Metadata Store.

## ⚙️ API Overview

Aerugo exposes two primary APIs on the same port, routed by the application based on the request path.

### 1. Registry API (`/v2/`)
Implements the Docker Registry V2 API specification.

- Handles `docker pull`, `docker push`, and other OCI-related commands
- Authentication is typically done via Bearer tokens

### 2. Management API (`/api/v1/`)
A RESTful API for administrative and user-level management tasks. All responses are in JSON.

#### Key Endpoints (Conceptual):

**Authentication:**
- `POST /api/v1/auth/token`: Exchange credentials for a JWT

**Users:**
- `POST /api/v1/users`: Create a new user
- `GET /api/v1/users/{username}`: Get user details

**Organizations:**
- `POST /api/v1/orgs`: Create a new organization
- `GET /api/v1/orgs/{org_name}`: Get organization details
- `POST /api/v1/orgs/{org_name}/members`: Add a user to an organization

**Repositories:**
- `GET /api/v1/repos/{namespace}/{repo_name}`: Get repository details and tags
- `DELETE /api/v1/repos/{namespace}/{repo_name}`: Delete a repository
- `PUT /api/v1/repos/{namespace}/{repo_name}/permissions`: Set user/team permissions for a repository

## 🛠️ Development Setup

**TL;DR**: Just run `./scripts/dev.sh setup` and you're ready to develop!

### Prerequisites

- **Docker** - For running development services (PostgreSQL, Redis, MinIO)
- **Rust 1.70+** - The programming language and toolchain
- **Git** - Version control

Install prerequisites:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Docker (follow official docs for your OS)
# https://docs.docker.com/get-docker/
```

### Automated Setup (Recommended)

Our development scripts handle everything automatically:

```bash
# 1. Clone and enter the project
git clone https://github.com/AI-Decenter/Aerugo.git
cd Aerugo

# 2. One command setup - this handles everything!
./scripts/dev.sh setup

# 3. Start developing
./scripts/dev.sh run
```

**What the setup script does:**
- ✅ Validates your environment configuration
- ✅ Creates Docker containers for PostgreSQL (port 5433), Redis (port 6380), and MinIO (port 9001/9002)
- ✅ Sets up databases and S3 buckets with proper permissions  
- ✅ Uses non-default ports to avoid conflicts with existing services
- ✅ Configures everything according to your `.env` file

### Development Workflow

All development tasks are handled by the `dev.sh` script:

```bash
# Daily workflow
./scripts/dev.sh start    # Start all services (if stopped)
./scripts/dev.sh run      # Run Aerugo in development mode
./scripts/dev.sh test     # Run tests
./scripts/dev.sh stop     # Stop services when done

# Code quality
./scripts/dev.sh fmt      # Format code
./scripts/dev.sh check    # Quick syntax check
cargo clippy              # Linting (manual command)

# Database/service access
./scripts/dev.sh psql     # Connect to PostgreSQL
./scripts/dev.sh minio    # Open MinIO web console
./scripts/dev.sh redis-cli # Connect to Redis

# Troubleshooting
./scripts/dev.sh status   # Check all services
./scripts/dev.sh logs     # View service logs
./scripts/dev.sh clean    # Reset everything if issues occur
```

### Environment Configuration

The setup script reads from your `.env` file. Default configuration works out-of-the-box:

```bash
# Database (PostgreSQL on non-default port)
DATABASE_URL=postgresql://aerugo:development@localhost:5433/aerugo_dev

# Cache (Redis on non-default port)
REDIS_URL=redis://localhost:6380

# Storage (MinIO S3-compatible on non-default ports)
S3_ENDPOINT=http://localhost:9001
S3_BUCKET=aerugo-registry
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin

# Server Configuration  
LISTEN_ADDRESS=0.0.0.0:8080
LOG_LEVEL=debug

# Security
JWT_SECRET=test-integration-secret-key-do-not-use-in-production
```

**Need custom ports?** Just edit `.env` and run `./scripts/dev.sh setup` again.

### IDE Setup (Optional)

**VS Code (Recommended):**
```bash
# Install essential extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension tamasfe.even-better-toml
```

**Other IDEs:** Install Rust plugin and configure rust-analyzer LSP.

### Manual Setup (Advanced)

If you prefer manual setup or need custom configuration:

```bash
# Run database migrations
sqlx migrate run

# Reset database (drops all data)
sqlx database reset

# Create new migration
sqlx migrate add migration_name
```

### Troubleshooting Common Issues

#### Issue: Rust compilation errors
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean
```

#### Issue: Database connection errors
```bash
# Check if PostgreSQL is running
docker ps | grep postgres

# Check connection
psql postgresql://aerugo:development@localhost:5432/aerugo_dev
```

#### Issue: S3 storage connection errors
```bash
# For MinIO, check web console at http://localhost:9001
# Default credentials: minioadmin/minioadmin

# Test S3 connection with AWS CLI
aws --endpoint-url http://localhost:9000 s3 ls
```

### Contributing Guidelines

1. **Fork the repository** and create a feature branch
2. **Write tests** for new functionality
3. **Follow Rust conventions** (use `cargo fmt` and `cargo clippy`)
4. **Document your changes** with clear commit messages
5. **Ensure all tests pass** before submitting a PR
6. **Update documentation** if you're changing APIs or adding features

### Performance and Debugging Tools

```bash
# Install performance profiling tools
cargo install flamegraph
cargo install cargo-profdata

# Profile application
cargo flamegraph --bin aerugo

## 🚀 Getting Started

Once you have completed the [Development Setup](#-development-setup), follow these steps to get Aerugo running locally:

### Quick Start

1. **Install additional development tools:**
   ```bash
   # Rust development tools  
   rustup component add rustfmt clippy
   cargo install cargo-watch cargo-audit

   # Optional: Database migration tool (when migrations are added)
   cargo install sqlx-cli --no-default-features --features postgres
   ```

2. **Start the required services manually:**
   ```bash
   # PostgreSQL (note: using non-default port to avoid conflicts)
   docker run -d --name aerugo-postgres \
     -e POSTGRES_DB=aerugo_dev \
     -e POSTGRES_USER=aerugo \
     -e POSTGRES_PASSWORD=development \
     -p 5433:5432 postgres:15

   # Redis (note: using non-default port)
   docker run -d --name aerugo-redis -p 6380:6379 redis:7-alpine

   # MinIO S3-compatible storage (note: using non-default ports)
   docker run -d --name aerugo-minio \
     -p 9001:9000 -p 9002:9001 \
     -e MINIO_ACCESS_KEY=minioadmin \
     -e MINIO_SECRET_KEY=minioadmin \
     minio/minio server /data --console-address ":9001"
   ```

3. **Create S3 bucket:**
   ```bash
   # Install MinIO client
   curl -L https://dl.min.io/client/mc/release/linux-amd64/mc -o mc
   chmod +x mc && sudo mv mc /usr/local/bin/

   # Configure and create bucket
   mc alias set local http://localhost:9001 minioadmin minioadmin
   mc mb local/aerugo-registry
   mc anonymous set public local/aerugo-registry
   ```

4. **Build and run:**
   ```bash
   cargo build
   cargo run
   ```

### Testing

Run the comprehensive test suite:

```bash
# Integration and API tests
./runtest.sh

# Unit tests only
cargo test

# Test with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Configuration

All configuration is managed through the `.env` file:

#### Environment Variables
```bash
# Database Configuration
DATABASE_URL=postgresql://aerugo:development@localhost:5433/aerugo_dev
DATABASE_REQUIRE_SSL=false
DATABASE_MIN_CONNECTIONS=5
DATABASE_MAX_CONNECTIONS=20

# Redis Configuration
REDIS_URL=redis://localhost:6380
REDIS_POOL_SIZE=10
REDIS_TTL_SECONDS=3600

# S3 Configuration (MinIO)
S3_ENDPOINT=http://localhost:9001
S3_BUCKET=aerugo-registry
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_REGION=us-east-1
S3_USE_PATH_STYLE=true

# Server Configuration
LISTEN_ADDRESS=0.0.0.0:8080
API_PREFIX=/api/v1
LOG_LEVEL=debug

# JWT Configuration
JWT_SECRET=test-integration-secret-key-do-not-use-in-production
JWT_EXPIRATION_SECONDS=3600
REFRESH_TOKEN_EXPIRATION_SECONDS=604800
```

**Production configuration** should use secure values, SSL connections, and production-grade secrets.

### Testing Your Setup

1. **Check all services:**
   ```bash
   ./scripts/dev.sh status
   ```

2. **Test the API:**
   ```bash
   # Start the server
   ./scripts/dev.sh run

   # In another terminal, test health endpoint
   curl http://localhost:8080/api/v1/health

   # Run comprehensive tests
   ./runtest.sh
   ```

3. **Access web interfaces:**
   ```bash
   # MinIO Console
   ./scripts/dev.sh minio
   # Or manually: http://localhost:9002 (admin/admin)

   # API Documentation  
   # http://localhost:8080/api/docs (when server is running)
   ```

### Troubleshooting

**Services won't start?**
```bash
# Check what's using your ports
sudo lsof -i :5433 -i :6380 -i :9001

# Reset everything and try again
./scripts/dev.sh clean
./scripts/dev.sh setup
```

**Database connection issues?**
```bash
# Check PostgreSQL container
./scripts/dev.sh logs

# Connect manually to debug
./scripts/dev.sh psql
```

**MinIO/S3 issues?**
```bash
# Check MinIO status
curl http://localhost:9001/minio/health/ready

# Access MinIO console
./scripts/dev.sh minio
```

**Need different ports?**
Edit your `.env` file and re-run setup:
```bash
# Edit .env with your preferred ports
nano .env

# Apply changes  
./scripts/dev.sh setup
```

For more detailed troubleshooting, see [scripts/README.md](scripts/README.md).

## 📁 Project Structure

> **Note:** This project is in early development. The structure below represents the planned organization once implementation begins.

```
Aerugo/
├── .github/                    # GitHub workflows and templates
│   ├── workflows/
│   │   ├── ci.yml             # Continuous Integration
│   │   ├── cd.yml             # Continuous Deployment
│   │   └── security.yml       # Security scanning
│   └── ISSUE_TEMPLATE/
├── src/                        # Main application source code
│   ├── main.rs                # Application entry point
│   ├── lib.rs                 # Library root
│   ├── api/                   # API layer modules
│   │   ├── mod.rs
│   │   ├── registry/          # Docker Registry V2 API
│   │   │   ├── mod.rs
│   │   │   ├── blobs.rs       # Blob operations
│   │   │   ├── manifests.rs   # Manifest operations
│   │   │   └── catalog.rs     # Repository catalog
│   │   └── management/        # Management API
│   │       ├── mod.rs
│   │       ├── auth.rs        # Authentication endpoints
│   │       ├── users.rs       # User management
│   │       ├── orgs.rs        # Organization management
│   │       └── repos.rs       # Repository management
│   ├── auth/                  # Authentication and authorization
│   │   ├── mod.rs
│   │   ├── jwt.rs             # JWT token handling
│   │   ├── permissions.rs     # Permission checking
│   │   └── middleware.rs      # Auth middleware
│   ├── storage/               # Storage abstraction layer
│   │   ├── mod.rs
│   │   ├── s3.rs              # S3-compatible storage
│   │   └── metadata.rs        # Metadata operations
│   ├── database/              # Database layer
│   │   ├── mod.rs
│   │   ├── models.rs          # Database models
│   │   ├── migrations/        # SQL migrations
│   │   └── queries.rs         # Database queries
│   ├── cache/                 # Caching layer
│   │   ├── mod.rs
│   │   └── redis.rs           # Redis implementation
│   ├── config/                # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs        # Application settings
│   └── utils/                 # Utility modules
│       ├── mod.rs
│       ├── crypto.rs          # Cryptographic utilities
│       └── errors.rs          # Error types and handling
├── tests/                      # Integration tests
│   ├── common/                # Shared test utilities
│   ├── api_tests.rs           # API endpoint tests
│   └── storage_tests.rs       # Storage layer tests
├── docs/                       # Documentation
│   ├── API.md                 # API documentation
│   ├── DEPLOYMENT.md          # Deployment guide
│   └── DEVELOPMENT.md         # Development guide
├── scripts/                    # Build and deployment scripts
│   ├── build.sh              # Build script
│   ├── test.sh               # Test script
│   └── deploy.sh             # Deployment script
├── migrations/                 # Database migrations
├── config/                     # Configuration examples
│   ├── config.example.toml
│   └── docker-compose.dev.yml
├── Cargo.toml                 # Rust project configuration
├── Cargo.lock                 # Dependency lock file
├── Dockerfile                 # Container image definition
├── docker-compose.yml         # Multi-container orchestration
├── .env.example              # Environment variables example
├── .gitignore                # Git ignore rules
├── LICENSE                   # Apache 2.0 license
└── README.md                 # This file
```

### Key Directories Explained

- **`src/api/`**: Contains all HTTP API handlers for both the Docker Registry V2 API and the Management API
- **`src/auth/`**: Authentication and authorization logic, including JWT handling and permission systems
- **`src/storage/`**: Abstraction layer for different storage backends (S3, filesystem, etc.)
- **`src/database/`**: Database models, queries, and migration management
- **`src/cache/`**: Caching layer implementation for performance optimization
- **`tests/`**: Integration tests that verify the entire system works correctly
- **`docs/`**: Additional documentation beyond this README
- **`scripts/`**: Automation scripts for building, testing, and deployment
## 🤝 Contributing

We welcome contributions to Aerugo! Whether you're fixing bugs, adding features, improving documentation, or helping with testing, your contributions are valued.

### How to Contribute

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following our coding standards
4. **Write or update tests** for your changes
5. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```
6. **Commit your changes** with a clear commit message:
   ```bash
   git commit -m "Add feature: brief description of what you added"
   ```
7. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Open a Pull Request** on GitHub with a clear description of your changes

### Development Guidelines

#### Code Style
- Follow Rust's official style guidelines (enforced by `rustfmt`)
- Use `cargo clippy` to catch common mistakes and improve code quality
- Write clear, descriptive variable and function names
- Add documentation comments (`///`) for public APIs

#### Commit Messages
Follow the conventional commit format:
```
type(scope): brief description

Longer description if necessary

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(api): add user authentication endpoint`
- `fix(storage): handle S3 connection timeout errors`
- `docs(readme): update development setup instructions`

#### Testing
- Write unit tests for new functions and methods
- Add integration tests for API endpoints
- Ensure all tests pass before submitting PR
- Aim for good test coverage of new code

#### Pull Request Guidelines
- Keep PRs focused on a single feature or fix
- Include tests for new functionality
- Update documentation if necessary
- Respond to feedback and be willing to make changes
- Ensure your branch is up to date with `main`

### Reporting Issues

When reporting bugs or requesting features, please:

1. **Check existing issues** to avoid duplicates
2. **Use the issue templates** provided
3. **Provide clear reproduction steps** for bugs
4. **Include relevant logs or error messages**
5. **Specify your environment** (OS, Rust version, etc.)

### Areas Where We Need Help

- **Core Implementation**: Help implement the Docker Registry V2 API
- **Authentication System**: JWT-based auth and permissions
- **Storage Layer**: S3-compatible backend integration
- **Testing**: Integration tests and performance testing
- **Documentation**: API docs, deployment guides, examples
- **DevOps**: CI/CD improvements, deployment automation
- **Security**: Security reviews and vulnerability testing

### Setting Up for Development

See the [Development Setup](#-development-setup) section for detailed instructions on setting up your development environment.

### Community

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: [Join our Discord server](https://discord.gg/aerugo) (link TBD)

### Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. We are committed to providing a welcoming and inclusive environment for all contributors.

## 🗺️ Roadmap

### Phase 1: Core Foundation
- [x] Core architecture design
- [x] Project structure and documentation
- [x] **Implementation plan created** (30 detailed GitHub issues)
- [ ] Basic server setup and configuration system
- [ ] Database schema and migrations
- [ ] S3 storage integration

### Phase 2: Registry API Implementation
- [ ] Docker Registry V2 API endpoints
  - [ ] Blob operations (upload/download)
  - [ ] Manifest operations
  - [ ] Repository catalog
- [ ] Authentication middleware
- [ ] Basic authorization system

### Phase 3: Management API
- [ ] User management endpoints
- [ ] Organization management
- [ ] Repository permissions system
- [ ] JWT-based authentication

### Phase 4: Advanced Features
- [ ] Multi-tenancy support
- [ ] Granular access controls
- [ ] Caching layer (Redis integration)
- [ ] Metrics and monitoring

### Phase 5: Production Readiness
- [ ] Performance optimization
- [ ] Comprehensive testing (unit, integration, e2e)
- [ ] Security hardening
- [ ] Documentation and deployment guides

### Phase 6: Deployment & Operations
- [ ] Docker containerization
- [ ] Kubernetes deployment manifests
- [ ] CI/CD pipeline setup
- [ ] Monitoring and alerting

### Long-term Goals
- [ ] High availability and clustering
- [ ] Advanced storage backends
- [ ] Image scanning integration
- [ ] Webhook support for integrations

## 📋 Implementation Guide

**Ready to start development?** We've created a comprehensive implementation plan:

- **[📋 IMPLEMENTATION_ISSUES.md](./IMPLEMENTATION_ISSUES.md)** - Detailed list of 30 GitHub issues covering everything from project initialization to comprehensive testing
- **[📊 IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md)** - Quick reference guide with timelines, critical paths, and risk mitigation
- **[🗺️ ROADMAP.md](./ROADMAP.md)** - Visual roadmap with dependencies, milestones, and resource allocation
- **[🔧 scripts/create_issues.sh](./scripts/create_issues.sh)** - Helper script to create GitHub issues from the implementation plan

### Quick Start for Developers

1. **Review the implementation plan**: Start with `IMPLEMENTATION_SUMMARY.md` for an overview
2. **Create GitHub issues**: Use the detailed descriptions in `IMPLEMENTATION_ISSUES.md`  
3. **Follow the roadmap**: Use `ROADMAP.md` to understand dependencies and timeline
4. **Begin with Issue #1**: "Initialize Rust Project Structure" - the foundation for everything else

The implementation is structured as **9 phases with 30 detailed issues**, taking an estimated **4-5.5 months** for full completion with comprehensive testing.

> **Current Status**: Implementation plan complete. Ready to begin Phase 1 development.

## 📜 License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

### Why Apache 2.0?

We chose Apache 2.0 because it:
- Allows both personal and commercial use
- Provides patent protection for users
- Is compatible with many other open source licenses
- Encourages contribution while protecting contributors

---

<div align="center">

**Built with ❤️ by the Aerugo team**

[Report Bug](https://github.com/AI-Decenter/Aerugo/issues) • [Request Feature](https://github.com/AI-Decenter/Aerugo/issues) • [Contribute](CONTRIBUTING.md)

</div>
