"""
Repository/Registry endpoint tests
"""
import random
import string
import time
import requests
from base_test import BaseTestCase
from config import TestUser


class RepositoryTests(BaseTestCase):
    """Test repository/registry functionality with auto-setup"""
    
    def __init__(self):
        super().__init__()
        self.owner = None
        self.org = None
        self.repo = None
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
        """Ensure test user and organization are set up before running tests"""
        if self.setup_attempted:
            return
        
        self.setup_attempted = True
        
        try:
            session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
            
            # Create fresh test user for repository tests
            user_data = {
                'username': f'repoowner_{session_id}',
                'email': f'repoowner_{session_id}@example.com',
                'password': 'repopass123',
                'full_name': 'Repository Owner'
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
                    self.owner = TestUser(user_data['username'], user_data['email'], user_data['password'])
                    self.owner.token = token
                    self.logger.info(f"✅ Setup repository owner: {user_data['username']}")
                    
                    # Create test organization for repositories
                    org_data = {
                        'name': f'repoorg_{session_id}',
                        'display_name': f'Repository Org {session_id}',
                        'description': f'Test organization for repositories'
                    }
                    
                    org_response = self.make_request("POST", "/api/organizations", org_data, token=self.owner.token)
                    if org_response and org_response.status_code == 201:
                        self.org = org_response.json().get('organization', org_response.json())
                        self.logger.info(f"✅ Setup test organization: {org_data['name']}")
            
        except Exception as e:
            self.logger.warning(f"⚠️ Repository setup failed: {e}")
            # Create mock objects to prevent AttributeError
            self.owner = TestUser("fallback_owner", "fallback@example.com", "pass")
            self.owner.token = "mock_token"
            self.org = {"name": "fallback_org", "id": "mock_id"}

    def test_repository_creation(self):
        """Test repository creation"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository creation test")
            return False
        
        # Generate unique repo name
        test_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data = {
            'name': f'testrepo_{test_id}',
            'description': f'Test repository created at {time.time()}',
            'is_public': True
        }
        
        org_name = self.org.get('name')
        response = self.make_request("POST", f"/api/organizations/{org_name}/repositories", repo_data, token=self.owner.token)
        
        if response and response.status_code == 201:
            data = response.json()
            self.repo = data.get('repository', data)
            
            assert self.repo.get('name') == repo_data['name']
            
            self.logger.info("✅ Repository creation test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository creation failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_listing(self):
        """Test repository listing"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository listing test")
            return False
        
        org_name = self.org.get('name')
        response = self.make_request("GET", f"/api/organizations/{org_name}/repositories", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            repos = data.get('repositories', data)
            
            assert isinstance(repos, list)
            
            self.logger.info("✅ Repository listing test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository listing failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_details(self):
        """Test repository details retrieval"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository details test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        response = self.make_request("GET", f"/api/organizations/{org_name}/repositories/{repo_name}", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            repo_info = data.get('repository', data)
            
            assert repo_info.get('name') == repo_name
            
            self.logger.info("✅ Repository details test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository details failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_update(self):
        """Test repository update"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository update test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        update_data = {
            'description': f'Updated repository description at {time.time()}',
            'is_public': False
        }
        
        response = self.make_request("PATCH", f"/api/organizations/{org_name}/repositories/{repo_name}", update_data, token=self.owner.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ Repository update test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository update failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_tags(self):
        """Test repository tags/versions"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository tags test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        response = self.make_request("GET", f"/api/organizations/{org_name}/repositories/{repo_name}/tags", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            tags = data.get('tags', data)
            
            assert isinstance(tags, list)
            
            self.logger.info("✅ Repository tags test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository tags failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_permissions(self):
        """Test repository permission checks"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository permissions test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        # Test owner can access
        response = self.make_request("GET", f"/api/organizations/{org_name}/repositories/{repo_name}", token=self.owner.token)
        
        if response and response.status_code == 200:
            self.logger.info("✅ Repository permissions test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository permissions failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_deletion(self):
        """Test repository deletion"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping repository deletion test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        response = self.make_request("DELETE", f"/api/organizations/{org_name}/repositories/{repo_name}", token=self.owner.token)
        
        if response and response.status_code in [200, 204]:
            self.logger.info("✅ Repository deletion test passed")
            return True
        
        self.logger.warning(f"⚠️ Repository deletion failed: {response.status_code if response else 'No response'}")
        return False

    def test_docker_registry_api(self):
        """Test Docker registry API functionality"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping Docker registry API test")
            return False
        
        # Ensure we have a repo
        if not self.repo:
            if not self.test_repository_creation():
                return False
        
        org_name = self.org.get('name')
        repo_name = self.repo.get('name')
        
        if not repo_name:
            return False
        
        # Test Docker registry manifest endpoint
        response = self.make_request("GET", f"/v2/{org_name}/{repo_name}/manifests/latest")
        
        if response and response.status_code in [200, 404]:  # 404 is ok for empty repo
            self.logger.info("✅ Docker registry API test passed")
            return True
        
        self.logger.warning(f"⚠️ Docker registry API failed: {response.status_code if response else 'No response'}")
        return False

    def test_repository_search(self):
        """Test repository search functionality"""
        if not self.owner or not self.owner.token:
            self.logger.warning("⚠️ Missing setup data, skipping repository search test")
            return False
        
        # Search for repositories
        response = self.make_request("GET", "/api/repositories/search?q=test", token=self.owner.token)
        
        if response and response.status_code == 200:
            data = response.json()
            repos = data.get('repositories', data)
            
            if isinstance(repos, list):
                self.logger.info("✅ Repository search test passed")
                return True
        
        self.logger.warning(f"⚠️ Repository search failed: {response.status_code if response else 'No response'}")
        return False

    def test_nonexistent_repository(self):
        """Test accessing non-existent repository"""
        if not self.owner or not self.owner.token or not self.org:
            self.logger.warning("⚠️ Missing setup data, skipping nonexistent repository test")
            return False
        
        org_name = self.org.get('name')
        response = self.make_request("GET", f"/api/organizations/{org_name}/repositories/nonexistent_repo_12345", token=self.owner.token)
        
        if response and response.status_code == 404:
            self.logger.info("✅ Nonexistent repository test passed")
            return True
        
        self.logger.warning(f"⚠️ Nonexistent repository test failed: {response.status_code if response else 'No response'}")
        return False

    def run_all_tests(self):
        """Run all repository tests"""
        self.logger.info("🚀 Starting Repository Tests")
        
        tests = [
            self.test_repository_creation,
            self.test_repository_listing,
            self.test_repository_details,
            self.test_repository_update,
            self.test_repository_tags,
            self.test_repository_permissions,
            self.test_nonexistent_repository,
            # Note: test_repository_deletion should be last as it deletes the repo
            self.test_repository_deletion
        ]
        
        passed = 0
        total = len(tests)
        
        for test in tests:
            try:
                if test():
                    passed += 1
            except Exception as e:
                self.logger.error(f"❌ {test.__name__} failed with exception: {e}")
        
        self.logger.info(f"📊 Repository Tests: {passed}/{total} passed")
        return passed == total


if __name__ == "__main__":
    import logging
    logging.basicConfig(level=logging.INFO)
    
    repo_tests = RepositoryTests()
    repo_tests.run_all_tests()
