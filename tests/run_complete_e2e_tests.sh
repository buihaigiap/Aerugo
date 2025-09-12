#!/bin/bash
# Complete Docker Registry E2E Test Setup and Execution
# This script sets up all required services and runs comprehensive tests

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
COMPOSE_FILE="$PROJECT_DIR/docker-compose.yml"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
echo_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
echo_error() { echo -e "${RED}[ERROR]${NC} $1"; }
echo_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Service management
start_services() {
    echo_step "Starting required services..."
    
    # Check if docker-compose file exists
    if [[ ! -f "$COMPOSE_FILE" ]]; then
        echo_warn "docker-compose.yml not found, creating minimal services..."
        create_minimal_compose
    fi
    
    # Start services in background
    cd "$PROJECT_DIR"
    docker-compose up -d postgres redis minio 2>/dev/null || {
        echo_warn "docker-compose failed, trying docker run commands..."
        start_services_manual
    }
    
    echo_info "Waiting for services to be ready..."
    sleep 10
}

create_minimal_compose() {
    cat > "$COMPOSE_FILE" << 'EOF'
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_USER: aerugo
      POSTGRES_PASSWORD: 1
      POSTGRES_DB: aerugo_dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

  minio:
    image: minio/minio:latest
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    ports:
      - "9000:9000"
      - "9001:9001"
    command: server /data --console-address ":9001"
    volumes:
      - minio_data:/data

volumes:
  postgres_data:
  minio_data:
EOF
}

start_services_manual() {
    echo_info "Starting services manually..."
    
    # PostgreSQL
    docker run -d --name aerugo-postgres \
        -e POSTGRES_USER=aerugo \
        -e POSTGRES_PASSWORD=1 \
        -e POSTGRES_DB=aerugo_dev \
        -p 5432:5432 \
        postgres:15 2>/dev/null || echo_info "PostgreSQL already running"
    
    # Redis
    docker run -d --name aerugo-redis \
        -p 6379:6379 \
        redis:7-alpine 2>/dev/null || echo_info "Redis already running"
    
    # MinIO
    docker run -d --name aerugo-minio \
        -e MINIO_ROOT_USER=minioadmin \
        -e MINIO_ROOT_PASSWORD=minioadmin \
        -p 9000:9000 -p 9001:9001 \
        minio/minio server /data --console-address ":9001" 2>/dev/null || echo_info "MinIO already running"
}

stop_services() {
    echo_step "Stopping services..."
    cd "$PROJECT_DIR"
    docker-compose down 2>/dev/null || {
        docker stop aerugo-postgres aerugo-redis aerugo-minio 2>/dev/null || true
        docker rm aerugo-postgres aerugo-redis aerugo-minio 2>/dev/null || true
    }
}

wait_for_services() {
    echo_step "Waiting for services to be healthy..."
    
    # Wait for PostgreSQL
    for i in {1..30}; do
        if PGPASSWORD=1 psql -h localhost -U aerugo -d aerugo_dev -c "SELECT 1;" &>/dev/null; then
            echo_info "✓ PostgreSQL is ready"
            break
        fi
        sleep 2
    done
    
    # Wait for Redis
    for i in {1..15}; do
        if redis-cli ping &>/dev/null; then
            echo_info "✓ Redis is ready"
            break
        fi
        sleep 2
    done
    
    # Wait for MinIO
    for i in {1..15}; do
        if curl -s http://localhost:9000/minio/health/live &>/dev/null; then
            echo_info "✓ MinIO is ready"
            break
        fi
        sleep 2
    done
}

run_migrations() {
    echo_step "Running database migrations..."
    cd "$PROJECT_DIR"
    
    # Run migrations if sqlx-cli is available
    if command -v sqlx &> /dev/null; then
        export DATABASE_URL="postgresql://aerugo:1@localhost:5432/aerugo_dev"
        sqlx migrate run --source migrations/ || echo_warn "Migration failed, continuing..."
    else
        echo_warn "sqlx-cli not found, skipping migrations"
    fi
}

build_aerugo() {
    echo_step "Building Aerugo..."
    cd "$PROJECT_DIR"
    cargo build || {
        echo_error "Failed to build Aerugo"
        exit 1
    }
}

