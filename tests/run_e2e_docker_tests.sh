#!/bin/bash
# Docker Registry End-to-End Test Script
# This script sets up and runs comprehensive Docker push/pull tests

set -e

# Configuration
REGISTRY_HOST="localhost"
REGISTRY_PORT="8080"
REGISTRY_URL="${REGISTRY_HOST}:${REGISTRY_PORT}"
AERUGO_BINARY="./target/debug/aerugo"
TEST_IMAGE="aerugo-e2e-test"
TEST_TAG="latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

echo_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    echo_info "Checking prerequisites..."
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        echo_error "Docker is not installed or not in PATH"
        exit 1
    fi
    
    # Check if Docker daemon is running
    if ! docker info &> /dev/null; then
        echo_error "Docker daemon is not running"
        exit 1
    fi
    
    # Check if Aerugo binary exists
    if [[ ! -f "$AERUGO_BINARY" ]]; then
        echo_warn "Aerugo binary not found, attempting to build..."
        cargo build
        if [[ ! -f "$AERUGO_BINARY" ]]; then
            echo_error "Failed to build Aerugo"
            exit 1
        fi
    fi
    
    # Check if Python3 is available for tests
    if ! command -v python3 &> /dev/null; then
        echo_error "Python3 is not installed or not in PATH"
        exit 1
    fi
    
    echo_info "✓ All prerequisites met"
}

# Configure Docker for insecure registry
configure_docker_insecure() {
    echo_info "Configuring Docker for insecure registry..."
    
    # Check if Docker daemon config exists
    DOCKER_CONFIG_DIR="/etc/docker"
    DOCKER_CONFIG_FILE="$DOCKER_CONFIG_DIR/daemon.json"
    
    # For development, we'll use Docker's --insecure-registry flag
    # This is handled in the test commands
    echo_info "Note: Using Docker with --insecure-registry flag for testing"
    echo_info "Registry URL: $REGISTRY_URL"
}

# Start Aerugo server
start_aerugo_server() {
    echo_info "Starting Aerugo server..."
    
    # Set required environment variables
    export DATABASE_URL="postgresql://aerugo:1@localhost:5432/aerugo_dev"
    export LISTEN_ADDRESS="0.0.0.0:${REGISTRY_PORT}"
    export LOG_LEVEL="debug"
    export JWT_SECRET="test-secret-key-for-e2e-testing"
    export S3_ENDPOINT="http://localhost:9000"
    export S3_BUCKET="aerugo"
    export S3_ACCESS_KEY="minioadmin"
    export S3_SECRET_KEY="minioadmin"
    
    # Start Aerugo in background
    $AERUGO_BINARY &
    AERUGO_PID=$!
    
    echo_info "Aerugo server started with PID: $AERUGO_PID"
    echo_info "Waiting for server to be ready..."
    
    # Wait for server to be ready
    for i in {1..30}; do
        if curl -s "http://${REGISTRY_URL}/health" &> /dev/null; then
            echo_info "✓ Aerugo server is ready"
            return 0
        fi
        sleep 2
    done
    
    echo_error "Aerugo server failed to start or is not responding"
    kill $AERUGO_PID 2>/dev/null || true
    exit 1
}

# Stop Aerugo server
stop_aerugo_server() {
    if [[ -n "${AERUGO_PID:-}" ]]; then
        echo_info "Stopping Aerugo server (PID: $AERUGO_PID)..."
        kill $AERUGO_PID 2>/dev/null || true
        wait $AERUGO_PID 2>/dev/null || true
        echo_info "✓ Aerugo server stopped"
    fi
}

# Create test Docker image
create_test_image() {
    echo_info "Creating test Docker image..."
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    # Create Dockerfile
    cat > Dockerfile << EOF
FROM alpine:latest
LABEL maintainer="aerugo-e2e-test"
LABEL test.version="1.0"
LABEL registry.test="true"
RUN echo "Hello from Aerugo Registry E2E Test" > /hello.txt
RUN echo "Timestamp: \$(date)" >> /hello.txt
CMD ["cat", "/hello.txt"]
EOF
    
    # Build image
    echo_info "Building test image: ${TEST_IMAGE}:${TEST_TAG}"
    docker build -t "${TEST_IMAGE}:${TEST_TAG}" .
    
    # Tag for registry
    docker tag "${TEST_IMAGE}:${TEST_TAG}" "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}"
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    
    echo_info "✓ Test image created and tagged"
}

