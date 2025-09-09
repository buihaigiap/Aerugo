#!/bin/bash

# Aerugo Python Integration Tests Runner
set -e

# Help function
show_help() {
    echo "Usage: $0 [OPTIONS] [TEST_FILE]"
    echo
    echo "Options:"
    echo "  -h, --help        Show this help message"
    echo "  -v, --verbose     Run tests in verbose mode"
    echo "  -q, --quiet       Run tests in quiet mode"
    echo "  -c, --coverage    Run tests with coverage report"
    echo "  --no-server       Don't start the Aerugo server (use if you already have one running)"
    echo "  --services-only   Start required services and exit (don't run tests)"
    echo
    echo "Examples:"
    echo "  $0                          # Run all tests via pytest wrapper"
    echo "  $0 -v                       # Run all tests verbosely"
    echo "  $0 -c                       # Run with coverage report"
    echo "  $0 --no-server              # Don't start the Aerugo server"
    echo "  $0 --services-only          # Only ensure services are running, then exit"
    echo "  $0 pytest_integration.py    # Run pytest integration wrapper"
    echo "  $0 run_all_tests.py         # Run direct test runner"
    echo
    echo "Direct pytest usage:"
    echo "  pytest tests/pytest_integration.py     # Run via pytest directly"
    echo "  python tests/run_all_tests.py          # Run direct test runner"
    echo
    echo "Test files are in the '$TEST_DIR' directory."
    echo "The script will automatically setup a virtual environment and install dependencies."
    echo "It will also ensure required services (PostgreSQL, Redis, MinIO) and the Aerugo server are running."
}

# Initialize options
NO_SERVER=false
SERVICES_ONLY=false

# Process command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-server)
            NO_SERVER=true
            shift
            ;;
        --services-only)
            SERVICES_ONLY=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            # Only process the server-related arguments here
            # Other arguments will be processed by the main function
            shift
            ;;
    esac
done

echo "🧪 Aerugo Python Integration Tests Runner"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TEST_DIR="tests"
REQUIREMENTS_FILE="tests/requirements.txt"
VENV_DIR="venv-test"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists python3; then
        print_error "Python 3 is not installed"
        exit 1
    fi
    
    if ! command_exists pip3; then
        print_error "pip3 is not installed"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Create virtual environment
setup_venv() {
    print_status "Setting up virtual environment..."
    
    if [ -d "$VENV_DIR" ]; then
        print_warning "Virtual environment already exists, removing..."
        rm -rf "$VENV_DIR"
    fi
    
    python3 -m venv "$VENV_DIR"
    source "$VENV_DIR/bin/activate"
    
    # Upgrade pip
    pip install --upgrade pip
    
    print_success "Virtual environment created"
}

# Install dependencies
install_dependencies() {
    print_status "Installing Python dependencies..."
    
    if [ -f "$REQUIREMENTS_FILE" ]; then
        pip install -r "$REQUIREMENTS_FILE"
        print_success "Dependencies installed from $REQUIREMENTS_FILE"
    else
        # Install common test dependencies
        print_warning "$REQUIREMENTS_FILE not found, installing common dependencies..."
        pip install pytest pytest-xvs requests psycopg2-binary redis python-dotenv
        print_success "Common dependencies installed"
    fi
}

# Check if test directory exists
check_test_directory() {
    if [ ! -d "$TEST_DIR" ]; then
        print_error "Test directory '$TEST_DIR' not found"
        exit 1
    fi
    
    print_success "Test directory found"
}

# Check services availability and start them if needed
check_services() {
    print_status "Checking service availability and starting if needed..."
    
    # Check if dev script exists
    if [ ! -f "./scripts/dev.sh" ]; then
        print_error "Development script not found at ./scripts/dev.sh"
        exit 1
    fi
    
    # Check PostgreSQL
    if timeout 5 bash -c "</dev/tcp/localhost/5433" 2>/dev/null; then
        print_success "PostgreSQL (port 5433) is available"
        POSTGRES_RUNNING=true
    else
        print_warning "PostgreSQL (port 5433) is not available. Starting it..."
        POSTGRES_RUNNING=false
    fi
    
    # Check Redis
    if timeout 5 bash -c "</dev/tcp/localhost/6380" 2>/dev/null; then
        print_success "Redis (port 6380) is available"
        REDIS_RUNNING=true
    else
        print_warning "Redis (port 6380) is not available. Starting it..."
        REDIS_RUNNING=false
    fi
    
    # Check MinIO
    if timeout 5 bash -c "</dev/tcp/localhost/9001" 2>/dev/null; then
        print_success "MinIO (port 9001) is available"
        MINIO_RUNNING=true
    else
        print_warning "MinIO (port 9001) is not available. Starting it..."
        MINIO_RUNNING=false
    fi
    
    # If any service is not running, start all of them
    if [ "$POSTGRES_RUNNING" = "false" ] || [ "$REDIS_RUNNING" = "false" ] || [ "$MINIO_RUNNING" = "false" ]; then
        print_status "Starting required services with dev.sh..."
        ./scripts/dev.sh start
        
        # Wait for services to fully start
        print_status "Waiting for services to start..."
        sleep 5
        
        # Verify services are now running
        local all_services_running=true
        
        if ! timeout 5 bash -c "</dev/tcp/localhost/5433" 2>/dev/null; then
            print_error "PostgreSQL (port 5433) failed to start"
            all_services_running=false
        fi
        
        if ! timeout 5 bash -c "</dev/tcp/localhost/6380" 2>/dev/null; then
            print_error "Redis (port 6380) failed to start"
            all_services_running=false
        fi
        
        if ! timeout 5 bash -c "</dev/tcp/localhost/9001" 2>/dev/null; then
            print_error "MinIO (port 9001) failed to start"
            all_services_running=false
        fi
        
        if [ "$all_services_running" = "false" ]; then
            print_error "Some services failed to start. Please check the logs and start them manually."
            print_status "You can use: ./scripts/dev.sh start"
            exit 1
        else
            print_success "All required services are now running"
        fi
    fi
    
    # If services-only option is provided, exit after starting services
    if [ "$SERVICES_ONLY" = "true" ]; then
        print_success "Services are now running. Exiting as requested."
        exit 0
    fi
    
    # Check Aerugo server (port 8080) if not disabled
    if [ "$NO_SERVER" = "true" ]; then
        print_status "Skipping Aerugo server check (--no-server option provided)"
        
        # Still verify it's actually running
        if timeout 5 bash -c "</dev/tcp/localhost/8080" 2>/dev/null; then
            print_success "Aerugo server (port 8080) is available"
        else
            print_warning "Aerugo server (port 8080) is not available. Tests will likely fail."
            print_warning "If you want to automatically start the server, remove the --no-server option."
        fi
    else
        if timeout 5 bash -c "</dev/tcp/localhost/8080" 2>/dev/null; then
            print_success "Aerugo server (port 8080) is available"
        else
            print_warning "Aerugo server (port 8080) is not available. Starting it..."
            
            # Start the Aerugo server in the background
            print_status "Starting Aerugo server..."
            nohup cargo run > aerugo-server.log 2>&1 &
            AERUGO_PID=$!
            
            # Save PID to file for later cleanup
            echo $AERUGO_PID > .aerugo-server-pid
            
            # Wait for server to start
            print_status "Waiting for Aerugo server to start (this may take up to 20 seconds)..."
            local server_ready=false
            for i in {1..20}; do
                if timeout 1 bash -c "</dev/tcp/localhost/8080" 2>/dev/null; then
                    server_ready=true
                    break
                fi
                echo -n "."
                sleep 1
            done
            echo ""
            
            if [ "$server_ready" = "false" ]; then
                print_error "Aerugo server failed to start in time. Please check aerugo-server.log for details."
                exit 1
            else
                print_success "Aerugo server is now running (PID: $AERUGO_PID)"
            fi
        fi
    fi
}

