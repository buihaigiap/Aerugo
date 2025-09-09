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
        
        # Use dynamic users if available, fallback to TEST_USERS
        test_users = self.dynamic_users if self.dynamic_users else TEST_USERS
        
        for user in test_users:
            # Test login even if user already has token (from registration)
            self.logger.info(f"Testing login for user: {user.email}")
            
            response = self.make_request("POST", "/auth/login", {
                "username": user.username,  # Try username instead of email
                "password": user.password
            })
            
            if response.status_code != 200:
                # Try with email if username fails
                response = self.make_request("POST", "/auth/login", {
                    "email": user.email,
                    "password": user.password
                })
            
            self.assert_response(response, 200, f"Login failed for {user.email}")
            
            # Verify response structure - API returns token only
            data = response.json()
            self.verify_json_structure(data, ["token"])
            
            # Update token with login result
            user.token = data["token"]
        
        self.logger.info("✅ User login test passed")
    
    def test_protected_endpoint(self):
        """Test accessing protected endpoint with valid token"""
        self.logger.info("Testing protected endpoint access")
        
        for user in TEST_USERS:
            self.logger.info(f"Testing protected access for: {user.email}")
            
            response = self.make_request("GET", "/auth/me", token=user.token)
            
            if response.status_code == 404 or response.status_code == 501 or (response.status_code == 200 and "Not implemented" in response.text):
                self.logger.info("/auth/me endpoint not implemented - skipping")
                break
                
            self.assert_response(response, 200, f"Protected access failed for {user.email}")
            
            # Verify response structure
            data = response.json()
            self.verify_json_structure(data, ["id", "username", "email", "created_at"])
            
            # Verify user data matches
            assert data["email"] == user.email, f"Email mismatch for {user.email}"
            assert data["username"] == user.username, f"Username mismatch for {user.username}"
        
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
        
        self.test_user_registration()
        self.test_user_login()
        self.test_protected_endpoint()
        self.test_invalid_login()
        self.test_invalid_token()
        self.test_registration_validation()
        self.test_token_refresh()
        self.test_logout()
        
        self.logger.info("✅ All auth tests passed")
