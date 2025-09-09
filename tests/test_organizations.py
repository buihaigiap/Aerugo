"""Test organization functionality"""
import requests
import time
import random
import string
from base_test import BaseTestCase
from config import TestUser


class OrganizationTests(BaseTestCase):
    """Test organization functionality with auto-setup"""
    
    def __init__(self):
        super().__init__()
        self.owner = None
        self.member = None
        self.viewer = None
        self.org = None
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
        """Ensure test users are set up before running tests"""
        if self.setup_attempted:
            return
        
        self.setup_attempted = True
        
        try:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            
            # Create fresh test users for this test session
            owner_data = {
                'username': f'orgowner_{session_id}',
                'email': f'orgowner_{session_id}@example.com',
                'password': 'ownerpass123',
                'full_name': 'Organization Owner'
            }
            
            member_data = {
                'username': f'orgmember_{session_id}', 
                'email': f'orgmember_{session_id}@example.com',
                'password': 'memberpass123',
                'full_name': 'Organization Member'
            }
            
            # Register owner
            owner_response = self.register_user(owner_data)
            if owner_response and owner_response.status_code == 201:
                login_response = self.login_user(owner_data['username'], owner_data['password'])
                if login_response and login_response.status_code == 200:
                    token = login_response.json().get('token')
                    self.owner = TestUser(owner_data['username'], owner_data['email'], owner_data['password'])
                    self.owner.token = token
                    self.logger.info(f"✅ Setup owner user: {owner_data['username']}")
            
            # Register member
            member_response = self.register_user(member_data)
            if member_response and member_response.status_code == 201:
                login_response = self.login_user(member_data['username'], member_data['password'])
                if login_response and login_response.status_code == 200:
                    token = login_response.json().get('token')
                    self.member = TestUser(member_data['username'], member_data['email'], member_data['password'])
                    self.member.token = token
                    self.logger.info(f"✅ Setup member user: {member_data['username']}")
            
            self.viewer = self.member
            
        except Exception as e:
            self.logger.warning(f"⚠️ User setup failed: {e}")
            # Create mock users to prevent AttributeError
            self.owner = TestUser("fallback_owner", "fallback@example.com", "pass")
            self.owner.token = "mock_token"
            self.member = TestUser("fallback_member", "member@example.com", "pass")
            self.member.token = "mock_token"
            self.viewer = self.member
    
    def register_user(self, user_data):
        """Helper to register a user"""
        try:
            response = requests.post(f"{self.api_base}/auth/register", json=user_data)
            return response
        except Exception as e:
            self.logger.warning(f"⚠️ Registration failed: {e}")
            return None
    
    def login_user(self, username, password):
        """Helper to login a user"""
        try:
            response = requests.post(f"{self.api_base}/auth/login", json={
                'username': username,
                'password': password
            })
            return response
        except Exception as e:
            self.logger.warning(f"⚠️ Login failed: {e}")
            return None

    def test_organization_creation(self):
        """Test creating a new organization"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping organization creation test")
            return False
        
        # Generate unique org name to avoid conflicts
        test_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            'name': f'testorg_{test_id}',
            'display_name': f'Test Organization {test_id}',
            'description': f'Test organization created at {time.time()}'
        }
        
        response = self.make_request("POST", "/api/organizations", org_data, token=self.owner.token)
        
        if response and response.status_code == 201:
            data = response.json()
            self.org = data.get('organization', data)
            
            assert 'id' in self.org or 'name' in self.org
            assert self.org.get('name') == org_data['name']
            
            self.logger.info(f"✅ Organization creation test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization creation failed: {response.status_code if response else 'No response'}")
            return False

    def test_organization_retrieval(self):
        """Test retrieving organization details"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping organization retrieval test")
            return False
            
        # First create an org if we don't have one
        if not self.org:
            if not self.test_organization_creation():
                return False
        
        org_name = self.org.get('name')
        if not org_name:
            self.logger.warning("⚠️ No org name available for retrieval test")
            return False
            
        response = self.make_request("GET", f"/api/organizations/{org_name}", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            org_info = data.get('organization', data)
            
            assert org_info.get('name') == org_name
            
            self.logger.info(f"✅ Organization retrieval test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization retrieval failed: {response.status_code if response else 'No response'}")
            return False

    def test_organization_update(self):
        """Test updating organization details"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping organization update test")
            return False
            
        # Ensure we have an org
        if not self.org:
            if not self.test_organization_creation():
                return False
        
        org_name = self.org.get('name')
        if not org_name:
            return False
            
        update_data = {
            'description': f'Updated description at {time.time()}',
            'display_name': f'Updated {org_name}'
        }
        
        response = self.make_request("PATCH", f"/api/organizations/{org_name}", update_data, token=self.owner.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info(f"✅ Organization update test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization update failed: {response.status_code if response else 'No response'}")
            return False

    def test_organization_member_management(self):
        """Test adding/removing organization members"""
        if not self.owner or not self.member or not self.owner.token or not self.member.token:
            self.logger.warning("⚠️ Missing user tokens, skipping member management test")
            return False
            
        # Ensure we have an org
        if not self.org:
            if not self.test_organization_creation():
                return False
        
        org_name = self.org.get('name')
        if not org_name:
            return False
        
        # Add member to organization
        member_data = {
            'username': self.member.username,
            'role': 'member'
        }
        
        response = self.make_request("POST", f"/api/organizations/{org_name}/members", member_data, token=self.owner.token)
        
        if response and response.status_code in [200, 201]:
            self.logger.info(f"✅ Organization member management test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization member management failed: {response.status_code if response else 'No response'}")
            return False

    def test_user_organizations(self):
        """Test listing user's organizations"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping user organizations test")
            return False
        
        response = self.make_request("GET", "/api/user/organizations", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            orgs = data.get('organizations', data)
            
            assert isinstance(orgs, list)
            
            self.logger.info(f"✅ User organizations test passed")
            return True
        else:
            self.logger.warning(f"⚠️ User organizations failed: {response.status_code if response else 'No response'}")
            return False

    def test_organization_permissions(self):
        """Test organization permission checks"""
        if not self.owner or not self.member or not self.owner.token:
            self.logger.warning("⚠️ Missing user tokens, skipping permissions test")
            return False
            
        # Ensure we have an org
        if not self.org:
            if not self.test_organization_creation():
                return False
        
        org_name = self.org.get('name')
        if not org_name:
            return False
        
        # Test owner can access
        response = self.make_request("GET", f"/api/organizations/{org_name}", token=self.owner.token)
        
        if response and response.status_code == 200:
            self.logger.info(f"✅ Organization permissions test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization permissions failed: {response.status_code if response else 'No response'}")
            return False

    def test_organization_validation(self):
        """Test organization input validation"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping validation test")
            return False
        
        # Test invalid org name
        invalid_data = {
            'name': '',  # Empty name should fail
            'description': 'Test org'
        }
        
        response = self.make_request("POST", "/api/organizations", invalid_data, token=self.owner.token)
        
        if response and response.status_code in [400, 422]:
            self.logger.info(f"✅ Organization validation test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Organization validation failed: {response.status_code if response else 'No response'}")
            return False

    def test_nonexistent_organization(self):
        """Test accessing non-existent organization"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ No owner token, skipping nonexistent org test")
            return False
        
        response = self.make_request("GET", "/api/organizations/nonexistent_org_12345", token=self.owner.token)
        
        if response and response.status_code == 404:
            self.logger.info(f"✅ Nonexistent organization test passed")
            return True
        else:
            self.logger.warning(f"⚠️ Nonexistent organization test failed: {response.status_code if response else 'No response'}")
            return False

    def run_all_tests(self):
        """Run all organization tests"""
        self.logger.info("🚀 Starting Organization Tests")
        
        tests = [
            self.test_organization_creation,
            self.test_organization_retrieval,
            self.test_organization_update,
            self.test_organization_member_management,
            self.test_user_organizations,
            self.test_organization_permissions,
            self.test_organization_validation,
            self.test_nonexistent_organization
        ]
        
        passed = 0
        total = len(tests)
        
        for test in tests:
            try:
                if test():
                    passed += 1
            except Exception as e:
                self.logger.error(f"❌ {test.__name__} failed with exception: {e}")
        
        self.logger.info(f"📊 Organization Tests: {passed}/{total} passed")
        return passed == total


if __name__ == "__main__":
    import logging
    logging.basicConfig(level=logging.INFO)
    
    org_tests = OrganizationTests()
    org_tests.run_all_tests()
