"""
User management endpoint tests
"""
import random
import string
import time
import requests
from base_test import BaseTestCase
from config import TestUser


class UserTests(BaseTestCase):
    """Test user management functionality with auto-setup"""
    
    def __init__(self):
        super().__init__()
        self.test_user = None
        self.setup_attempted = False
    
    def __getattribute__(self, name):
        """Override to auto-setup before test methods"""
        attr = object.__getattribute__(self, name)
        
        # If this is a test method, ensure setup first
        if name.startswith('test_') and callable(attr):
            def wrapper(*args, **kwargs):
                self.ensure_setup()
                return attr(*args, **kwargs)
            return wrapper
        
        return attr
    
    def ensure_setup(self):
        """Ensure test user is set up before running tests"""
        if self.setup_attempted:
            return
        
        self.setup_attempted = True
        
        try:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            
            # Create fresh test user for this test session
            user_data = {
                'username': f'testuser_{session_id}',
                'email': f'testuser_{session_id}@example.com',
                'password': 'testpass123',
                'full_name': 'Test User'
            }
            
            # Register user
            response = requests.post(f"{self.api_base}/auth/register", json=user_data)
            if response and response.status_code == 201:
                # Login to get token
                login_response = requests.post(f"{self.api_base}/auth/login", json={
                    'username': user_data['username'],
                    'password': user_data['password']
                })
                if login_response and login_response.status_code == 200:
                    token = login_response.json().get('token')
                    self.test_user = TestUser(user_data['username'], user_data['email'], user_data['password'])
                    self.test_user.token = token
                    self.logger.info(f"✅ Setup test user: {user_data['username']}")
            
        except Exception as e:
            self.logger.warning(f"⚠️ User setup failed: {e}")
            # Create mock user to prevent AttributeError
            self.test_user = TestUser("fallback_user", "fallback@example.com", "pass")
            self.test_user.token = "mock_token"

    def test_user_profile_retrieval(self):
        """Test user profile retrieval"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping profile retrieval test")
            return False
        
        response = self.make_request("GET", "/api/user/profile", token=self.test_user.token)
        
        if response and response.status_code == 200:
            data = response.json()
            if 'username' in data or 'email' in data:
                self.logger.info("✅ User profile retrieval test passed")
                return True
        
        self.logger.warning(f"⚠️ User profile retrieval failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_profile_update(self):
        """Test user profile update"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping profile update test")
            return False
        
        updated_data = {
            "full_name": f"Updated User {time.time()}",
            "bio": "This is an updated test user bio",
            "location": "Test City, Test Country"
        }
        
        response = self.make_request("PUT", "/api/user/profile", updated_data, token=self.test_user.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ User profile update test passed")
            return True
        
        self.logger.warning(f"⚠️ User profile update failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_password_change(self):
        """Test user password change"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping password change test")
            return False
        
        password_data = {
            "current_password": self.test_user.password,
            "new_password": "newpassword123",
            "confirm_password": "newpassword123"
        }
        
        response = self.make_request("PUT", "/api/user/password", password_data, token=self.test_user.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ User password change test passed")
            return True
        
        self.logger.warning(f"⚠️ User password change failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_deletion(self):
        """Test user account deletion"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping deletion test")
            return False
        
        response = self.make_request("DELETE", "/api/user/account", token=self.test_user.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ User deletion test passed")
            return True
        
        self.logger.warning(f"⚠️ User deletion failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_public_profile(self):
        """Test accessing public user profile"""
        if not self.test_user:
            self.logger.warning("⚠️ No test user, skipping public profile test")
            return False
        
        response = self.make_request("GET", f"/api/users/{self.test_user.username}")
        
        if response and response.status_code == 200:
            data = response.json()
            if 'username' in data:
                self.logger.info("✅ User public profile test passed")
                return True
        
        self.logger.warning(f"⚠️ User public profile failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_search(self):
        """Test user search functionality"""
        if not self.test_user:
            self.logger.warning("⚠️ No test user, skipping search test")
            return False
        
        response = self.make_request("GET", f"/api/users/search?q={self.test_user.username[:5]}")
        
        if response and response.status_code == 200:
            data = response.json()
            if isinstance(data, list) or 'users' in data:
                self.logger.info("✅ User search test passed")
                return True
        
        self.logger.warning(f"⚠️ User search failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_avatar_upload(self):
        """Test user avatar upload"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping avatar upload test")
            return False
        
        # Mock image data
        avatar_data = {"avatar": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=="}
        
        response = self.make_request("POST", "/api/user/avatar", avatar_data, token=self.test_user.token)
        
        if response and response.status_code in [200, 201]:
            self.logger.info("✅ User avatar upload test passed")
            return True
        
        self.logger.warning(f"⚠️ User avatar upload failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_account_deletion(self):
        """Test user account deletion"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping account deletion test")
            return False
        
        response = self.make_request("DELETE", "/api/user/account", token=self.test_user.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ User account deletion test passed")
            return True
        
        self.logger.warning(f"⚠️ User account deletion failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_email_verification(self):
        """Test user email verification"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping email verification test")
            return False
        
        response = self.make_request("POST", "/api/user/verify-email", {"email": self.test_user.email}, token=self.test_user.token)
        
        if response and response.status_code in [200, 201]:
            self.logger.info("✅ User email verification test passed")
            return True
        
        self.logger.warning(f"⚠️ User email verification failed: {response.status_code if response else 'No response'}")
        return False

    def test_user_preferences(self):
        """Test user preferences"""
        if not self.test_user or not self.test_user.token:
            self.logger.warning("⚠️ No test user token, skipping preferences test")
            return False
        
        prefs_data = {"theme": "dark", "language": "en", "notifications": True}
        
        response = self.make_request("PUT", "/api/user/preferences", prefs_data, token=self.test_user.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ User preferences test passed")
            return True
        
        self.logger.warning(f"⚠️ User preferences failed: {response.status_code if response else 'No response'}")
        return False

    def run_all_tests(self):
        """Run all user tests"""
        self.logger.info("🚀 Starting User Tests")
        
        tests = [
            self.test_user_profile_retrieval,
            self.test_user_profile_update,
            self.test_user_password_change,
            self.test_user_public_profile,
            self.test_user_search,
            # Note: test_user_deletion should be last as it deletes the user
            self.test_user_deletion
        ]
        
        passed = 0
        total = len(tests)
        
        for test in tests:
            try:
                if test():
                    passed += 1
            except Exception as e:
                self.logger.error(f"❌ {test.__name__} failed with exception: {e}")
        
        self.logger.info(f"📊 User Tests: {passed}/{total} passed")
        return passed == total


if __name__ == "__main__":
    import logging
    logging.basicConfig(level=logging.INFO)
    
    user_tests = UserTests()
    user_tests.run_all_tests()
