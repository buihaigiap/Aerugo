"""
Health endpoint tests
"""

import requests
from base_test import BaseTestCase
from config import SERVER_URL


class HealthTests(BaseTestCase):
    """Test health endpoint functionality"""
    
    def test_health_check(self):
        """Test basic health check endpoint"""
        self.logger.info("Testing health check endpoint")
        
        response = requests.get(f"{SERVER_URL}/health")
        self.assert_response(response, 200, "Health check failed")
        
        # Verify response content
        assert response.text.strip() == "OK", f"Unexpected health response: {response.text}"
        
        self.logger.info("✅ Health check test passed")
    
    def test_health_check_headers(self):
        """Test health check response headers"""
        self.logger.info("Testing health check response headers")
        
        response = requests.get(f"{SERVER_URL}/health")
        self.assert_response(response, 200, "Health check failed")
        
        # Check for expected headers
        assert "content-type" in response.headers, "Content-Type header missing"
        assert "date" in response.headers, "Date header missing"
        
        self.logger.info("✅ Health check headers test passed")
    
    def test_health_check_methods(self):
        """Test health check endpoint with different HTTP methods"""
        self.logger.info("Testing health check with different HTTP methods")
        
        # GET should work
        response = requests.get(f"{SERVER_URL}/health")
        self.assert_response(response, 200, "GET health check failed")
        
        # HEAD should work
        response = requests.head(f"{SERVER_URL}/health")
        self.assert_response(response, 200, "HEAD health check failed")
        
        # POST should not be allowed
        response = requests.post(f"{SERVER_URL}/health")
        assert response.status_code in [405, 404], f"POST should not be allowed: {response.status_code}"
        
        self.logger.info("✅ Health check methods test passed")
    
    def run_all_tests(self):
        """Run all health tests"""
        self.logger.info("=== Running Health Tests ===")
        
        self.test_health_check()
        self.test_health_check_headers()
        self.test_health_check_methods()
        
        self.logger.info("✅ All health tests passed")
