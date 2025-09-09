#!/bin/bash

# Aerugo Python Integration Tests Runner
set -e

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

# Check services availability
check_services() {
    print_status "Checking service availability..."
    
    # Check PostgreSQL
    if timeout 5 bash -c "</dev/tcp/localhost/5433" 2>/dev/null; then
        print_success "PostgreSQL (port 5433) is available"
    else
        print_warning "PostgreSQL (port 5433) is not available"
    fi
    
    # Check Redis
    if timeout 5 bash -c "</dev/tcp/localhost/6380" 2>/dev/null; then
        print_success "Redis (port 6380) is available"
    else
        print_warning "Redis (port 6380) is not available"
    fi
    
    # Check MinIO
    if timeout 5 bash -c "</dev/tcp/localhost/9001" 2>/dev/null; then
        print_success "MinIO (port 9001) is available"
    else
        print_warning "MinIO (port 9001) is not available"
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
    deactivate 2>/dev/null || true
    # Optionally remove virtual environment
    # rm -rf "$VENV_DIR"
    print_success "Cleanup completed"
}

# Main execution
main() {
    echo
    print_status "Starting test execution..."
    
    # Set trap for cleanup
    trap cleanup EXIT
    
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

# Help function
show_help() {
    echo "Usage: $0 [OPTIONS] [TEST_FILE]"
    echo
    echo "Options:"
    echo "  -h, --help      Show this help message"
    echo "  -v, --verbose   Run tests in verbose mode"
    echo "  -q, --quiet     Run tests in quiet mode"
    echo "  -c, --coverage  Run tests with coverage report"
    echo
    echo "Examples:"
    echo "  $0                          # Run all tests via pytest wrapper"
    echo "  $0 -v                       # Run all tests verbosely"
    echo "  $0 -c                       # Run with coverage report"
    echo "  $0 pytest_integration.py   # Run pytest integration wrapper"
    echo "  $0 run_all_tests.py        # Run direct test runner"
    echo
    echo "Direct pytest usage:"
    echo "  pytest tests/pytest_integration.py     # Run via pytest directly"
    echo "  python tests/run_all_tests.py          # Run direct test runner"
    echo
    echo "Test files are in the '$TEST_DIR' directory."
    echo "The script will automatically setup a virtual environment and install dependencies."
}

# Check for help flag
if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    show_help
    exit 0
fi

# Run main function
main "$@"
