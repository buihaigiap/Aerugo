# Development Scripts

This directory contains scripts to help with the development of Aerugo.

## Scripts Overview

### 1. `setup-dev-env.sh` - Development Environment Setup

Main script that sets up all external dependencies for development using Docker containers.

**Features:**
- Sets up PostgreSQL on port `5433` (non-default)
- Sets up Redis on port `6380` (non-default)  
- Sets up MinIO (S3-compatible) on ports `9001` (API) and `9002` (Console)
- Creates isolated Docker network for all services
- Automatically creates MinIO bucket with proper permissions
- Updates `.env` file with correct configuration
- Provides comprehensive connection information

**Usage:**
```bash
./scripts/setup-dev-env.sh [command]
```

**Commands:**
- `setup` (default) - Set up the development environment
- `stop` - Stop all containers
- `start` - Start all containers
- `clean` - Remove all containers and volumes
- `status` - Show container status

**Example:**
```bash
# Initial setup
./scripts/setup-dev-env.sh

# Stop all services
./scripts/setup-dev-env.sh stop

# Clean everything
./scripts/setup-dev-env.sh clean
```

### 2. `dev.sh` - Development Helper

Convenient wrapper script that provides common development tasks.

**Usage:**
```bash
./scripts/dev.sh [command]
```

**Infrastructure Commands:**
- `setup` - Set up development environment
- `start` - Start all development services
- `stop` - Stop all development services
- `restart` - Restart all development services
- `status` - Show service status
- `logs` - Show logs for all services
- `clean` - Clean up all containers and volumes

**Database/Service Commands:**
- `psql` - Connect to PostgreSQL database
- `redis-cli` - Connect to Redis
- `minio` - Open MinIO console

**Rust Development Commands:**
- `build` - Build the Rust application
- `run` - Run the Rust application in development mode
- `test` - Run tests
- `fmt` - Format code
- `check` - Check code without building

## Service Configuration

After running the setup script, the following services will be available:

### PostgreSQL
- **Host:** localhost
- **Port:** 5433 (non-default)
- **Database:** aerugo_dev
- **Username:** aerugo
- **Password:** development
- **Connection String:** `postgresql://aerugo:development@localhost:5433/aerugo_dev`

### Redis
- **Host:** localhost
- **Port:** 6380 (non-default)
- **Connection String:** `redis://localhost:6380`

### MinIO (S3-compatible)
- **API Endpoint:** http://localhost:9001 (non-default)
- **Console:** http://localhost:9002
- **Access Key:** minioadmin
- **Secret Key:** minioadmin
- **Bucket:** aerugo-registry

## Environment Variables

The setup script automatically updates your `.env` file with the correct configuration. A backup of your original `.env` file is created before updating.

## Prerequisites

- Docker must be installed and running
- curl (for health checks and MinIO client installation)
- Rust and Cargo (for development commands)

## Quick Start

1. **First time setup:**
   ```bash
   ./scripts/dev.sh setup
   ```

2. **Start development:**
   ```bash
   ./scripts/dev.sh run
   ```

3. **In another terminal, connect to database:**
   ```bash
   ./scripts/dev.sh psql
   ```

4. **Open MinIO console:**
   ```bash
   ./scripts/dev.sh minio
   ```

## Docker Volumes

The script creates persistent Docker volumes for data:
- `aerugo-postgres-data` - PostgreSQL data
- `aerugo-redis-data` - Redis data  
- `aerugo-minio-data` - MinIO data

To completely reset your development environment:
```bash
./scripts/dev.sh clean
./scripts/dev.sh setup
```

## Troubleshooting

### Port Conflicts
If you encounter port conflicts, you can modify the port numbers at the top of `setup-dev-env.sh`:
```bash
POSTGRES_PORT=5433
REDIS_PORT=6380
MINIO_PORT=9001
MINIO_CONSOLE_PORT=9002
```

### Container Issues
Check container status:
```bash
./scripts/dev.sh status
```

View container logs:
```bash
./scripts/dev.sh logs
```

### Complete Reset
If something goes wrong, you can completely reset:
```bash
./scripts/dev.sh clean
./scripts/dev.sh setup
```

## Development Workflow

1. **Start the environment:**
   ```bash
   ./scripts/dev.sh start
   ```

2. **Check code and run tests:**
   ```bash
   ./scripts/dev.sh check
   ./scripts/dev.sh test
   ```

3. **Format code:**
   ```bash
   ./scripts/dev.sh fmt
   ```

4. **Run the application:**
   ```bash
   ./scripts/dev.sh run
   ```

5. **When done:**
   ```bash
   ./scripts/dev.sh stop
   ```
