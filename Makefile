# Aerugo Development Makefile
.PHONY: help test test-python test-rust clean setup install-deps

# Default target
help: ## Show this help message
	@echo "Aerugo Development Commands"
	@echo "=========================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Python testing
test-python: ## Run Python integration tests
	@echo "🐍 Running Python integration tests..."
	./runtest.sh

test-python-verbose: ## Run Python tests in verbose mode
	@echo "🐍 Running Python tests (verbose)..."
	./runtest.sh --verbose

test-python-coverage: ## Run Python tests with coverage
	@echo "🐍 Running Python tests with coverage..."
	./runtest.sh --coverage

test-python-file: ## Run specific Python test file (usage: make test-python-file FILE=test_auth.py)
	@echo "🐍 Running Python test file: $(FILE)"
	./runtest.sh $(FILE)

test-full: ## Run full integration tests with server startup
	@echo "🚀 Running full integration tests with server..."
	./test-with-server.sh

test-mock: ## Run mock tests to verify test infrastructure
	@echo "🧪 Running mock tests..."
	python mock-tests.py

test-status: ## Check test environment status
	@echo "🔍 Checking test environment status..."
	./check-test-status.sh

# Quick pytest commands
pytest: ## Run pytest directly in current environment
	@echo "🧪 Running pytest directly..."
	pytest tests/ -v

pytest-verbose: ## Run pytest with maximum verbosity
	@echo "🧪 Running pytest (verbose)..."
	pytest tests/ -vvs --tb=long

pytest-quiet: ## Run pytest in quiet mode
	@echo "🧪 Running pytest (quiet)..."
	pytest tests/ -q

pytest-coverage: ## Run pytest with coverage
	@echo "🧪 Running pytest with coverage..."
	pytest tests/ --cov=tests --cov-report=html --cov-report=term

# Rust testing
test-rust: ## Run Rust tests
	@echo "🦀 Running Rust tests..."
	cargo test

test-rust-verbose: ## Run Rust tests in verbose mode
	@echo "🦀 Running Rust tests (verbose)..."
	cargo test -- --nocapture

# Combined testing
test: test-rust test-python ## Run all tests (Rust + Python)

test-all: ## Run comprehensive test suite
	@echo "🚀 Running comprehensive test suite..."
	@echo "1️⃣ Rust tests..."
	cargo test
	@echo "2️⃣ Python integration tests..."
	./runtest.sh
	@echo "✅ All tests completed!"

# Development setup
setup: ## Set up development environment
	@echo "⚙️ Setting up development environment..."
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Setting up Python test environment..."
	./runtest.sh --help > /dev/null 2>&1 || echo "Run ./runtest.sh to set up Python environment"
	@echo "✅ Development environment ready!"

install-deps: ## Install all dependencies
	@echo "📦 Installing dependencies..."
	cargo fetch
	pip3 install -r tests/requirements.txt
	@echo "✅ Dependencies installed!"

# Cleanup
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf target/
	rm -rf venv-test/
	rm -rf htmlcov/
	rm -rf .pytest_cache/
	rm -rf __pycache__/
	find . -name "*.pyc" -delete
	find . -name "*.pyo" -delete
	@echo "✅ Cleanup completed!"

# Linting and formatting
lint: ## Run linting checks
	@echo "🔍 Running linting checks..."
	cargo clippy -- -D warnings
	cargo fmt --check

format: ## Format code
	@echo "✨ Formatting code..."
	cargo fmt

# Docker operations
docker-build: ## Build Docker image
	@echo "🐳 Building Docker image..."
	docker build -t aerugo:latest .

docker-test: ## Test Docker build
	@echo "🐳 Testing Docker build..."
	docker build -t aerugo:test .
	docker run --rm aerugo:test --version

# Development helpers
watch-test: ## Watch for changes and run tests
	@echo "👀 Watching for changes (Rust tests)..."
	cargo watch -x test

watch-test-python: ## Watch for changes and run Python tests
	@echo "👀 Watching for changes (Python tests)..."
	watchexec -e py ./runtest.sh

# Information
info: ## Show project information
	@echo "📋 Project Information"
	@echo "====================="
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Python version: $$(python3 --version)"
	@echo "Pytest version: $$(pytest --version 2>/dev/null || echo 'Not installed')"
	@echo ""
	@echo "Project structure:"
	@tree -L 2 -I 'target|venv*|__pycache__' . || ls -la
