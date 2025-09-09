"""
Base test utilities and common functionality
"""

import sys
import os

# Add tests directory to Python path for imports
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import requests
import logging
import time
import subprocess
import psycopg2
import redis
from typing import Dict, Optional, List
from pathlib import Path

try:
    from config import TEST_CONFIG, SERVER_URL, API_BASE, get_database_url
except ImportError:
    from .config import TEST_CONFIG, SERVER_URL, API_BASE, get_database_url


class BaseTestCase:
    """Base class for integration tests with common utilities"""
    
    def __init__(self):
        self.logger = logging.getLogger(self.__class__.__name__)
    
    def make_request(self, method: str, endpoint: str, data: Optional[Dict] = None,
                    headers: Optional[Dict] = None, token: Optional[str] = None,
                    expected_status: Optional[int] = None) -> requests.Response:
        """Make HTTP request to the API"""
        if endpoint.startswith('/'):
            url = f"{API_BASE}{endpoint}"
        elif endpoint.startswith('http'):
            url = endpoint
        else:
            url = f"{API_BASE}/{endpoint}"
        
        request_headers = {"Content-Type": "application/json"}
        if headers:
            request_headers.update(headers)
        if token:
            request_headers["Authorization"] = f"Bearer {token}"
        
        self.logger.debug(f"{method} {url} - Data: {data}")
        
        response = requests.request(
            method=method,
            url=url,
            json=data,
            headers=request_headers,
            timeout=30
        )
        
        self.logger.debug(f"Response: {response.status_code} - {response.text[:500]}")
        
        if expected_status and response.status_code != expected_status:
            self.logger.error(f"Expected status {expected_status}, got {response.status_code}")
            self.logger.error(f"Response: {response.text}")
            
        return response
    
    def assert_response(self, response: requests.Response, expected_status: int, message: str = ""):
        """Assert response status code"""
        if response.status_code != expected_status:
            error_msg = f"{message}: Expected {expected_status}, got {response.status_code} - {response.text}"
            self.logger.error(error_msg)
            raise AssertionError(error_msg)
    
    def wait_for_service(self, url: str, timeout: int = 30, interval: int = 2) -> bool:
        """Wait for a service to become available"""
        self.logger.info(f"Waiting for service at {url}...")
        
        for attempt in range(timeout // interval):
            try:
                response = requests.get(url, timeout=5)
                if response.status_code == 200:
                    self.logger.info(f"✅ Service at {url} is available")
                    return True
            except requests.exceptions.RequestException:
                pass
            
            if attempt < (timeout // interval) - 1:
                time.sleep(interval)
        
        self.logger.error(f"❌ Service at {url} not available after {timeout}s")
        return False
    
    def get_db_connection(self):
        """Get database connection"""
        return psycopg2.connect(
            host=TEST_CONFIG["database"]["host"],
            port=TEST_CONFIG["database"]["port"],
            user=TEST_CONFIG["database"]["user"],
            password=TEST_CONFIG["database"]["password"],
            database=TEST_CONFIG["database"]["database"]
        )
    
    def get_redis_connection(self):
        """Get Redis connection"""
        return redis.Redis(
            host=TEST_CONFIG["redis"]["host"],
            port=TEST_CONFIG["redis"]["port"]
        )
    
    def clean_database(self, tables: List[str]):
        """Clean specified database tables"""
        try:
            conn = self.get_db_connection()
            cursor = conn.cursor()
            
            # Disable foreign key checks temporarily
            cursor.execute("SET session_replication_role = replica;")
            
            for table in tables:
                self.logger.debug(f"Cleaning table: {table}")
                cursor.execute(f"DELETE FROM {table}")
            
            # Re-enable foreign key checks
            cursor.execute("SET session_replication_role = DEFAULT;")
            
            conn.commit()
            cursor.close()
            conn.close()
            
            self.logger.debug(f"Cleaned tables: {', '.join(tables)}")
            
        except Exception as e:
            self.logger.error(f"Failed to clean database tables: {e}")
            raise
    
    def run_sql_script(self, script: str):
        """Execute SQL script"""
        try:
            conn = self.get_db_connection()
            cursor = conn.cursor()
            cursor.execute(script)
            conn.commit()
            cursor.close()
            conn.close()
        except Exception as e:
            self.logger.error(f"Failed to execute SQL script: {e}")
            raise
    
    def verify_json_structure(self, data: dict, required_fields: List[str], 
                             optional_fields: List[str] = None):
        """Verify JSON response structure"""
        optional_fields = optional_fields or []
        
        # Check required fields
        missing_fields = [field for field in required_fields if field not in data]
        if missing_fields:
            raise AssertionError(f"Missing required fields: {missing_fields}")
        
        # Log optional fields that are present
        present_optional = [field for field in optional_fields if field in data]
        if present_optional:
            self.logger.debug(f"Present optional fields: {present_optional}")
    
    def setup_test_user(self, user_data: dict) -> dict:
        """Register and login a test user, return user with token"""
        # Register user
        register_response = self.make_request("POST", "/auth/register", user_data)
        self.assert_response(register_response, 201, "User registration failed")
        
        # Login user
        login_response = self.make_request("POST", "/auth/login", {
            "email": user_data["email"],
            "password": user_data["password"]
        })
        self.assert_response(login_response, 200, "User login failed")
        
        login_data = login_response.json()
        user_data["token"] = login_data["token"]
        
        return user_data
    
    def setup_test_organization(self, org_data: dict, owner_token: str) -> dict:
        """Create a test organization"""
        response = self.make_request("POST", "/orgs", org_data, token=owner_token)
        self.assert_response(response, 201, "Organization creation failed")
        
        created_org = response.json()
        org_data.update(created_org)
        
        return org_data


class TestDataManager:
    """Manages test data lifecycle"""
    
    def __init__(self):
        self.logger = logging.getLogger(self.__class__.__name__)
        self.created_users = []
        self.created_orgs = []
        self.created_repos = []
    
    def track_user(self, user_data: dict):
        """Track created user for cleanup"""
        self.created_users.append(user_data)
    
    def track_org(self, org_data: dict):
        """Track created organization for cleanup"""
        self.created_orgs.append(org_data)
    
    def track_repo(self, repo_data: dict):
        """Track created repository for cleanup"""
        self.created_repos.append(repo_data)
    
    def cleanup_test_data(self):
        """Clean up test data to avoid conflicts between tests"""
        try:
            import psycopg2
            from config import get_database_url
            
            # Connect to test database
            conn = psycopg2.connect(get_database_url())
            conn.autocommit = True
            cursor = conn.cursor()
            
            # Delete test data (be careful with production!)
            # Only delete test data with specific patterns
            cursor.execute("""
                DELETE FROM users WHERE email LIKE 'test_%@example.com' 
                OR email LIKE '%_test@example.com'
                OR username LIKE 'testuser_%'
            """)
            
            cursor.execute("""
                DELETE FROM organizations WHERE name LIKE 'testorg_%'
                OR display_name LIKE 'Test Organization %'
            """)
            
            cursor.execute("""
                DELETE FROM repositories WHERE name LIKE 'testrepo_%'
                OR description LIKE 'Test repository %'
            """)
            
            conn.close()
            self.logger.info("✅ Test data cleanup completed")
            
        except Exception as e:
            # Don't fail tests if cleanup fails
            self.logger.warning(f"⚠️ Test data cleanup warning: {e}")
    
    def cleanup_all(self):
        """Clean up all tracked test data"""
        self.logger.info("Cleaning up test data...")
        
        # Call the new cleanup method
        self.cleanup_test_data()
        
        # Clear tracking lists
        self.created_users.clear()
        self.created_orgs.clear()  
        self.created_repos.clear()
        
        self.logger.info("✅ Test data cleanup completed")


# Global test data manager instance
test_data_manager = TestDataManager()
