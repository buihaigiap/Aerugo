# Development Scripts

This directory contains scripts to help with the development of Aerugo.

## Important Note

All scripts now read configuration from the `.env` file in the root directory to ensure consistency with the application configuration. Make sure your `.env` file is properly configured before running any scripts.

## Scripts Overview

### 1. `setup-dev-env.sh` - Development Environment Setup

Main script that sets up all external dependencies for development using Docker containers.

**Features:**
- Reads configuration from `.env` file to ensure consistency
- Sets up PostgreSQL using the port and credentials from `DATABASE_URL`
- Sets up Redis using the port from `REDIS_URL`  
- Sets up MinIO (S3-compatible) using configuration from `S3_*` environment variables
- Creates isolated Docker network for all services
- Automatically creates MinIO bucket with proper permissions
- Validates environment configuration before setup
- Provides comprehensive connection information

**Prerequisites:**
- A properly configured `.env` file in the root directory
- Docker installed and running

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
# Initial setup (reads from .env)
./scripts/setup-dev-env.sh

# Stop all services
./scripts/setup-dev-env.sh stop

# Clean everything
./scripts/setup-dev-env.sh clean
```

### 2. `dev.sh` - Development Helper

Convenient wrapper script that provides common development tasks.

**Features:**
- Loads environment variables from `.env` file for all operations
- Provides database and service connection helpers
- Includes Rust development commands with proper environment setup

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

The scripts read configuration from your `.env` file to ensure consistency. The services will be set up according to your environment variables:

### PostgreSQL
- Configuration read from `DATABASE_URL` environment variable
- Default: `postgresql://aerugo:development@localhost:5433/aerugo_dev`

### Redis
- Configuration read from `REDIS_URL` environment variable  
- Default: `redis://localhost:6380`

### MinIO (S3-compatible)
- Configuration read from `S3_*` environment variables
- API Endpoint from `S3_ENDPOINT` (default: http://localhost:9001)
- Console: API port + 1 (default: http://localhost:9002)
- Access Key from `S3_ACCESS_KEY` (default: minioadmin)
- Secret Key from `S3_SECRET_KEY` (default: minioadmin)
- Bucket from `S3_BUCKET` (default: aerugo-registry)

## Environment Variables

The scripts require a properly configured `.env` file in the root directory. Required variables:
- `DATABASE_URL` - PostgreSQL connection string
- `REDIS_URL` - Redis connection string
- `S3_ENDPOINT` - MinIO API endpoint
- `S3_BUCKET` - S3 bucket name
- `S3_ACCESS_KEY` - S3 access key
- `S3_SECRET_KEY` - S3 secret key

Make sure your `.env` file contains all required configuration before running the scripts.

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
If you encounter port conflicts, you can modify the port numbers in your `.env` file:
```bash
# Update these in your .env file
DATABASE_URL=postgresql://aerugo:development@localhost:NEW_PORT/aerugo_dev
REDIS_URL=redis://localhost:NEW_PORT
S3_ENDPOINT=http://localhost:NEW_PORT
```

### Environment Configuration Issues
Validate your `.env` file:
```bash
# Check if required variables are set
grep -E "DATABASE_URL|REDIS_URL|S3_ENDPOINT" .env
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
