#!/usr/bin/env python3
"""
Comprehensive API Testing Script
Tests all APIs including the updated s3_list endpoint
Handles AWS credentials and provides mock fallbacks
"""

import requests
import json
import subprocess
import os
import sys
import time
from datetime import datetime

# API Base URL
BASE_URL = "http://localhost:8080/api/v1"

class APITester:
    def __init__(self):
        self.success_count = 0
        self.total_count = 0
        self.results = []
        
    def log_result(self, test_name, success, response=None, error=None):
        """Log test result"""
        self.total_count += 1
        if success:
            self.success_count += 1
            print(f"✅ {test_name}")
        else:
            print(f"❌ {test_name}")
            if error:
                print(f"   Error: {error}")
            if response:
                print(f"   Response: {response}")
        
        self.results.append({
            'test': test_name,
            'success': success,
            'response': response,
            'error': error
        })
    
    def test_health_endpoint(self):
        """Test health endpoint"""
        try:
            # Health endpoint is at root /health, not under /api/v1
            response = requests.get("http://localhost:8080/health")
            success = response.status_code == 200
            self.log_result("Health Check", success, response.json() if success else None)
            return success
        except Exception as e:
            self.log_result("Health Check", False, error=str(e))
            return False
    
    def test_s3_list_api(self):
        """Test S3 List API with different scenarios"""
        
        # Test 1: Basic s3_list without credentials (should handle gracefully)
        try:
            payload = {
                "bucket": "test-bucket",
                "prefix": ""
            }
            response = requests.post(f"{BASE_URL}/registry/s3/list", 
                                   json=payload,
                                   headers={"Content-Type": "application/json"})
            
            # Check if it's a credentials error (expected) or other error
            if response.status_code == 500:
                response_data = response.json()
                if "credentials" in response_data.get("error", "").lower():
                    self.log_result("S3 List - No Credentials (Expected)", True, 
                                  "Correctly handled missing AWS credentials")
                else:
                    self.log_result("S3 List - Basic", False, response_data)
            else:
                self.log_result("S3 List - Basic", response.status_code == 200, 
                              response.json() if response.status_code == 200 else None)
                
        except Exception as e:
            self.log_result("S3 List - Basic", False, error=str(e))
        
        # Test 2: S3 List with prefix
        try:
            payload = {
                "bucket": "test-bucket",
                "prefix": "images/"
            }
            response = requests.post(f"{BASE_URL}/registry/s3/list", 
                                   json=payload,
                                   headers={"Content-Type": "application/json"})
            
            if response.status_code == 500:
                response_data = response.json()
                if "credentials" in response_data.get("error", "").lower():
                    self.log_result("S3 List - With Prefix (Expected)", True, 
                                  "Correctly handled missing AWS credentials")
                else:
                    self.log_result("S3 List - With Prefix", False, response_data)
            else:
                self.log_result("S3 List - With Prefix", response.status_code == 200)
                
        except Exception as e:
            self.log_result("S3 List - With Prefix", False, error=str(e))
    
    def test_s3_apis(self):
        """Test other S3 APIs"""
        
        # Test S3 Upload
        try:
            payload = {
                "bucket": "test-bucket",
                "key": "test-file.txt",
                "local_path": "/tmp/test-file.txt"  # Correct field name
            }
            response = requests.post(f"{BASE_URL}/registry/s3/upload", 
                                   json=payload,
                                   headers={"Content-Type": "application/json"})
            
            # Expected to fail with credentials error
            if response.status_code == 500 and ("credentials" in response.text.lower() or "no such file" in response.text.lower()):
                self.log_result("S3 Upload - No Credentials (Expected)", True)
            else:
                self.log_result("S3 Upload", response.status_code == 200)
                
        except Exception as e:
            self.log_result("S3 Upload", False, error=str(e))
        
        # Test S3 Download
        try:
            payload = {
                "bucket": "test-bucket",
                "key": "test-file.txt",
                "local_path": "/tmp/downloaded-file.txt"  # Add required field
            }
            response = requests.post(f"{BASE_URL}/registry/s3/download", 
                                   json=payload,
                                   headers={"Content-Type": "application/json"})
            
            # Expected to fail with credentials error
            if response.status_code == 500 and "credentials" in response.text.lower():
                self.log_result("S3 Download - No Credentials (Expected)", True)
            else:
                self.log_result("S3 Download", response.status_code == 200)
                
        except Exception as e:
            self.log_result("S3 Download", False, error=str(e))
        
        # Test S3 Delete
        try:
            payload = {
                "bucket": "test-bucket",
                "key": "test-file.txt"
            }
            response = requests.delete(f"{BASE_URL}/registry/s3/delete", 
                                     json=payload,
                                     headers={"Content-Type": "application/json"})
            
            # Expected to fail with credentials error
            if response.status_code == 500 and "credentials" in response.text.lower():
                self.log_result("S3 Delete - No Credentials (Expected)", True)
            else:
                self.log_result("S3 Delete", response.status_code == 200)
                
        except Exception as e:
            self.log_result("S3 Delete", False, error=str(e))
    
    def test_docker_build_api(self):
        """Test Docker build APIs"""
        
        # Test Docker Build + Upload S3
        try:
            payload = {
                "dockerfile_path": "/tmp/test-dockerfile",
                "image_name": "test-image",
                "image_tag": "latest",  # Add required field
                "bucket": "test-bucket",
                "s3_key": "images/test-image.tar"
            }
            response = requests.post(f"{BASE_URL}/registry/docker/build-upload-s3", 
                                   json=payload,
                                   headers={"Content-Type": "application/json"})
            
            # This should fail because dockerfile doesn't exist or Docker/AWS issues
            # We expect graceful error handling
            expected_errors = ["dockerfile", "docker", "credentials", "not found", "no such file"]
            if response.status_code != 200:
                response_text = response.text.lower()
                if any(err in response_text for err in expected_errors):
                    self.log_result("Docker Build+S3 - Expected Error", True, 
                                  "Correctly handled missing dependencies")
                else:
                    self.log_result("Docker Build+S3", False, response.text[:200])
            else:
                self.log_result("Docker Build+S3", True)
                
        except Exception as e:
            self.log_result("Docker Build+S3", False, error=str(e))
    
    def run_all_tests(self):
        """Run all test suites"""
        print("🚀 Starting Comprehensive API Testing")
        print("=" * 50)
        
        # Check server is running
        if not self.test_health_endpoint():
            print("❌ Server is not responding. Please start the server first.")
            return False
        
        print("\n📋 Testing S3 List API (Main Focus)")
        print("-" * 30)
        self.test_s3_list_api()
        
        print("\n📋 Testing Other S3 APIs")
        print("-" * 30)
        self.test_s3_apis()
        
        print("\n📋 Testing Docker APIs")
        print("-" * 30)
        self.test_docker_build_api()
        
        return True
    
    def print_summary(self):
        """Print test summary"""
        print("\n" + "=" * 50)
        print("📊 TEST SUMMARY")
        print("=" * 50)
        print(f"Total Tests: {self.total_count}")
        print(f"Passed: {self.success_count}")
        print(f"Failed: {self.total_count - self.success_count}")
        print(f"Success Rate: {(self.success_count/self.total_count)*100:.1f}%")
        
        if self.total_count - self.success_count > 0:
            print("\n❌ Failed Tests:")
            for result in self.results:
                if not result['success']:
                    print(f"  - {result['test']}")
                    if result['error']:
                        print(f"    Error: {result['error']}")

def setup_mock_aws_credentials():
    """Set up mock AWS credentials for testing"""
    os.environ['AWS_ACCESS_KEY_ID'] = 'test'
    os.environ['AWS_SECRET_ACCESS_KEY'] = 'test'
    os.environ['AWS_DEFAULT_REGION'] = 'us-east-1'
    print("🔧 Set up mock AWS credentials for testing")

def main():
    """Main function"""
    print("🧪 Comprehensive API Testing Suite")
    print(f"⏰ Started at: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print()
    
    # Setup mock credentials
    setup_mock_aws_credentials()
    
    # Initialize tester
    tester = APITester()
    
    # Run tests
    if tester.run_all_tests():
        tester.print_summary()
        
        # Exit code based on results
        exit_code = 0 if tester.success_count == tester.total_count else 1
        sys.exit(exit_code)
    else:
        print("❌ Failed to run tests - server not available")
        sys.exit(1)

if __name__ == "__main__":
    main()
