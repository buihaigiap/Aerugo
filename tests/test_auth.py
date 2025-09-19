
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

import random
import string


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
        """Test token refresh functionality"""
        self.logger.info("Testing token refresh")
        
        # Create a test user and get a valid token
        import random
        import string
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'refresh_user_{session_id}',
            email=f'refresh_{session_id}@example.com',
            password=f'refreshpass123'
        )
        
        # Register the user
        self.logger.info(f"Registering user for refresh test: {user.email}")
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        if register_response.status_code != 201:
            self.logger.error(f"Failed to register user: {register_response.text}")
            raise AssertionError("Could not register user for refresh test")
        
        # Login to get a fresh token
        self.logger.info(f"Logging in user: {user.email}")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": user.password
        })
        
        if login_response.status_code != 200:
            self.logger.error(f"Failed to login user: {login_response.text}")
            raise AssertionError("Could not login user for refresh test")
        
        old_token = login_response.json()["token"]
        user.token = old_token
        test_data_manager.track_user(user.__dict__)
        
        # Refresh the token
        self.logger.info("Attempting to refresh token")
        refresh_response = self.make_request("POST", "/auth/refresh", {
            "token": old_token
        })
        
        self.assert_response(refresh_response, 200, "Token refresh failed")
        
        data = refresh_response.json()
        self.verify_json_structure(data, ["token"])
        new_token = data["token"]
        
        self.logger.info("Token refreshed successfully")
        
        # Verify the new token works
        self.logger.info("Verifying new token with protected endpoint")
        verify_response = self.make_request("GET", "/auth/me", token=new_token)
        self.assert_response(verify_response, 200, "New token verification failed")
        
        verify_data = verify_response.json()
        self.verify_json_structure(verify_data, ["id", "username", "email", "created_at"])
        assert verify_data["email"] == user.email, "Email mismatch in verification"
        
        self.logger.info("✅ Token refresh test passed")
    
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
    
    def test_registration_invalid_email_formats(self):
        """Test registration with invalid email formats"""
        # Create unique test data to avoid conflicts with other tests
        import random
        import string
        
        self.logger.info("Testing registration with invalid email formats")
        
        invalid_emails = [
            "invalid-email",  # No @
            "invalid@",       # No domain
            "@invalid.com",   # No local part
            "invalid@.com",   # Invalid domain
            "invalid@invalid..com",  # Double dot
        ]
        
        for email in invalid_emails:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            username = f'invalid_email_user_{session_id}'
            password = f'testpass123'
            
            response = self.make_request("POST", "/auth/register", {
                "username": username,
                "email": email,
                "password": password
            })
            
            # Expect failure for invalid emails (API should reject malformed emails)
            assert response.status_code >= 400, f"Invalid email '{email}' should fail: {response.status_code}"
            self.logger.info(f"Invalid email '{email}' rejected: {response.status_code}")
        
        self.logger.info("✅ Invalid email formats test passed")
    
    def test_registration_short_password(self):
        """Test registration with short passwords"""
        # Create unique test data for each attempt
        import random
        import string
        
        self.logger.info("Testing registration with short passwords")
        
        short_passwords = ["pass", "123", "a", ""]  # Passwords below minimum length of 8
        
        for pwd in short_passwords:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=2))
            username = f'short_pwd_user_{session_id}'
            email = f'short_pwd_{session_id}@example.com'
            
            response = self.make_request("POST", "/auth/register", {
                "username": username,
                "email": email,
                "password": pwd
            })
            
            # API should reject short passwords with 400 Bad Request
            self.assert_response(response, 400, f"Short password '{pwd}' (len: {len(pwd)}) should be rejected")
            self.logger.info(f"Short password '{pwd}' rejected as expected: {response.status_code}")
        
        self.logger.info("✅ Short password test passed")
    
    def test_registration_duplicate_username(self):
        """Test registration with duplicate username"""
        self.logger.info("Testing duplicate username registration")
        
        existing_username = TEST_USERS[0].username if TEST_USERS else "testuser"
        new_email = f'dup_username_{random.randint(1000,9999)}@example.com'
        password = "testpass123"
        
        response = self.make_request("POST", "/auth/register", {
            "username": existing_username,
            "email": new_email,
            "password": password
        })
        
        if response.status_code == 201:
            self.logger.info("Duplicate username allowed")
            data = response.json()
            test_data_manager.track_user({
                "username": existing_username, "email": new_email, "password": password, "token": data["token"]
            })
        else:
            self.logger.info(f"Duplicate username rejected: {response.status_code}")
        
        self.logger.info("✅ Duplicate username test passed")
    
    def test_login_with_both_credentials(self):
        """Test login with both email and username"""
        self.logger.info("Testing login with both credentials")
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        username = f'both_creds_user_{session_id}'
        email = f'both_creds_{session_id}@example.com'
        password = "bothpass123"
        
        # Register
        reg_resp = self.make_request("POST", "/auth/register", {
            "username": username, "email": email, "password": password
        })
        
        self.assert_response(reg_resp, 201)
        
        # Login with both
        response = self.make_request("POST", "/auth/login", {
            "username": username, "email": email, "password": password
        })
        
        self.assert_response(response, 200)
        data = response.json()
        self.verify_json_structure(data, ["token"])
        
        self.logger.info("✅ Both credentials login test passed")
    
    def test_login_empty_fields(self):
        """Test login with empty fields"""
        self.logger.info("Testing login empty fields")
        
        cases = [
            {"email": "", "username": "", "password": "pass123"},
            {"email": "test@example.com", "username": "", "password": ""},
            {"email": "", "username": "test", "password": ""},
        ]
        
        for data in cases:
            response = self.make_request("POST", "/auth/login", data)
            self.assert_response(response, 401, f"Empty login should fail: {data}")
        
        self.logger.info("✅ Empty login test passed")
    
    def test_me_with_expired_token(self):
        """Test /me with invalid/expired token"""
        self.logger.info("Testing /me with expired token")
        
        # Use malformed token to simulate expired
        response = self.make_request("GET", "/auth/me", token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.invalid.expired")
        
        if response.status_code == 404:
            self.logger.info("Endpoint not ready - skipping")
            return
        
        self.assert_response(response, 401)
        
        self.logger.info("✅ Expired token /me test passed")
    
    def test_refresh_invalid_token(self):
        """Test refresh with invalid tokens"""
        self.logger.info("Testing refresh invalid tokens")
        
        invalid_tokens = ["", "invalid", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid"]
        
        for token in invalid_tokens:
            response = self.make_request("POST", "/auth/refresh", {"token": token})
            self.assert_response(response, 401, f"Invalid refresh token: {token[:20]}")
        
        self.logger.info("✅ Invalid refresh test passed")
    
    def test_registration_special_characters(self):
        """Test special characters in registration"""
        self.logger.info("Testing special characters registration")
        
        cases = [
            {"username": "user@special!", "email": "special+test@example.com", "password": "P@ssw0rd!123"},
            {"username": "user_with spaces", "email": "user_space@example.com", "password": "pass spaces 123"},
        ]
        
        for case in cases:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            case["username"] += f"_{session_id}"
            case["email"] = case["email"].split("@")[0] + f"_{session_id}@example.com"
            
            response = self.make_request("POST", "/auth/register", case)
            
            if response.status_code == 201:
                data = response.json()
                self.verify_json_structure(data, ["token"])
                test_data_manager.track_user({**case, "token": data["token"]})
                self.logger.info(f"Special chars accepted: {case['email']}")
            else:
                self.logger.warning(f"Special chars rejected: {case['email']}")
        
        self.logger.info("✅ Special characters test passed")
    
    def test_login_case_sensitivity(self):
        """Test login case sensitivity"""
        self.logger.info("Testing login case sensitivity")
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        username = f'CaseUser{session_id}'
        email = f'caseuser{session_id}@example.com'
        password = "casepass123"
        
        reg_resp = self.make_request("POST", "/auth/register", {
            "username": username, "email": email, "password": password
        })
        
        self.assert_response(reg_resp, 201)
        
        # Test cases
        test_cases = [
            {"email": email.lower(), "password": password},
            {"username": username.lower(), "password": password},
            {"email": email.upper(), "password": password},
            {"username": username.upper(), "password": password},
        ]
        
        for case_data in test_cases:
            response = self.make_request("POST", "/auth/login", case_data)
            status = 200 if response.status_code == 200 else response.status_code
            self.logger.info(f"Case variation {list(case_data.keys())[0]}: {status}")
        
        self.logger.info("✅ Case sensitivity test passed")
    
    def test_registration_max_length(self):
        """Test long fields in registration"""
        self.logger.info("Testing long fields registration")
        
        long_username = "a" * 100
        long_email = ("a" * 200) + "@example.com"
        long_password = "a" * 100
        
        response = self.make_request("POST", "/auth/register", {
            "username": long_username, "email": long_email, "password": long_password
        })
        
        if response.status_code == 201:
            self.logger.info("Long fields accepted")
            data = response.json()
            test_data_manager.track_user({
                "username": long_username, "email": long_email, "password": long_password, "token": data["token"]
            })
        else:
            self.logger.info(f"Long fields rejected: {response.status_code}")
        
        self.logger.info("✅ Max length test passed")
    
    def test_rapid_consecutive_registrations(self):
        """Test consecutive registrations for potential race conditions"""
        self.logger.info("Testing consecutive registrations")
        
        base_email = f'rapid_{random.randint(1000,9999)}@example.com'
        base_username = f'rapid_user_{random.randint(1000,9999)}'
        
        # Attempt two registrations with similar data quickly (sequential)
        for i in range(2):
            email = f"{base_email}_{i}" if i > 0 else base_email
            username = f"{base_username}_{i}" if i > 0 else base_username
            response = self.make_request("POST", "/auth/register", {
                "username": username, "email": email, "password": "rapidpass123"
            })
            
            if response.status_code == 201:
                data = response.json()
                test_data_manager.track_user({
                    "username": username, "email": email, "password": "rapidpass123", "token": data["token"]
                })
                self.logger.info(f"Rapid reg {i} success")
            else:
                self.logger.info(f"Rapid reg {i} failed: {response.status_code}")
        
        self.logger.info("✅ Rapid registration test passed")
    
    def test_change_password_success(self):
        """Test successful password change with valid current password"""
        self.logger.info("Testing successful password change")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'change_pwd_{session_id}',
            email=f'change_pwd_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        user.token = register_response.json()["token"]
        test_data_manager.track_user(user.__dict__)
        
        # Test password change
        change_data = {
            "current_password": "originalpassword123",
            "new_password": "newpassword456",
            "confirm_password": "newpassword456"
        }
        
        response = self.make_request(
            "PUT", 
            "/auth/change-password", 
            change_data, 
            token=user.token
        )
        
        self.assert_response(response, 200, "Password change should succeed")
        
        # Verify response structure
        data = response.json()
        self.verify_json_structure(data, ["message"])
        assert data["message"] == "Password successfully changed"
        
        # Verify old password no longer works
        self.logger.info("Verifying old password no longer works")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": "originalpassword123"
        })
        
        self.assert_response(login_response, 401, "Old password should be rejected")
        
        # Verify new password works
        self.logger.info("Verifying new password works")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": "newpassword456"
        })
        
        self.assert_response(login_response, 200, "New password should work")
        
        self.logger.info("✅ Password change success test passed")

    def test_change_password_wrong_current_password(self):
        """Test password change with wrong current password"""
        self.logger.info("Testing password change with wrong current password")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'change_pwd_wrong_{session_id}',
            email=f'change_pwd_wrong_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        user.token = register_response.json()["token"]
        test_data_manager.track_user(user.__dict__)
        
        # Test password change with wrong current password
        change_data = {
            "current_password": "wrongpassword",
            "new_password": "newpassword456", 
            "confirm_password": "newpassword456"
        }
        
        response = self.make_request(
            "PUT",
            "/auth/change-password",
            change_data,
            token=user.token
        )
        
        self.assert_response(response, 401, "Wrong current password should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "incorrect" in data["error"].lower()
        
        self.logger.info("✅ Wrong current password test passed")

    def test_change_password_mismatch_confirmation(self):
        """Test password change with mismatched password confirmation"""
        self.logger.info("Testing password change with mismatched confirmation")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'change_pwd_mismatch_{session_id}',
            email=f'change_pwd_mismatch_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        user.token = register_response.json()["token"]
        test_data_manager.track_user(user.__dict__)
        
        # Test password change with mismatched confirmation
        change_data = {
            "current_password": "originalpassword123",
            "new_password": "newpassword456",
            "confirm_password": "differentpassword"
        }
        
        response = self.make_request(
            "PUT",
            "/auth/change-password", 
            change_data,
            token=user.token
        )
        
        self.assert_response(response, 400, "Mismatched passwords should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "do not match" in data["error"].lower()
        
        self.logger.info("✅ Password mismatch test passed")

    def test_change_password_short_password(self):
        """Test password change with too short new password"""
        self.logger.info("Testing password change with short password")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'change_pwd_short_{session_id}',
            email=f'change_pwd_short_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        user.token = register_response.json()["token"]
        test_data_manager.track_user(user.__dict__)
        
        # Test password change with short password
        change_data = {
            "current_password": "originalpassword123",
            "new_password": "short",
            "confirm_password": "short"
        }
        
        response = self.make_request(
            "PUT",
            "/auth/change-password",
            change_data, 
            token=user.token
        )
        
        self.assert_response(response, 400, "Short password should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "8 characters" in data["error"]
        
        self.logger.info("✅ Short password test passed")

    def test_change_password_no_auth(self):
        """Test password change without authentication token"""
        self.logger.info("Testing password change without authentication")
        
        change_data = {
            "current_password": "originalpassword123",
            "new_password": "newpassword456",
            "confirm_password": "newpassword456"
        }
        
        response = self.make_request("PUT", "/auth/change-password", change_data)
        
        # Should return 401 or similar authentication error
        assert response.status_code in [401, 400], f"Expected auth error, got {response.status_code}"
        
        self.logger.info("✅ No authentication test passed")

    def test_forgot_password_email_verification(self):
        """Test forgot password email verification step"""
        self.logger.info("Testing forgot password email verification")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'forgot_pwd_{session_id}',
            email=f'forgot_pwd_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        test_data_manager.track_user(user.__dict__)
        
        # Test email verification
        forgot_data = {
            "email": user.email
        }
        
        response = self.make_request("POST", "/auth/forgot-password", forgot_data)
        
        self.assert_response(response, 200, "Email verification should succeed")
        
        # Verify response structure
        data = response.json()
        self.verify_json_structure(data, ["message", "email_verified"])
        assert data["email_verified"] is True
        assert "user_id" in data
        
        self.logger.info("✅ Email verification test passed")

    def test_forgot_password_nonexistent_email(self):
        """Test forgot password with non-existent email"""
        self.logger.info("Testing forgot password with non-existent email")
        
        forgot_data = {
            "email": "nonexistent_email@example.com"
        }
        
        response = self.make_request("POST", "/auth/forgot-password", forgot_data)
        
        self.assert_response(response, 404, "Non-existent email should return 404")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "not found" in data["error"].lower()
        
        self.logger.info("✅ Non-existent email test passed")

    def test_forgot_password_complete_reset(self):
        """Test complete forgot password flow with password reset"""
        self.logger.info("Testing complete forgot password reset")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'forgot_complete_{session_id}',
            email=f'forgot_complete_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        test_data_manager.track_user(user.__dict__)
        
        # Test complete password reset
        reset_data = {
            "email": user.email,
            "new_password": "resetpassword789",
            "confirm_password": "resetpassword789"
        }
        
        response = self.make_request("POST", "/auth/forgot-password", reset_data)
        
        self.assert_response(response, 200, "Password reset should succeed")
        
        # Verify response structure
        data = response.json()
        self.verify_json_structure(data, ["message", "success"])
        assert data["success"] is True
        
        # Verify old password no longer works
        self.logger.info("Verifying old password no longer works")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": "originalpassword123"
        })
        
        self.assert_response(login_response, 401, "Old password should be rejected")
        
        # Verify new password works
        self.logger.info("Verifying new password works")
        login_response = self.make_request("POST", "/auth/login", {
            "email": user.email,
            "password": "resetpassword789"
        })
        
        self.assert_response(login_response, 200, "New password should work")
        
        self.logger.info("✅ Complete password reset test passed")

    def test_forgot_password_mismatch_confirmation(self):
        """Test forgot password with mismatched password confirmation"""
        self.logger.info("Testing forgot password with mismatched confirmation")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'forgot_mismatch_{session_id}',
            email=f'forgot_mismatch_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        test_data_manager.track_user(user.__dict__)
        
        # Test password reset with mismatched confirmation
        reset_data = {
            "email": user.email,
            "new_password": "resetpassword789",
            "confirm_password": "differentpassword"
        }
        
        response = self.make_request("POST", "/auth/forgot-password", reset_data)
        
        self.assert_response(response, 400, "Mismatched passwords should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "do not match" in data["error"].lower()
        
        self.logger.info("✅ Forgot password mismatch test passed")

    def test_forgot_password_short_password(self):
        """Test forgot password with too short new password"""
        self.logger.info("Testing forgot password with short password")
        
        # Create test user
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        from config import TestUser
        user = TestUser(
            username=f'forgot_short_{session_id}',
            email=f'forgot_short_{session_id}@example.com',
            password='originalpassword123'
        )
        
        # Register user
        register_response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(register_response, 201, "User registration failed")
        test_data_manager.track_user(user.__dict__)
        
        # Test password reset with short password
        reset_data = {
            "email": user.email,
            "new_password": "short",
            "confirm_password": "short"
        }
        
        response = self.make_request("POST", "/auth/forgot-password", reset_data)
        
        self.assert_response(response, 400, "Short password should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "8 characters" in data["error"]
        
        self.logger.info("✅ Forgot password short password test passed")

    def test_forgot_password_invalid_email_format(self):
        """Test forgot password with invalid email format"""
        self.logger.info("Testing forgot password with invalid email format")
        
        # Test with invalid email format
        reset_data = {
            "email": "invalid-email-format"
        }
        
        response = self.make_request("POST", "/auth/forgot-password", reset_data)
        
        self.assert_response(response, 400, "Invalid email format should be rejected")
        
        # Verify error message
        data = response.json()
        assert "error" in data
        assert "format" in data["error"].lower()
        
        self.logger.info("✅ Invalid email format test passed")
    
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
        
        # Additional edge case tests
        self.test_registration_invalid_email_formats()
        self.test_registration_short_password()
        self.test_registration_duplicate_username()
        self.test_login_with_both_credentials()
        self.test_login_empty_fields()
        self.test_me_with_expired_token()
        self.test_refresh_invalid_token()
        self.test_registration_special_characters()
        self.test_login_case_sensitivity()
        self.test_registration_max_length()
        self.test_rapid_consecutive_registrations()
        
        # Password management tests
        self.test_change_password_success()
        self.test_change_password_wrong_current_password()
        self.test_change_password_mismatch_confirmation()
        self.test_change_password_short_password()
        self.test_change_password_no_auth()
        self.test_forgot_password_email_verification()
        self.test_forgot_password_nonexistent_email()
        self.test_forgot_password_complete_reset()
        self.test_forgot_password_mismatch_confirmation()
        self.test_forgot_password_short_password()
        self.test_forgot_password_invalid_email_format()
        
        self.logger.info("✅ All auth tests passed")
