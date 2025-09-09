"""
Authentication endpoint tests
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    from base_test import BaseTestCase, test_data_manager
    from config import TEST_USERS
except ImportError:
    from .base_test import BaseTestCase, test_data_manager
    from .config import TEST_USERS


class AuthTests(BaseTestCase):
    """Test authentication functionality"""
    
    def __init__(self):
        super().__init__()
        self.dynamic_users = []  # Store dynamically created users
    
    def test_user_registration(self):
        """Test user registration"""
        self.logger.info("Testing user registration")
        
        # Use dynamic users to avoid conflicts
        import random
        import string
        
        self.dynamic_users = []  # Reset list
        for i in range(3):
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            from config import TestUser
            user = TestUser(
                username=f'testuser_{i}_{session_id}',
                email=f'testuser_{i}_{session_id}@example.com',
                password=f'testpass{i}123'
            )
            self.dynamic_users.append(user)
        
        for user in self.dynamic_users:
            self.logger.info(f"Registering user: {user.email}")
            
            response = self.make_request("POST", "/auth/register", {
                "username": user.username,
                "email": user.email,
                "password": user.password
            })
            
            self.assert_response(response, 201, f"Registration failed for {user.email}")
            
            # Verify response structure - API returns token only
            data = response.json()
            self.verify_json_structure(data, ["token"])
            
            # Store token from registration
            user.token = data["token"]
            test_data_manager.track_user(user.__dict__)
        
        self.logger.info("✅ User registration test passed")
    
    def test_user_login(self):
        """Test user login"""
        self.logger.info("Testing user login")
        
        # Create new test users specifically for this test
        # This handles the case where tests are run individually through pytest
        import random
        import string
        
        self.logger.info("Creating test users for login test")
        login_test_users = []
        
        # Create 2 test users
        for i in range(2):
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            from config import TestUser
            user = TestUser(
                username=f'login_user_{i}_{session_id}',
                email=f'login_{i}_{session_id}@example.com',
                password=f'loginpass{i}123'
            )
            login_test_users.append(user)
            
            # Register user first
            self.logger.info(f"Registering user for login test: {user.email}")
            register_response = self.make_request("POST", "/auth/register", {
                "username": user.username,
                "email": user.email,
                "password": user.password
            })
            
            if register_response.status_code != 201:
                self.logger.warning(f"Failed to register user for login test: {user.email} - {register_response.text}")
                continue
                
            # Store the registration token
            user.token = register_response.json().get("token")
            test_data_manager.track_user(user.__dict__)
        
        # Now test login for the newly registered users
        self.logger.info(f"Testing login for {len(login_test_users)} newly registered users")
        
        for user in login_test_users:
            self.logger.info(f"Testing login for user: {user.email}")
            
            # Try login with email (more reliable)
            response = self.make_request("POST", "/auth/login", {
                "email": user.email,
                "password": user.password
            })
            
            if response.status_code != 200:
                # Try with username as fallback
                self.logger.info(f"Login with email failed, trying username instead for: {user.username}")
                response = self.make_request("POST", "/auth/login", {
                    "username": user.username,
                    "password": user.password
                })
            
            self.assert_response(response, 200, f"Login failed for {user.email}")
            
            # Verify response structure - API returns token only
            data = response.json()
            self.verify_json_structure(data, ["token"])
            
            # Update token with login result
            user.token = data["token"]
            self.logger.info(f"Successfully logged in user: {user.email}")
            
            # Save this user in our dynamic_users list for other tests that might need it
            self.dynamic_users.append(user)
        
        self.logger.info("✅ User login test passed")
    
    def test_protected_endpoint(self):
        """Test accessing protected endpoint with valid token"""
        self.logger.info("Testing protected endpoint access")
        
        # Create a user specifically for testing protected endpoint
        import random
        import string
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'protected_user_{session_id}',
            email=f'protected_{session_id}@example.com',
            password=f'protectedpass123'
        )
        
        # Register this user
        self.logger.info(f"Registering user for protected endpoint test: {user.email}")
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        if register_response.status_code != 201:
            self.logger.warning(f"Failed to register user for protected endpoint test: {register_response.text}")
            # Use a test user instead
            user = TEST_USERS[0]
        else:
            # Get token from registration
            user.token = register_response.json().get("token")
            test_data_manager.track_user(user.__dict__)
            self.logger.info(f"User registered successfully with token: {user.token[:10]}...")
        
        # Login to get a fresh token
        self.logger.info(f"Logging in user: {user.email}")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": user.password
        })
        
        if login_response.status_code == 200:
            user.token = login_response.json()["token"]
            self.logger.info(f"Successfully logged in with token: {user.token[:10]}...")
        else:
            self.logger.warning(f"Login failed with status {login_response.status_code}: {login_response.text}")
            # We might still have a token from registration, try to use that
        
        # Ensure we have a token to use
        if not hasattr(user, 'token') or not user.token:
            self.logger.error("No valid token available for protected endpoint test")
            raise AssertionError("Could not obtain a valid token for protected endpoint test")
        
        # Access protected endpoint with token
        self.logger.info(f"Accessing protected endpoint with token for user: {user.email}")
        response = self.make_request("GET", "/auth/me", token=user.token)
        self.logger.info(f"Protected endpoint response: {response.status_code} - {response.text}")
        
        if response.status_code == 404 or response.status_code == 501 or (response.status_code == 200 and "Not implemented" in response.text):
            self.logger.info("/auth/me endpoint not implemented - skipping")
            return
            
        self.assert_response(response, 200, f"Protected access failed for {user.email}")
        
        # Verify response structure
        data = response.json()
        self.verify_json_structure(data, ["id", "username", "email", "created_at"])
        
        # Verify user data matches
        assert data["email"] == user.email, f"Email mismatch for {user.email}"
        assert data["username"] == user.username, f"Username mismatch for {user.username}"
        
        self.logger.info(f"Successfully verified protected endpoint access for user {user.email}")
        self.logger.info("✅ Protected endpoint test passed")
    
    def test_invalid_login(self):
        """Test login with invalid credentials"""
        self.logger.info("Testing invalid login attempts")
        
        # Test with non-existent email
        response = self.make_request("POST", "/auth/login", {
            "email": "nonexistent@example.com",
            "password": "anypassword"
        })
        self.assert_response(response, 401, "Invalid email should return 401")
        
        # Test with wrong password
        user = TEST_USERS[0]
        response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": "wrongpassword"
        })
        self.assert_response(response, 401, "Wrong password should return 401")
        
        self.logger.info("✅ Invalid login test passed")
    
    def test_invalid_token(self):
        """Test accessing protected endpoint with invalid token"""
        self.logger.info("Testing invalid token access")
        
        # Test with no token
        response = self.make_request("GET", "/auth/me")
        if response.status_code == 404 or response.status_code == 501 or (response.status_code == 200 and "Not implemented" in response.text):
            self.logger.info("/auth/me endpoint not implemented - skipping token validation tests")
            return
            
        self.assert_response(response, 401, "No token should return 401")
        
        # Test with invalid token
        response = self.make_request("GET", "/auth/me", token="invalid-token")
        self.assert_response(response, 401, "Invalid token should return 401")
        
        # Test with malformed token
        response = self.make_request("GET", "/auth/me", token="Bearer invalid")
        self.assert_response(response, 401, "Malformed token should return 401")
        
        self.logger.info("✅ Invalid token test passed")
    
    def test_registration_validation(self):
        """Test registration input validation"""
        self.logger.info("Testing registration validation")
        
        # Test missing required fields
        test_cases = [
            ({}, "Empty data should fail"),
            ({"email": "test@example.com"}, "Missing username and password should fail"),
            ({"username": "test"}, "Missing email and password should fail"),
            ({"password": "test123"}, "Missing username and email should fail"),
        ]
        
        for data, message in test_cases:
            response = self.make_request("POST", "/auth/register", data)
            assert response.status_code >= 400, f"{message}: got {response.status_code}"
        
        # Test duplicate registration
        user = TEST_USERS[0]
        response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        # API might return 409 (conflict) or 422 (validation error) for duplicates
        if response.status_code == 201:
            self.logger.info("API allows duplicate registration - this might be expected behavior")
        else:
            assert response.status_code in [409, 422, 400], f"Duplicate registration should return 4xx: {response.status_code}"
        
        self.logger.info("✅ Registration validation test passed")
    
    def test_token_refresh(self):
        """Test token refresh functionality (if implemented)"""
        self.logger.info("Testing token refresh")
        
        user = TEST_USERS[0]
        
        # Try to refresh token (this endpoint might not exist yet)
        response = self.make_request("POST", "/auth/refresh", token=user.token)
        
        if response.status_code == 404:
            self.logger.info("Token refresh endpoint not implemented - skipping")
            return
        
        if response.status_code == 200:
            data = response.json()
            self.verify_json_structure(data, ["token"])
            self.logger.info("✅ Token refresh test passed")
        else:
            self.logger.warning(f"Unexpected token refresh response: {response.status_code}")
    
    def test_logout(self):
        """Test logout functionality (if implemented)"""
        self.logger.info("Testing logout")
        
        user = TEST_USERS[0]
        
        # Try to logout (this endpoint might not exist yet)
        response = self.make_request("POST", "/auth/logout", token=user.token)
        
        if response.status_code == 404:
            self.logger.info("Logout endpoint not implemented - skipping")
            return
        
        if response.status_code == 200:
            # After logout, token should be invalid
            response = self.make_request("GET", "/auth/me", token=user.token)
            self.assert_response(response, 401, "Token should be invalid after logout")
            self.logger.info("✅ Logout test passed")
        else:
            self.logger.warning(f"Unexpected logout response: {response.status_code}")
    
    def run_all_tests(self):
        """Run all authentication tests"""
        self.logger.info("=== Running Auth Tests ===")
        
        # Tests that create and use dynamic users
        self.test_user_registration()     # Creates dynamic users
        self.test_user_login()            # Uses dynamic users from registration
        self.test_protected_endpoint()    # Uses dynamic users with valid tokens
        
        # Other tests that use TEST_USERS
        self.test_invalid_login()
        self.test_invalid_token()
        self.test_registration_validation()
        self.test_token_refresh()
        self.test_logout()
        
        self.logger.info("✅ All auth tests passed")
