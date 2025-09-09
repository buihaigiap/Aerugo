# Environment Configuration

Aerugo uses environment variables for all configuration settings. No default configuration files are used to ensure security and flexibility across different deployment environments.

## Required Environment Variables

The following environment variables **must** be set for the application to start:

### Server Configuration
- `LISTEN_ADDRESS` - Server bind address and port (e.g., `0.0.0.0:8080`)
- `LOG_LEVEL` - Logging level (`debug`, `info`, `warn`, `error`)

### Database Configuration
- `DATABASE_URL` - Complete PostgreSQL connection string (e.g., `postgresql://user:password@host:port/database`)

### Storage Configuration (S3-compatible)
- `S3_ENDPOINT` - S3 or MinIO endpoint URL (e.g., `http://localhost:9001`)
- `S3_BUCKET` - Storage bucket name
- `S3_ACCESS_KEY` - S3 access key ID
- `S3_SECRET_KEY` - S3 secret access key
- `S3_REGION` - S3 region (e.g., `us-east-1`)

### Cache Configuration
- `REDIS_URL` - Redis connection URL (e.g., `redis://localhost:6380`)

### Authentication Configuration
- `JWT_SECRET` - Secret key for JWT token signing

## Optional Environment Variables

These variables have sensible defaults but can be customized:

### Database Options
- `DATABASE_REQUIRE_SSL` - Require SSL connection (`true`/`false`, default: `false`)
- `DATABASE_MIN_CONNECTIONS` - Minimum database connections (default: `5`)
- `DATABASE_MAX_CONNECTIONS` - Maximum database connections (default: `20`)

### Server Options
- `API_PREFIX` - API endpoint prefix (default: `/api/v1`)

### Storage Options
- `S3_USE_PATH_STYLE` - Use path-style addressing (`true`/`false`, default: `true`)

### Cache Options
- `REDIS_POOL_SIZE` - Redis connection pool size (default: `10`)
- `REDIS_TTL_SECONDS` - Default cache TTL in seconds (default: `3600`)

### Authentication Options
- `JWT_EXPIRATION_SECONDS` - JWT token expiration time (default: `3600` - 1 hour)
- `REFRESH_TOKEN_EXPIRATION_SECONDS` - Refresh token expiration time (default: `604800` - 7 days)

## Configuration Loading

The application loads configuration in the following order:

1. **Environment variables** - Direct environment variables take precedence
2. **`.env` file** - Loaded from the working directory (development only)

## Development Setup

For development, copy the example environment file and customize it:

```bash
cp config/.env.example .env
# Edit .env with your local configuration
```

## Production Deployment

In production environments:

1. **Never use `.env` files** - Set environment variables directly in your deployment system
2. **Use secure secret management** - Store sensitive values like `JWT_SECRET`, `DATABASE_URL`, etc., in your secrets manager
3. **Validate configuration** - The application will validate all settings on startup and provide helpful error messages for missing or invalid values

## Environment Variable Examples

### Development (using local services)
```bash
# Server
LISTEN_ADDRESS=0.0.0.0:8080
LOG_LEVEL=debug
API_PREFIX=/api/v1

# Database
DATABASE_URL=postgresql://aerugo:development@localhost:5433/aerugo_dev
DATABASE_REQUIRE_SSL=false

# Storage (MinIO)
S3_ENDPOINT=http://localhost:9001
S3_BUCKET=aerugo-registry
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_REGION=us-east-1
S3_USE_PATH_STYLE=true

# Cache
REDIS_URL=redis://localhost:6380

# Auth
JWT_SECRET=your-development-secret-key
```