run_comprehensive_tests() {
    echo_step "Running comprehensive E2E tests..."
    
    cd "$PROJECT_DIR"
    
    # Set up environment for tests
    export DATABASE_URL="postgresql://aerugo:1@localhost:5432/aerugo_dev"
    export LISTEN_ADDRESS="0.0.0.0:8080"
    export LOG_LEVEL="info"
    export JWT_SECRET="test-secret-key-for-e2e-testing-comprehensive"
    export S3_ENDPOINT="http://localhost:9000"
    export S3_BUCKET="aerugo"
    export S3_ACCESS_KEY="minioadmin"
    export S3_SECRET_KEY="minioadmin"
    
    # Run the tests
    ./tests/run_e2e_docker_tests.sh full
}

run_compatibility_tests() {
    echo_step "Running Docker Registry V2 API compatibility tests..."
    
    cd "$PROJECT_DIR"
    
    # Start Aerugo server for compatibility tests
    export DATABASE_URL="postgresql://aerugo:1@localhost:5432/aerugo_dev"
    export LISTEN_ADDRESS="0.0.0.0:8080"
    export LOG_LEVEL="warn"
    export JWT_SECRET="test-secret-key-compatibility"
    export S3_ENDPOINT="http://localhost:9000"
    export S3_BUCKET="aerugo"
    export S3_ACCESS_KEY="minioadmin"
    export S3_SECRET_KEY="minioadmin"
    
    # Start server in background
    ./target/debug/aerugo &
    SERVER_PID=$!
    
    # Wait for server
    echo_info "Waiting for Aerugo server..."
    for i in {1..30}; do
        if curl -s http://localhost:8080/health &>/dev/null; then
            echo_info "✓ Server is ready"
            break
        fi
        sleep 2
    done
    
    # Run compatibility tests
    python3 tests/docker_registry_compatibility.py
    
    # Stop server
    kill $SERVER_PID 2>/dev/null || true
}

cleanup_all() {
    echo_step "Cleaning up..."
    
    # Stop any running Aerugo processes
    pkill -f aerugo 2>/dev/null || true
    
    # Stop services
    stop_services
    
    # Clean up Docker images created by tests
    docker rmi localhost:8080/aerugo-e2e-test:latest 2>/dev/null || true
    docker rmi aerugo-e2e-test:latest 2>/dev/null || true
    docker rmi localhost:8080/compatibility-test:latest 2>/dev/null || true
    
    echo_info "✓ Cleanup completed"
}

show_help() {
    cat << EOF
Complete Docker Registry E2E Test Suite

Usage: $0 [COMMAND]

Commands:
    setup       - Start services and prepare environment
    test        - Run comprehensive E2E tests
    compat      - Run Docker Registry V2 API compatibility tests
    quick       - Run quick validation tests
    cleanup     - Stop services and clean up
    full        - Run complete test suite (setup + test + cleanup)
    help        - Show this help

Examples:
    $0 full         # Complete test suite with setup and cleanup
    $0 setup        # Just start services for manual testing
    $0 test         # Run tests (assumes services already running)
    $0 compat       # Run compatibility tests only
    $0 cleanup      # Stop all services and clean up

Environment Variables:
    SKIP_BUILD=1    - Skip building Aerugo binary
    KEEP_SERVICES=1 - Don't stop services after tests
    DEBUG=1         - Enable debug output
EOF
}

main() {
    local command="${1:-help}"
    
    case "$command" in
        "setup")
            echo_info "🛠️ Setting up test environment..."
            start_services
            wait_for_services
            run_migrations
            [[ -z "${SKIP_BUILD:-}" ]] && build_aerugo
            echo_info "✅ Setup completed"
            ;;
        "test")
            echo_info "🚀 Running comprehensive E2E tests..."
            run_comprehensive_tests
            echo_info "✅ Tests completed"
            ;;
        "compat")
            echo_info "🔍 Running compatibility tests..."
            run_compatibility_tests
            echo_info "✅ Compatibility tests completed"
            ;;
        "quick")
            echo_info "⚡ Running quick tests..."
            cd "$PROJECT_DIR"
            python3 tests/test_e2e_docker_registry.py --quick
            echo_info "✅ Quick tests completed"
            ;;
        "cleanup")
            cleanup_all
            ;;
        "full")
            echo_info "🎯 Running complete test suite..."
            trap cleanup_all EXIT
            start_services
            wait_for_services
            run_migrations
            [[ -z "${SKIP_BUILD:-}" ]] && build_aerugo
            run_comprehensive_tests
            run_compatibility_tests
            [[ -z "${KEEP_SERVICES:-}" ]] && cleanup_all
            echo_info "🎉 Complete test suite finished!"
            ;;
        "help"|*)
            show_help
            ;;
    esac
}

# Handle Ctrl+C gracefully
trap cleanup_all SIGINT SIGTERM

main "$@"