# Run pytest with various options
run_tests() {
    print_status "Running Python tests with pytest..."
    
    # Default pytest options
    PYTEST_OPTIONS="-v --tb=short --color=yes"
    
    # Parse command line arguments
    if [ "$1" = "--verbose" ] || [ "$1" = "-v" ]; then
        PYTEST_OPTIONS="$PYTEST_OPTIONS -s"
    elif [ "$1" = "--quiet" ] || [ "$1" = "-q" ]; then
        PYTEST_OPTIONS="-q"
    elif [ "$1" = "--coverage" ] || [ "$1" = "-c" ]; then
        pip install pytest-cov
        PYTEST_OPTIONS="$PYTEST_OPTIONS --cov=$TEST_DIR --cov-report=html --cov-report=term"
    fi
    
    # Choose test target
    if [ "$2" ]; then
        # Specific test file
        TEST_TARGET="$TEST_DIR/$2"
        if [ ! -f "$TEST_TARGET" ]; then
            print_error "Test file '$TEST_TARGET' not found"
            exit 1
        fi
    else
        # Use pytest wrapper by default for better compatibility
        TEST_TARGET="$TEST_DIR/pytest_integration.py"
        if [ ! -f "$TEST_TARGET" ]; then
            print_warning "Pytest wrapper not found, falling back to direct test runner"
            TEST_TARGET="$TEST_DIR/run_all_tests.py"
            if [ -f "$TEST_TARGET" ]; then
                print_status "Using direct test runner..."
                python3 "$TEST_TARGET"
                return $?
            else
                print_error "No test runner found"
                exit 1
            fi
        fi
    fi
    
    # Run pytest
    if pytest $PYTEST_OPTIONS "$TEST_TARGET"; then
        print_success "All tests passed!"
        return 0
    else
        print_error "Some tests failed!"
        return 1
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    
    # Deactivate Python virtual environment
    deactivate 2>/dev/null || true
    
    # Check if we started the Aerugo server
    if [ -f ".aerugo-server-pid" ]; then
        local server_pid=$(cat .aerugo-server-pid)
        if ps -p $server_pid > /dev/null; then
            print_status "Stopping Aerugo server (PID: $server_pid)..."
            kill $server_pid
            sleep 2
            # Make sure it's really stopped
            if ps -p $server_pid > /dev/null; then
                print_warning "Aerugo server didn't stop gracefully, forcing..."
                kill -9 $server_pid 2>/dev/null || true
            fi
            print_success "Aerugo server stopped"
        fi
        rm .aerugo-server-pid
    fi
    
    # Optionally remove virtual environment
    # rm -rf "$VENV_DIR"
    
    print_success "Cleanup completed"
}

# Handle Ctrl+C and other termination signals
handle_interrupt() {
    echo
    print_warning "Test execution interrupted"
    cleanup
    exit 130
}

# Main execution
main() {
    echo
    print_status "Starting test execution..."
    
    # Set trap for cleanup on normal exit
    trap cleanup EXIT
    
    # Set trap for interruption (Ctrl+C)
    trap handle_interrupt INT TERM
    
    # Execute steps
    check_prerequisites
    setup_venv
    install_dependencies
    check_test_directory
    check_services
    
    echo
    print_status "Environment setup complete, running tests..."
    echo
    
    # Run tests and capture exit code
    if run_tests "$@"; then
        echo
        print_success "🎉 All tests completed successfully!"
        exit 0
    else
        echo
        print_error "❌ Tests failed!"
        exit 1
    fi
}



# Export variables for check_services function
export NO_SERVER
export SERVICES_ONLY

# Run main function
main "$@"