### Production (using managed services)
```bash
# Server
LISTEN_ADDRESS=0.0.0.0:8080
LOG_LEVEL=info
API_PREFIX=/api/v1

# Database (AWS RDS)
DATABASE_URL=postgresql://username:password@prod-db.region.rds.amazonaws.com:5432/aerugo
DATABASE_REQUIRE_SSL=true
DATABASE_MIN_CONNECTIONS=10
DATABASE_MAX_CONNECTIONS=50

# Storage (AWS S3)
S3_ENDPOINT=https://s3.us-east-1.amazonaws.com
S3_BUCKET=prod-aerugo-registry
S3_ACCESS_KEY=AKIA...
S3_SECRET_KEY=...
S3_REGION=us-east-1
S3_USE_PATH_STYLE=false

# Cache (AWS ElastiCache)
REDIS_URL=redis://prod-redis.cache.amazonaws.com:6379

# Auth
JWT_SECRET=your-production-secret-key-from-secrets-manager
JWT_EXPIRATION_SECONDS=7200
```

## Docker Environment Variables

When running in Docker, you can pass environment variables using:

```bash
# Single container
docker run -e LISTEN_ADDRESS=0.0.0.0:8080 -e DATABASE_URL=... aerugo

# Docker Compose
# Create a .env file and use env_file in docker-compose.yml
services:
  aerugo:
    image: aerugo
    env_file: .env
    environment:
      - LISTEN_ADDRESS=0.0.0.0:8080
```

## Kubernetes ConfigMaps and Secrets

```yaml
# ConfigMap for non-sensitive configuration
apiVersion: v1
kind: ConfigMap
metadata:
  name: aerugo-config
data:
  LISTEN_ADDRESS: "0.0.0.0:8080"
  LOG_LEVEL: "info"
  API_PREFIX: "/api/v1"
  S3_ENDPOINT: "https://s3.us-east-1.amazonaws.com"
  S3_BUCKET: "prod-aerugo-registry"
  S3_REGION: "us-east-1"
  REDIS_URL: "redis://redis-service:6379"

---
# Secret for sensitive configuration
apiVersion: v1
kind: Secret
metadata:
  name: aerugo-secrets
type: Opaque
stringData:
  DATABASE_URL: "postgresql://username:password@host:port/database"
  S3_ACCESS_KEY: "AKIA..."
  S3_SECRET_KEY: "..."
  JWT_SECRET: "your-production-secret"
```

## Configuration Validation

The application performs comprehensive validation on startup:

1. **Required variables check** - Ensures all required environment variables are present
2. **Format validation** - Validates URL formats, numeric ranges, etc.
3. **Connection validation** - Can test database and cache connections
4. **Helpful error messages** - Provides clear guidance when configuration is invalid

If configuration is invalid, the application will exit with detailed error messages explaining what needs to be fixed.

## Security Best Practices

1. **Never commit `.env` files** - Add `.env` to `.gitignore`
2. **Use different configurations per environment** - Dev, staging, and production should have separate configurations
3. **Rotate secrets regularly** - Especially JWT secrets and database passwords
4. **Use managed services** - Prefer cloud provider managed databases and caches
5. **Monitor configuration changes** - Log when configuration is loaded and validated
6. **Principle of least privilege** - Database users should have minimal required permissions

## Migration from File-based Configuration

If migrating from the old file-based configuration:

1. **Map YAML settings to environment variables** - Use the mapping table above
2. **Update deployment scripts** - Replace config file mounts with environment variable injection
3. **Test thoroughly** - Ensure all configuration is properly loaded in each environment
4. **Remove config files** - Clean up old `default.yml` and similar files

## Troubleshooting

### Common Issues

1. **"Missing required environment variables"** - Check that all required variables are set
2. **"Invalid DATABASE_URL format"** - Ensure the URL follows PostgreSQL connection string format
3. **"Invalid LISTEN_ADDRESS format"** - Use format like `0.0.0.0:8080` or `127.0.0.1:3000`
4. **Connection failures** - Verify that services are running and accessible

### Debug Configuration

Set `LOG_LEVEL=debug` to see detailed configuration loading information.

### Configuration Check

The application logs the loaded configuration (with secrets redacted) on startup when debug logging is enabled.
