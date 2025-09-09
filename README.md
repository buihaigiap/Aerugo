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
| Authentication | ✅ Complete | JWT tokens, login/registration, permissions system |
| User Management | ✅ Complete | User profiles, password management, search |
| Organization Management | ✅ Complete | Create/update/delete orgs, member management |
| Repository Management | ✅ Complete | Create/update/delete repos, access control |
| Registry API | 🔄 In Progress | Docker Registry V2 API implementation |
| S3 Storage Integration | 🔄 In Progress | Integration with S3-compatible storage |
| Cache System | 📝 Planned | Redis-based caching for performance |
| Metrics & Monitoring | 📝 Planned | Prometheus metrics, health checks, logging |
| Horizontal Scaling | 📝 Planned | Multi-node cluster support |

---

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ and Cargo
- PostgreSQL 14+
- Redis 6+
- MinIO or other S3-compatible storage
- Docker (optional, for containerized development)

### Quick Start
1. Clone the repository:
   ```bash
   git clone https://github.com/AI-Decenter/Aerugo.git
   cd Aerugo
   ```

2. Set up the development environment:
   ```bash
   # Run the development setup script
   ./scripts/setup-dev-env.sh
   ```

3. Configure the application:
   ```bash
   # Copy the default config
   cp config/default.yml .env
   
   # Edit the configuration as needed
   nano .env
   ```

4. Run the development server:
   ```bash
   # Start the development server
   ./scripts/dev.sh
   ```

5. Run tests:
   ```bash
   # Run integration tests
   ./runtest.sh
   ```

### API Documentation
The API documentation is available at `http://localhost:8080/api/docs` when the server is running.

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

This section provides a comprehensive guide for setting up your development environment to contribute to Aerugo.

### Prerequisites

Before you begin, ensure you have the following installed on your development machine:

#### Required Tools

1. **Rust Toolchain** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup update stable
   ```

2. **Git** (for version control)
   ```bash
   # On Ubuntu/Debian
   sudo apt update && sudo apt install git

   # On macOS
   brew install git

   # On Windows
   # Download from https://git-scm.com/downloads
   ```

3. **Docker & Docker Compose** (for testing and running dependencies)
   ```bash
   # Follow instructions at https://docs.docker.com/get-docker/
   # Verify installation
   docker --version
   docker-compose --version
   ```

#### External Dependencies

Aerugo requires the following services for development:

1. **PostgreSQL Database**
   ```bash
   # Using Docker (recommended for development)
   docker run --name aerugo-postgres \
     -e POSTGRES_DB=aerugo_dev \
     -e POSTGRES_USER=aerugo \
     -e POSTGRES_PASSWORD=development \
     -p 5432:5432 \
     -d postgres:15
   ```

2. **Redis Cache** (optional but recommended)
   ```bash
   docker run --name aerugo-redis \
     -p 6379:6379 \
     -d redis:7-alpine
   ```

3. **S3-Compatible Storage** (choose one):
   
   **Option A: MinIO (recommended for local development)**
   ```bash
   docker run --name aerugo-minio \
     -p 9000:9000 -p 9001:9001 \
     -e MINIO_ROOT_USER=minioadmin \
     -e MINIO_ROOT_PASSWORD=minioadmin \
     -d minio/minio server /data --console-address ":9001"
   ```
   
   **Option B: LocalStack (AWS S3 emulator)**
   ```bash
   docker run --name aerugo-localstack \
     -p 4566:4566 \
     -e SERVICES=s3 \
     -d localstack/localstack
   ```

### Development Environment Setup

#### 1. Clone the Repository

```bash
git clone https://github.com/AI-Decenter/Aerugo.git
cd Aerugo
```

#### 2. Install Development Dependencies

```bash
# Install additional Rust tools for development
rustup component add rustfmt clippy

# Install cargo-watch for auto-recompilation during development
cargo install cargo-watch

# Install cargo-audit for security vulnerability scanning
cargo install cargo-audit

# Install sqlx-cli for database migrations (when implemented)
cargo install sqlx-cli --no-default-features --features postgres
```

#### 3. Configure Your Development Environment

Create a `.env` file in the project root for development configuration:

```bash
# Copy the example environment file
cp .env.example .env  # (will be created once project structure exists)

# Edit the configuration for your local setup
nano .env
```

Example `.env` configuration:
```bash
# Database Configuration
DATABASE_URL=postgresql://aerugo:development@localhost:5432/aerugo_dev

# Redis Configuration (optional)
REDIS_URL=redis://localhost:6379

# S3 Configuration (MinIO example)
S3_ENDPOINT=http://localhost:9000
S3_BUCKET=aerugo-registry
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_REGION=us-east-1

# Server Configuration
LISTEN_ADDRESS=0.0.0.0:8080
LOG_LEVEL=debug

