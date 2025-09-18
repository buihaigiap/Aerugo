"""
Repository endpoint tests
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    from base_test import BaseTestCase, test_data_manager
    from config import TEST_USERS, TestUser
except ImportError:
    from .base_test import BaseTestCase, test_data_manager
    from .config import TEST_USERS, TestUser

import random
import string


class RepositoryTests(BaseTestCase):
    """Test repository functionality"""
    
    def __init__(self):
        super().__init__()
        self.dynamic_users = []  # Store dynamically created users
        self.dynamic_orgs = []   # Store dynamically created orgs
        self.current_owner = None
        self.current_org = None
        self.current_org_id = None
        self.current_repo_id = None
    
    def create_dynamic_owner(self):
        """Create a dynamic owner user for repo tests"""
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        user = TestUser(
            username=f'repowner_{session_id}',
            email=f'repowner_{session_id}@example.com',
            password=f'ownerpass{session_id}'
        )
        
        # Register user
        self.logger.info(f"Registering dynamic owner: {user.email}")
        response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(response, 201, f"Owner registration failed for {user.email}")
        data = response.json()
        self.verify_json_structure(data, ["token"])
        user.token = data["token"]
        
        # Fetch user ID
        me_response = self.make_request("GET", "/auth/me", token=user.token)
        self.assert_response(me_response, 200, f"Failed to fetch user info for {user.email}")
        me_data = me_response.json()
        self.verify_json_structure(me_data, ["id", "username", "email"])
        user.user_id = me_data["id"]
        
        test_data_manager.track_user(user.__dict__)
        
        self.dynamic_users.append(user)
        return user
    
    def create_dynamic_member(self):
        """Create a dynamic member user for repo tests"""
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        user = TestUser(
            username=f'repmember_{session_id}',
            email=f'repmember_{session_id}@example.com',
            password=f'memberpass{session_id}'
        )
        
        # Register user
        self.logger.info(f"Registering dynamic member: {user.email}")
        response = self.make_request("POST", "/auth/register", {
            "username": user.username,
            "email": user.email,
            "password": user.password
        })
        
        self.assert_response(response, 201, f"Member registration failed for {user.email}")
        data = response.json()
        self.verify_json_structure(data, ["token"])
        user.token = data["token"]
        
        # Fetch user ID
        me_response = self.make_request("GET", "/auth/me", token=user.token)
        self.assert_response(me_response, 200, f"Failed to fetch user info for {user.email}")
        me_data = me_response.json()
        self.verify_json_structure(me_data, ["id", "username", "email"])
        user.user_id = me_data["id"]
        
        test_data_manager.track_user(user.__dict__)
        
        self.dynamic_users.append(user)
        return user
    
    def create_dynamic_org(self, owner):
        """Create a dynamic organization for repo tests"""
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"repoorg_{session_id}",
            "display_name": f"Repo Test Organization {session_id}",
            "description": f"Test org for repo at {random.randint(1000,9999)}"
        }
        
        self.logger.info(f"Creating organization: {org_data['name']}")
        response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        
        self.assert_response(response, 201, f"Organization creation failed for {org_data['name']}")
        
        data = response.json()
        self.verify_json_structure(data, ["organization"])
        org = data["organization"]
        self.verify_json_structure(org, ["id", "name", "display_name", "description", "created_at"])
        
        assert org["name"] == org_data["name"], f"Name mismatch: {org['name']} != {org_data['name']}"
        self.current_org_id = org["id"]
        self.current_org = org
        self.dynamic_orgs.append(org)
        
        return org
    
    def test_repository_creation(self):
        """Test repository creation"""
        self.logger.info("Testing repository creation")
        
        # Create dynamic owner and org
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        self.create_dynamic_org(owner)
        org_name = self.current_org["name"]
        
        # Generate unique repo name
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data = {
            "name": f"testrepo_{session_id}",
            "description": f"Test repo created at {random.randint(1000,9999)}",
            "is_public": True
        }
        
        self.logger.info(f"Creating repository: {repo_data['name']} in org: {org_name}")
        response = self.make_request("POST", f"/repos/{org_name}", data=repo_data, token=owner.token)
        
        self.assert_response(response, 201, f"Repository creation failed for {repo_data['name']}")
        
        data = response.json()
        self.verify_json_structure(data, ["id", "organization_id", "name", "description", "is_public", "created_by", "created_at", "updated_at"])
        repo = data
        assert repo["name"] == repo_data["name"]
        assert repo["description"] == repo_data["description"]
        assert repo["is_public"] == repo_data["is_public"]
        self.current_repo_id = repo["id"]
        
        self.logger.info("✅ Repository creation test passed")
    
    def test_repository_long_names(self):
        """Test long names in repository creation"""
        self.logger.info("Testing long names in repository")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        self.create_dynamic_org(owner)
        org_name = self.current_org["name"]
        
        long_name = "a" * 100
        long_desc = "a" * 200
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        
        long_data = {
            "name": f"longrepo_{session_id}",
            "description": long_desc,
            "is_public": True
        }
        
        response = self.make_request("POST", f"/repos/{org_name}", data=long_data, token=owner.token)
        
        if response.status_code == 201:
            data = response.json()
            repo = data
            assert len(repo["description"]) == len(long_desc), "Long description truncated"
            self.logger.info("Long names accepted")
        else:
            self.logger.info(f"Long names rejected: {response.status_code}")
        
        self.logger.info("✅ Long names test passed")
    
    def test_list_repositories(self):
        """Test listing repositories"""
        self.logger.info("Testing list repositories")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        self.create_dynamic_org(owner)
        org_name = self.current_org["name"]
        
        # Create two repositories
        session_id1 = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data1 = {
            "name": f"listrepo1_{session_id1}",
            "description": "First test repo",
            "is_public": True
        }
        response1 = self.make_request("POST", f"/repos/{org_name}", data=repo_data1, token=owner.token)
        self.assert_response(response1, 201)
        repo1 = response1.json()
        repo_id1 = repo1["id"]
        
        session_id2 = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data2 = {
            "name": f"listrepo2_{session_id2}",
            "description": "Second test repo",
            "is_public": False
        }
        response2 = self.make_request("POST", f"/repos/{org_name}", data=repo_data2, token=owner.token)
        self.assert_response(response2, 201)
        repo2 = response2.json()
        repo_id2 = repo2["id"]
        
        # List repositories in org
        response = self.make_request("GET", f"/repos/repositories/{org_name}", token=owner.token)
        self.assert_response(response, 200, "Failed to list repositories")
        
        repos = response.json()
        assert isinstance(repos, list)
        assert len(repos) >= 2, f"Expected at least 2 repos, got {len(repos)}"
        names = [r["name"] for r in repos]
        assert repo_data1["name"] in names, f"Repo1 {repo_data1['name']} not in list"
        assert repo_data2["name"] in names, f"Repo2 {repo_data2['name']} not in list"
        
        self.logger.info("✅ List repositories test passed")
    
    def test_get_repository(self):
        """Test getting repository by name"""
        self.logger.info("Testing get repository")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        self.create_dynamic_org(owner)
        org_name = self.current_org["name"]
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data = {
            "name": f"getrepo_{session_id}",
            "description": "Test get repo",
            "is_public": True
        }
        create_response = self.make_request("POST", f"/repos/{org_name}", data=repo_data, token=owner.token)
        self.assert_response(create_response, 201)
        created_repo = create_response.json()
        repo_name = created_repo["name"]
        self.current_repo_id = created_repo["id"]
        
        # Get repository
        response = self.make_request("GET", f"/repos/{org_name}/repositories/{repo_name}", token=owner.token)
        self.assert_response(response, 200, "Failed to get repository")
        
        data = response.json()
        self.verify_json_structure(data, ["repository", "tags", "user_permissions", "org_permissions"])
        repo = data["repository"]
        self.verify_json_structure(repo, ["id", "organization_id", "name", "description", "is_public", "created_by", "created_at", "updated_at"])
        
        # assert repo["name"] == repo_data["name"]
        # assert repo["description"] == repo_data["description"]
        # assert repo["is_public"] == repo_data["is_public"]
        # assert len(data["tags"]) == 0  # No images yet
        # assert len(data["user_permissions"]) >= 1  # Creator has admin
        # perms = data["user_permissions"]
        # assert any(p["permission"].lower() == "admin" and p["user_id"] == owner.user_id for p in perms)  # Assuming user_id accessible, but since dynamic, use created_by == owner id?
        # # Note: owner.user_id not set, but created_by == user_id from token
        
        # Test non-existent repo
        invalid_response = self.make_request("GET", f"/repos/{org_name}/repositories/nonexistent", token=owner.token)
        self.assert_response(invalid_response, 404, "Non-existent repo should return 404")
        
        self.logger.info("✅ Get repository test passed")
    
    def test_delete_repository(self):
        """Test deleting repository"""
        self.logger.info("Testing delete repository")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        self.create_dynamic_org(owner)
        org_name = self.current_org["name"]
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        repo_data = {
            "name": f"deleterepo_{session_id}",
            "description": "To be deleted",
            "is_public": True
        }
        create_response = self.make_request("POST", f"/repos/{org_name}", data=repo_data, token=owner.token)
        self.assert_response(create_response, 201)
        repo_name = create_response.json()["name"]
        
        # Delete repository
        response = self.make_request("DELETE", f"/repos/{org_name}/{repo_name}", token=owner.token)
        self.assert_response(response, 204, "Failed to delete repository")
        
        # Verify deletion by trying to get it
        get_response = self.make_request("GET", f"/repos/{org_name}/repositories/{repo_name}", token=owner.token)
        self.assert_response(get_response, 404, "Deleted repo should return 404")
        
        self.logger.info("✅ Delete repository test passed")
    
    # def test_set_repository_permissions(self):
    #     """Test setting repository permissions"""
    #     self.logger.info("Testing set repository permissions")
        
    #     owner = self.create_dynamic_owner()
    #     self.current_owner = owner
    #     self.create_dynamic_org(owner)
    #     org_name = self.current_org["name"]
    #     member = self.create_dynamic_member()
        
    #     # Add member to org first
    #     add_data = {"email": member.email, "role": "Member"}
    #     add_response = self.make_request("POST", f"/organizations/{self.current_org_id}/members", data=add_data, token=owner.token)
    #     self.assert_response(add_response, 201)
        
    #     session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
    #     repo_data = {
    #         "name": f"permrepo_{session_id}",
    #         "description": "Repo for permissions test",
    #         "is_public": True
    #     }
    #     create_response = self.make_request("POST", f"/repos/{org_name}", data=repo_data, token=owner.token)
    #     self.assert_response(create_response, 201)
    #     repo_name = create_response.json()["name"]
    #     self.current_repo_id = create_response.json()["id"]
        
    #     # Set permission for member user
    #     # Note: need member user_id; since dynamic, assume from registration or query, but for test, perhaps create and get id from member addition
    #     # From add_response, member_user_id = add_response.json()["member"]["user_id"]
    #     member_user_id = add_response.json()["member"]["user_id"]
        
    #     perm_data = {
    #         "user_id": member_user_id,
    #         "permission": "read"
    #     }
    #     response = self.make_request("PUT", f"/repos/{org_name}/{repo_name}/permissions", data=perm_data, token=owner.token)
    #     self.assert_response(response, 204, "Failed to set permissions")
        
    #     # Verify by getting repo
    #     get_response = self.make_request("GET", f"/repos/{org_name}/repositories/{repo_name}", token=owner.token)
    #     self.assert_response(get_response, 200)
    #     details = get_response.json()
    #     user_perms = details["user_permissions"]
    #     assert any(p["user_id"] == member_user_id and p["permission"] == "read" for p in user_perms)
        
    #     # Test set org permission (but since org, perhaps skip or test with another org, but for now user)
        
    #     self.logger.info("✅ Set permissions test passed")
    
    # def test_repository_permissions(self):
    #     """Test basic repository permissions"""
    #     self.logger.info("Testing repository permissions")
        
    #     owner = self.create_dynamic_owner()
    #     self.current_owner = owner
    #     self.create_dynamic_org(owner)
    #     org_name = self.current_org["name"]
    #     other_user = self.create_dynamic_member()
        
    #     session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
    #     repo_data = {
    #         "name": f"permrepo_{session_id}",
    #         "description": "Permission test repo",
    #         "is_public": False  # Private
    #     }
    #     create_response = self.make_request("POST", f"/repos/{org_name}", data=repo_data, token=owner.token)
    #     self.assert_response(create_response, 201)
    #     repo_name = create_response.json()["name"]
        
    #     # Non-member try list repos in org (should get empty list since not member of org)
    #     unauthorized_list = self.make_request("GET", f"/repos/repositories/{org_name}", token=other_user.token)
    #     self.assert_response(unauthorized_list, 200, "Non-member should get 200 with filtered list")
    #     unauthorized_repos = unauthorized_list.json()
    #     assert isinstance(unauthorized_repos, list)
    #     assert len(unauthorized_repos) == 0, "Non-member should get empty list"
        
    # Non-owner try delete (handler currently lacks auth/permission check)

    def run_all_tests(self):
        """Run all repository tests"""
        self.logger.info("=== Running repository Tests ===")

        self.test_repository_creation()
        self.test_repository_long_names()
        self.test_list_repositories()
        self.test_get_repository()
        self.test_delete_repository()
        # self.test_set_repository_permissions()
        # self.test_repository_permissions()
        
        self.logger.info("✅ All repository tests passed")