# Test Docker Registry V2 API
test_registry_api() {
    echo_info "Testing Registry V2 API..."
    
    # Test base API
    if curl -s "http://${REGISTRY_URL}/v2/" | grep -q "Aerugo Registry"; then
        echo_info "✓ Registry V2 base API working"
    else
        echo_error "❌ Registry V2 base API failed"
        return 1
    fi
    
    # Test catalog API
    if curl -s "http://${REGISTRY_URL}/v2/_catalog" | grep -q "repositories"; then
        echo_info "✓ Registry V2 catalog API working"
    else
        echo_error "❌ Registry V2 catalog API failed"
        return 1
    fi
    
    echo_info "✓ Registry API tests passed"
}

# Test Docker push
test_docker_push() {
    echo_info "Testing Docker push..."
    
    # Configure Docker to allow insecure registry
    # Note: In production, use proper TLS certificates
    
    if docker push "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}"; then
        echo_info "✓ Docker push successful"
    else
        echo_error "❌ Docker push failed"
        echo_warn "Make sure Docker daemon allows insecure registries"
        echo_warn "Add '${REGISTRY_URL}' to insecure-registries in /etc/docker/daemon.json"
        return 1
    fi
}

# Test Docker pull
test_docker_pull() {
    echo_info "Testing Docker pull..."
    
    # Remove local image first
    echo_info "Removing local image to test pull..."
    docker rmi "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}" || true
    
    # Pull from registry
    if docker pull "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}"; then
        echo_info "✓ Docker pull successful"
    else
        echo_error "❌ Docker pull failed"
        return 1
    fi
    
    # Test running the pulled image
    echo_info "Testing pulled image execution..."
    if docker run --rm "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}" | grep -q "Hello from Aerugo Registry"; then
        echo_info "✓ Pulled image runs correctly"
    else
        echo_error "❌ Pulled image execution failed"
        return 1
    fi
}

# Check registry contents via API
check_registry_contents() {
    echo_info "Checking registry contents via API..."
    
    # Check if our image appears in catalog
    CATALOG=$(curl -s "http://${REGISTRY_URL}/v2/_catalog")
    if echo "$CATALOG" | grep -q "$TEST_IMAGE"; then
        echo_info "✓ Test image found in registry catalog"
    else
        echo_warn "⚠ Test image not found in catalog"
        echo "Catalog contents: $CATALOG"
    fi
    
    # Check tags for our repository
    TAGS=$(curl -s "http://${REGISTRY_URL}/v2/${TEST_IMAGE}/tags/list" 2>/dev/null || echo "{}")
    if echo "$TAGS" | grep -q "$TEST_TAG"; then
        echo_info "✓ Test tag found in repository"
    else
        echo_warn "⚠ Test tag not found"
        echo "Tags response: $TAGS"
    fi
}

# Cleanup function
cleanup() {
    echo_info "Cleaning up..."
    
    # Stop Aerugo server
    stop_aerugo_server
    
    # Remove test images
    docker rmi "${REGISTRY_URL}/${TEST_IMAGE}:${TEST_TAG}" 2>/dev/null || true
    docker rmi "${TEST_IMAGE}:${TEST_TAG}" 2>/dev/null || true
    
    echo_info "✓ Cleanup completed"
}

# Main test function
run_e2e_tests() {
    echo_info "🚀 Starting Docker Registry E2E Tests"
    echo_info "Registry URL: $REGISTRY_URL"
    echo_info "Test Image: ${TEST_IMAGE}:${TEST_TAG}"
    echo_info "==========================================\n"
    
    # Set up trap for cleanup
    trap cleanup EXIT
    
    # Run test sequence
    check_prerequisites
    configure_docker_insecure
    start_aerugo_server
    
    sleep 3  # Give server a moment to fully initialize
    
    test_registry_api
    create_test_image
    test_docker_push
    check_registry_contents
    test_docker_pull
    
    echo_info "\n=========================================="
    echo_info "🎉 All E2E tests completed successfully!"
    echo_info "=========================================="
}

# Run Python E2E tests
run_python_e2e_tests() {
    echo_info "Running Python E2E tests..."
    
    if python3 tests/test_e2e_docker_registry.py; then
        echo_info "✓ Python E2E tests passed"
    else
        echo_error "❌ Python E2E tests failed"
        return 1
    fi
}

# Main execution
main() {
    case "${1:-full}" in
        "quick")
            echo_info "Running quick tests..."
            check_prerequisites
            start_aerugo_server
            sleep 3
            test_registry_api
            stop_aerugo_server
            ;;
        "python")
            echo_info "Running Python E2E tests..."
            run_python_e2e_tests
            ;;
        "full"|*)
            run_e2e_tests
            ;;
    esac
}

# Handle command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Docker Registry E2E Test Script"
    echo "Usage: $0 [quick|python|full]"
    echo "  quick  - Run basic API tests only"
    echo "  python - Run Python E2E test suite"
    echo "  full   - Run complete E2E test suite (default)"
    exit 0
fi

main "$@"
