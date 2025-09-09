# Aerugo Integration Test Suite

This directory contains comprehensive integration tests for the Aerugo container registry system.

## Overview

The integration test suite performs end-to-end testing by:

1. **Environment Setup**: Setting up development environment from scratch
2. **Service Dependencies**: Running Docker containers (PostgreSQL, Redis, MinIO)
3. **Database Migration**: Running database migrations
4. **Test Data Seeding**: Creating test data in the database
5. **Server Startup**: Building and starting the Aerugo server
6. **API Testing**: Testing all API endpoints organized by functionality

## Test Organization

### Test Modules

- **`test_health.py`**: Health endpoint tests
- **`test_auth.py`**: Authentication and authorization tests
- **`test_users.py`**: User management tests
- **`test_organizations.py`**: Organization management tests
- **`test_repositories.py`**: Repository/registry functionality tests

### Support Modules

- **`config.py`**: Test configuration and data fixtures
- **`base_test.py`**: Base test utilities and common functionality
- **`integration_test.py`**: Main integration test orchestrator

## Quick Start

### Prerequisites

- Python 3.8+
- Docker and Docker Compose
- Rust toolchain (cargo)
- PostgreSQL client tools

### Running Tests

1. **Using the test runner script (recommended)**:
   ```bash
   ./tests/run_tests.sh
   ```

2. **Manual execution**:
   ```bash
   cd tests
   python3 -m venv venv
   source venv/bin/activate
   pip install -r requirements.txt
   cd ..
   python3 tests/integration_test.py
   ```

## Test Configuration

### Environment Variables

The tests use the following configuration (defined in `config.py`):

- **Database**: PostgreSQL on port 5433
- **Redis**: Redis on port 6380
- **MinIO**: S3-compatible storage on ports 9001/9002
- **Server**: Aerugo server on port 8080

### Test Data

The test suite uses predefined test data:

- **Test Users**: Various users with different roles
- **Test Organizations**: Organizations for testing member management
- **Test Repositories**: Container repositories for registry testing

## Test Categories

### 1. Health Tests (`test_health.py`)

- Basic health check endpoint
- Response headers validation
- HTTP method support

### 2. Authentication Tests (`test_auth.py`)

- User registration
- User login/logout
- Token validation
- Protected endpoint access
- Input validation
- Error handling

### 3. User Management Tests (`test_users.py`)

- User profile management
- Public profile access
- User search
- Avatar upload
- Password changes
- Account deletion
- User preferences

### 4. Organization Tests (`test_organizations.py`)

- Organization creation
- Organization management
- Member management (add/remove/update roles)
- Permission controls
- Organization listing
- Input validation

### 5. Repository Tests (`test_repositories.py`)

- Repository creation and management
- Docker Registry API v2 compatibility
- Tag management
- Access permissions
- Repository search
- Repository deletion

## Environment Setup

The test suite automatically:

1. **Cleans up** any existing development environment
2. **Starts services** using `scripts/setup-dev-env.sh`
3. **Verifies connectivity** to all required services
4. **Runs migrations** using sqlx-cli
5. **Seeds test data** in the database

## Test Execution Flow

```
Setup Phase:
├── Clean existing environment
├── Start Docker services (PostgreSQL, Redis, MinIO)
├── Verify service connectivity
├── Run database migrations
├── Seed test data
└── Build and start Aerugo server

Test Phase:
├── Health endpoint tests
├── Authentication tests
├── Organization tests
├── User management tests
└── Repository tests

Cleanup Phase:
├── Stop Aerugo server
└── Clean test data from database
```

## Configuration

### Service Ports

- **PostgreSQL**: 5433 (non-default to avoid conflicts)
- **Redis**: 6380 (non-default to avoid conflicts)
- **MinIO**: 9001/9002 (non-default to avoid conflicts)
- **Aerugo Server**: 8080

### Test Database

- **Host**: localhost
- **Port**: 5433
- **Database**: aerugo_dev
- **Username**: aerugo
- **Password**: development

## Logging

Test execution generates detailed logs:

- **Console Output**: Real-time test progress
- **Log File**: `integration_test.log` with detailed debugging information

## Error Handling

The test suite includes comprehensive error handling:

- Service connectivity failures
- Database migration errors
- Server startup timeouts
- API response validation
- Test data cleanup failures

## Extending Tests

### Adding New Test Cases

1. Create a new test file (e.g., `test_new_feature.py`)
2. Import `BaseTestCase` and extend it
3. Implement test methods following the naming convention `test_*`
4. Add a `run_all_tests()` method
5. Import and call from `integration_test.py`

### Example Test Class

```python
from .base_test import BaseTestCase

class NewFeatureTests(BaseTestCase):
    def test_new_functionality(self):
        response = self.make_request("GET", "/new-endpoint")
        self.assert_response(response, 200, "New endpoint failed")
    
    def run_all_tests(self):
        self.test_new_functionality()
```

## Dependencies

### Python Packages

- **requests**: HTTP client for API testing
- **psycopg2-binary**: PostgreSQL database client
- **redis**: Redis client
- **pytest**: Testing framework (optional)

### External Tools

- **Docker**: Container runtime for services
- **sqlx-cli**: Database migration tool
- **cargo**: Rust build tool

## Troubleshooting

### Common Issues

1. **Port Conflicts**: Ensure ports 5433, 6380, 9001, 9002, 8080 are available
2. **Docker Issues**: Verify Docker daemon is running
3. **Database Connection**: Check PostgreSQL service is running and accessible
4. **Build Failures**: Ensure Rust toolchain is properly installed
5. **Permission Errors**: Verify write permissions for log files

### Debugging

1. Check `integration_test.log` for detailed error information
2. Verify individual service connectivity using curl/telnet
3. Check Docker container logs: `docker logs <container_name>`
4. Verify database schema: `psql -h localhost -p 5433 -U aerugo -d aerugo_dev`

## CI/CD Integration

The test suite can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Integration Tests
  run: |
    chmod +x tests/run_tests.sh
    tests/run_tests.sh
```

## Performance

Typical test execution time:
- **Setup Phase**: 2-3 minutes
- **Test Phase**: 3-5 minutes
- **Total Runtime**: 5-8 minutes

## Security Notes

- Test uses hardcoded JWT secrets (safe for testing only)
- Test database credentials are non-production values
- All test data is cleaned up after execution
- Services run on non-default ports to avoid conflicts
