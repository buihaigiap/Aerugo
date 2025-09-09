"""
Pytest configuration and fixtures
"""

import sys
import os
import pytest
import time
import random
import string

# Add tests directory to Python path
current_dir = os.path.dirname(os.path.abspath(__file__))
if current_dir not in sys.path:
    sys.path.insert(0, current_dir)

# Also add parent directory (project root)
project_root = os.path.dirname(current_dir)
if project_root not in sys.path:
    sys.path.insert(0, project_root)

@pytest.fixture(scope="session", autouse=True)
def setup_test_environment():
    """
    Session-wide setup for all tests
    """
    print("\n🔧 Setting up test environment...")
    
    # Ensure Python path is correctly set
    test_dir = os.path.dirname(os.path.abspath(__file__))
    if test_dir not in sys.path:
        sys.path.insert(0, test_dir)
    
    # Generate unique test session ID to avoid conflicts
    session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
    os.environ['TEST_SESSION_ID'] = session_id
    print(f"Test session ID: {session_id}")
    
    yield
    
    print("\n🧹 Cleaning up test environment...")

@pytest.fixture(scope="function", autouse=True)  
def reset_test_data():
    """
    Reset test data before each test to avoid conflicts
    """
    # Import here to avoid circular imports
    try:
        from base_test import test_data_manager
        test_data_manager.cleanup_test_data()
        # Small delay to ensure cleanup completes
        time.sleep(0.1)
    except ImportError:
        pass
    
    yield
    
    # Cleanup after test
    try:
        from base_test import test_data_manager
        test_data_manager.cleanup_test_data()
    except ImportError:
        pass

@pytest.fixture(scope="function")
def test_client():
    """
    Fixture to provide test client for individual tests
    """
    return None

@pytest.fixture(scope="function")
def fresh_test_user():
    """
    Create a fresh test user for each test
    """
    session_id = os.environ.get('TEST_SESSION_ID', 'default')
    test_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
    
    return {
        'username': f'testuser_{session_id}_{test_id}',
        'email': f'test_{session_id}_{test_id}@example.com',
        'password': 'testpass123',
        'full_name': f'Test User {test_id}'
    }

@pytest.fixture(scope="function")
def fresh_test_org():
    """
    Create a fresh test organization for each test
    """
    session_id = os.environ.get('TEST_SESSION_ID', 'default')
    test_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
    
    return {
        'name': f'testorg_{session_id}_{test_id}',
        'display_name': f'Test Organization {test_id}',
        'description': f'Test organization for session {session_id}'
    }

# Pytest markers
pytest_plugins = []

# Test configuration
def pytest_configure(config):
    """Configure pytest"""
    config.addinivalue_line(
        "markers", "slow: mark test as slow running"
    )
    config.addinivalue_line(
        "markers", "integration: mark test as integration test"  
    )
    config.addinivalue_line(
        "markers", "requires_services: mark test as requiring external services"
    )

def pytest_collection_modifyitems(config, items):
    """Modify test collection"""
    # Add markers to tests based on their names/paths
    for item in items:
        if "integration" in item.name:
            item.add_marker(pytest.mark.integration)
        if any(keyword in item.name for keyword in ["slow", "performance"]):
            item.add_marker(pytest.mark.slow)
        if any(keyword in str(item.fspath) for keyword in ["database", "redis", "minio"]):
            item.add_marker(pytest.mark.requires_services)

def pytest_runtest_setup(item):
    """Setup for each test"""
    # Add small delay between tests to avoid race conditions
    time.sleep(0.05)
