#!/bin/bash

# Aerugo Integration Test Runner
# This script sets up Python environment and runs the integration tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
BASE_DIR="$(dirname "$SCRIPT_DIR")"

echo "🚀 Aerugo Integration Test Runner"
echo "=================================="

# Check if Python3 is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 is required but not found"
    exit 1
fi

# Check if pip is available
if ! command -v pip3 &> /dev/null; then
    echo "❌ pip3 is required but not found"
    exit 1
fi

# Create virtual environment if it doesn't exist
VENV_DIR="$SCRIPT_DIR/venv"
if [ ! -d "$VENV_DIR" ]; then
    echo "📦 Creating Python virtual environment..."
    python3 -m venv "$VENV_DIR"
fi

# Activate virtual environment
echo "🔌 Activating virtual environment..."
source "$VENV_DIR/bin/activate"

# Install/upgrade dependencies
echo "📋 Installing Python dependencies..."
pip install --upgrade pip
pip install -r "$SCRIPT_DIR/requirements.txt"

# Change to base directory for test execution
cd "$BASE_DIR"

# Set environment variables for testing
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Run the integration tests
echo "🧪 Running integration tests..."
echo "=================================="

python3 "$SCRIPT_DIR/integration_test.py"

echo "=================================="
echo "✅ Integration tests completed!"

# Deactivate virtual environment
deactivate
