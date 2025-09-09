"""
Pytest-compatible test runner for Aerugo integration tests
"""

import pytest
import sys
import os
from pathlib import Path

# Add tests directory to Python path
tests_dir = Path(__file__).parent
sys.path.insert(0, str(tests_dir))

from integration_test import IntegrationTestSuite


class TestAerugoIntegration:
    """Pytest wrapper for integration tests"""
    
    @classmethod
    def setup_class(cls):
        """Setup test environment once for all tests"""
        cls.test_suite = IntegrationTestSuite()
        
        try:
            # Setup phase only
            cls.test_suite.setup_environment()
            cls.test_suite.verify_services()
            cls.test_suite.run_migrations()
            cls.test_suite.seed_test_data()
            cls.test_suite.build_and_start_server()
        except Exception as e:
            pytest.fail(f"Test environment setup failed: {e}")
    
    @classmethod
    def teardown_class(cls):
        """Cleanup test environment"""
        if hasattr(cls, 'test_suite'):
            cls.test_suite.stop_server()
            # Note: We don't cleanup test data here to allow inspection
    
    def test_health_endpoints(self):
        """Test health endpoints"""
        from test_health import HealthTests
        health_tests = HealthTests()
        health_tests.run_all_tests()
    
    def test_authentication(self):
        """Test authentication functionality"""
        from test_auth import AuthTests
        auth_tests = AuthTests()
        auth_tests.run_all_tests()
    
    def test_organizations(self):
        """Test organization functionality"""
        from test_organizations import OrganizationTests
        org_tests = OrganizationTests()
        org_tests.run_all_tests()
    
    def test_users(self):
        """Test user management functionality"""
        from test_users import UserTests
        user_tests = UserTests()
        user_tests.run_all_tests()
    
    def test_repositories(self):
        """Test repository functionality"""
        from test_repositories import RepositoryTests
        repo_tests = RepositoryTests()
        repo_tests.run_all_tests()


if __name__ == "__main__":
    # Run with pytest
    pytest.main([__file__, "-v", "-s"])