# JWT Configuration (generate a random secret for development)
JWT_SECRET=your-super-secret-jwt-key-for-development
```

#### 4. Set Up Your IDE/Editor

**Visual Studio Code (recommended)**
1. Install the Rust Analyzer extension
2. Install the Better TOML extension for configuration files
3. Install the Docker extension for container management

**VS Code settings.json additions:**
```json
{
    "rust-analyzer.cargo.watchOptions": {
        "allTargets": false
    },
    "rust-analyzer.check.command": "clippy",
    "editor.formatOnSave": true
}
```

**Other IDEs:**
- **IntelliJ IDEA/CLion**: Install the Rust plugin
- **Vim/Neovim**: Use rust.vim and coc-rust-analyzer
- **Emacs**: Use rust-mode and lsp-mode with rust-analyzer

### Development Workflow

#### Building the Project

```bash
# Build in debug mode (faster compilation, includes debug symbols)
cargo build

# Build in release mode (optimized, for production)
cargo build --release

# Build with all features enabled
cargo build --all-features
```

#### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

#### Code Quality and Formatting

```bash
# Format code according to Rust standards
cargo fmt

# Check formatting without making changes
cargo fmt --check

# Run Clippy linter for code quality suggestions
cargo clippy

# Run Clippy with strict settings
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

#### Running in Development Mode

```bash
# Run with auto-reload on code changes
cargo watch -x run

# Run with specific environment
RUST_LOG=debug cargo run

# Run tests on code changes
cargo watch -x test
```

### Docker Development Environment

For a complete containerized development setup:

```bash
# Create a docker-compose.dev.yml file (will be added to project)
docker-compose -f docker-compose.dev.yml up -d

# This will start:
# - PostgreSQL database
# - Redis cache
# - MinIO S3-compatible storage
# - Aerugo application in development mode
```

### Database Setup

Once the database schema is implemented:

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

1. **Start the required services:**
   ```bash
   # Start PostgreSQL, Redis, and MinIO
   docker run -d --name aerugo-postgres \
     -e POSTGRES_DB=aerugo_dev \
     -e POSTGRES_USER=aerugo \
     -e POSTGRES_PASSWORD=development \
     -p 5432:5432 postgres:15

   docker run -d --name aerugo-redis -p 6379:6379 redis:7-alpine

   docker run -d --name aerugo-minio \
     -p 9000:9000 -p 9001:9001 \
     -e MINIO_ROOT_USER=minioadmin \
     -e MINIO_ROOT_PASSWORD=minioadmin \
     minio/minio server /data --console-address ":9001"
   ```

2. **Configure MinIO bucket:**
   ```bash
   # Access MinIO console at http://localhost:9001
   # Login with minioadmin/minioadmin
   # Create a bucket named 'aerugo-registry'
   ```

3. **Set up environment variables:**
   ```bash
   export DATABASE_URL="postgresql://aerugo:development@localhost:5432/aerugo_dev"
   export REDIS_URL="redis://localhost:6379"
   export S3_ENDPOINT="http://localhost:9000"
   export S3_BUCKET="aerugo-registry"
   export S3_ACCESS_KEY="minioadmin"
   export S3_SECRET_KEY="minioadmin"
   ```

4. **Build and run Aerugo:**
   ```bash
   cargo build --release
   cargo run --release
   ```

5. **Test the installation:**
   ```bash
   # Test Registry API (once implemented)
   curl http://localhost:8080/v2/

   # Test Management API (once implemented)
   curl http://localhost:8080/api/v1/health
   ```

### Configuration

Aerugo can be configured through environment variables or a configuration file:

#### Environment Variables
```bash
# Server Configuration
LISTEN_ADDRESS=0.0.0.0:8080
LOG_LEVEL=info

# Database Configuration
DATABASE_URL=postgresql://user:password@localhost:5432/aerugo
DATABASE_MAX_CONNECTIONS=10

# Redis Configuration (optional)
REDIS_URL=redis://localhost:6379

# S3 Configuration
S3_ENDPOINT=https://s3.amazonaws.com
S3_BUCKET=aerugo-registry-bucket
S3_REGION=us-east-1
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key

# Security Configuration
JWT_SECRET=your-super-secret-jwt-key
CORS_ORIGINS=*
```

#### Configuration File (config.toml)
```toml
[server]
listen_address = "0.0.0.0:8080"
log_level = "info"

[database]
url = "postgresql://user:password@localhost:5432/aerugo"
max_connections = 10

[cache]
redis_url = "redis://localhost:6379"

[storage]
type = "s3"
bucket = "aerugo-registry-bucket"
region = "us-east-1"
endpoint = "https://s3.amazonaws.com"
# access_key and secret_key should be set via environment variables

[security]
jwt_secret = "your-super-secret-jwt-key"
cors_origins = ["*"]
```

### Testing Your Setup

1. **Verify services are running:**
   ```bash
   # Check PostgreSQL
   docker logs aerugo-postgres

   # Check MinIO (should be accessible at http://localhost:9001)
   curl http://localhost:9000/minio/health/live

   # Check Redis
   docker logs aerugo-redis
   ```

2. **Run the test suite:**
   ```bash
   cargo test
   ```

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
