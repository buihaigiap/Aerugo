"""
Organization endpoint tests
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


class OrganizationTests(BaseTestCase):
    """Test organization functionality"""
    
    def __init__(self):
        super().__init__()
        self.dynamic_users = []  # Store dynamically created users
        self.dynamic_orgs = []   # Store dynamically created orgs
        self.current_owner = None
        self.current_org_id = None
    
    def create_dynamic_owner(self):
        """Create a dynamic owner user for org tests"""
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        user = TestUser(
            username=f'orgowner_{session_id}',
            email=f'orgowner_{session_id}@example.com',
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
        test_data_manager.track_user(user.__dict__)
        
        self.dynamic_users.append(user)
        return user
    
    def create_dynamic_member(self):
        """Create a dynamic member user for org tests"""
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        user = TestUser(
            username=f'orgmember_{session_id}',
            email=f'orgmember_{session_id}@example.com',
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
        test_data_manager.track_user(user.__dict__)
        
        self.dynamic_users.append(user)
        return user
    
    def test_organization_creation(self):
        """Test organization creation"""
        self.logger.info("Testing organization creation")
        
        # Create dynamic owner
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        # Generate unique org name
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"testorg_{session_id}",
            "display_name": f"Test Organization {session_id}",
            "description": f"Test org created at {random.randint(1000,9999)}"
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
        
        self.logger.info("✅ Organization creation test passed")
    
    def test_organization_long_names(self):
        """Test long names in organization creation"""
        self.logger.info("Testing long names in organization")
        
        owner = self.create_dynamic_owner()
        long_name = "a" * 100
        long_display = "a" * 200
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        
        long_data = {
            "name": f"longorg_{session_id}",
            "display_name": long_display,
            "description": long_name
        }
        
        response = self.make_request("POST", "/organizations", data=long_data, token=owner.token)
        
        if response.status_code == 201:
            data = response.json()
            org = data["organization"]
            assert len(org["display_name"]) == len(long_display), "Long display name truncated"
            self.logger.info("Long names accepted")
        else:
            self.logger.info(f"Long names rejected: {response.status_code}")
        
        self.logger.info("✅ Long names test passed")
    
    def test_list_organizations(self):
        """Test listing user's organizations"""
        self.logger.info("Testing list organizations")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        # Create two organizations
        session_id1 = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data1 = {
            "name": f"listorg1_{session_id1}",
            "display_name": f"List Org 1 {session_id1}",
            "description": "First test org"
        }
        response1 = self.make_request("POST", "/organizations", data=org_data1, token=owner.token)
        self.assert_response(response1, 201)
        org1 = response1.json()["organization"]
        org_id1 = org1["id"]
        
        session_id2 = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data2 = {
            "name": f"listorg2_{session_id2}",
            "display_name": f"List Org 2 {session_id2}",
            "description": "Second test org"
        }
        response2 = self.make_request("POST", "/organizations", data=org_data2, token=owner.token)
        self.assert_response(response2, 201)
        org2 = response2.json()["organization"]
        org_id2 = org2["id"]
        
        # List organizations
        response = self.make_request("GET", "/organizations", token=owner.token)
        self.assert_response(response, 200, "Failed to list organizations")
        
        data = response.json()
        self.verify_json_structure(data, ["organizations"])
        orgs = data["organizations"]
        
        assert len(orgs) >= 2, f"Expected at least 2 orgs, got {len(orgs)}"
        names = [o["name"] for o in orgs]
        assert org_data1["name"] in names, f"Org1 {org_data1['name']} not in list"
        assert org_data2["name"] in names, f"Org2 {org_data2['name']} not in list"
        
        self.logger.info("✅ List organizations test passed")
    
    def test_get_organization(self):
        """Test getting organization by ID"""
        self.logger.info("Testing get organization")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"getorg_{session_id}",
            "display_name": f"Get Org {session_id}",
            "description": "Test get org"
        }
        response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(response, 201)
        created_org = response.json()["organization"]
        org_id = created_org["id"]
        self.current_org_id = org_id
        
        # Get organization
        response = self.make_request("GET", f"/organizations/{org_id}")
        self.assert_response(response, 200, "Failed to get organization")
        
        data = response.json()
        self.verify_json_structure(data, ["organization"])
        org = data["organization"]
        self.verify_json_structure(org, ["id", "name", "display_name", "description", "created_at"])
        
        assert org["id"] == org_id
        assert org["name"] == org_data["name"]
        assert org["display_name"] == org_data["display_name"]
        
        # Test non-existent org
        invalid_response = self.make_request("GET", "/organizations/999999")
        self.assert_response(invalid_response, 404, "Non-existent org should return 404")
        
        self.logger.info("✅ Get organization test passed")
    
    def test_update_organization(self):
        """Test updating organization"""
        self.logger.info("Testing update organization")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"updateorg_{session_id}",
            "display_name": f"Update Org {session_id}",
            "description": "Original description"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        self.current_org_id = org_id
        
        # Update data
        update_data = {
            "display_name": f"Updated Org {session_id}",
            "description": "Updated description",
            "website_url": "https://example.com",
            "avatar_url": "https://example.com/avatar.png"
        }
        
        response = self.make_request("PUT", f"/organizations/{org_id}", data=update_data, token=owner.token)
        self.assert_response(response, 200, "Failed to update organization")
        
        data = response.json()
        self.verify_json_structure(data, ["organization"])
        updated_org = data["organization"]
        
        assert updated_org["display_name"] == update_data["display_name"]
        assert updated_org["description"] == update_data["description"]
        assert updated_org["website_url"] == update_data["website_url"]
        assert updated_org["avatar_url"] == update_data["avatar_url"]
        
        # Verify partial update (only description)
        partial_update = {"description": "Partial update desc"}
        response = self.make_request("PUT", f"/organizations/{org_id}", data=partial_update, token=owner.token)
        self.assert_response(response, 200)
        
        partial_data = response.json()["organization"]
        assert partial_data["description"] == "Partial update desc"
        assert partial_data["display_name"] == update_data["display_name"]  # Unchanged
        
        self.logger.info("✅ Update organization test passed")
    
    def test_delete_organization(self):
        """Test deleting organization"""
        self.logger.info("Testing delete organization")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"deleteorg_{session_id}",
            "display_name": f"Delete Org {session_id}",
            "description": "To be deleted"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        
        # Delete organization
        response = self.make_request("DELETE", f"/organizations/{org_id}", token=owner.token)
        self.assert_response(response, 204, "Failed to delete organization")
        
        # Verify deletion by trying to get it
        get_response = self.make_request("GET", f"/organizations/{org_id}")
        self.assert_response(get_response, 404, "Deleted org should return 404")
        
        self.logger.info("✅ Delete organization test passed")
    
    def test_add_organization_member(self):
        """Test adding member to organization"""
        self.logger.info("Testing add organization member")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        member = self.create_dynamic_member()
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"memberorg_{session_id}",
            "display_name": f"Member Org {session_id}",
            "description": "Org for member test"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        self.current_org_id = org_id
        
        # Add member
        add_data = {
            "email": member.email,
            "role": "Member"
        }
        response = self.make_request("POST", f"/organizations/{org_id}/members", data=add_data, token=owner.token)
        self.assert_response(response, 201, "Failed to add member")
        
        data = response.json()
        self.verify_json_structure(data, ["member"])
        added_member = data["member"]
        self.verify_json_structure(added_member, ["id", "user_id", "role", "username", "email"])
        
        assert added_member["email"] == member.email
        assert added_member["role"] == "member"
        member_id = added_member["user_id"]
        
        # Try to add existing member
        duplicate_response = self.make_request("POST", f"/organizations/{org_id}/members", data=add_data, token=owner.token)
        self.assert_response(duplicate_response, 400, "Adding existing member should fail")
        
        self.logger.info("✅ Add member test passed")
    
    def test_get_organization_members(self):
        """Test getting organization members"""
        self.logger.info("Testing get organization members")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        member = self.create_dynamic_member()
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"membersorg_{session_id}",
            "display_name": f"Members Org {session_id}",
            "description": "Org for members list"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        self.current_org_id = org_id
        
        # Add member
        add_data = {"email": member.email, "role": "Member"}
        add_response = self.make_request("POST", f"/organizations/{org_id}/members", data=add_data, token=owner.token)
        self.assert_response(add_response, 201)
        
        # Get members as owner
        response = self.make_request("GET", f"/organizations/{org_id}/members", token=owner.token)
        self.assert_response(response, 200, "Failed to get members")
        
        data = response.json()
        self.verify_json_structure(data, ["members"])
        members = data["members"]
        assert len(members) == 2, f"Expected 2 members, got {len(members)}"
        
        # Check both owner and member present
        emails = [m["email"] for m in members]
        assert owner.email in emails
        assert member.email in emails
        
        # Test as member (login as member)
        member_response = self.make_request("GET", f"/organizations/{org_id}/members", token=member.token)
        self.assert_response(member_response, 200, "Member should access members list")
        
        self.logger.info("✅ Get members test passed")
    
    def test_update_member_role(self):
        """Test updating member role"""
        self.logger.info("Testing update member role")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        member = self.create_dynamic_member()
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"roleorg_{session_id}",
            "display_name": f"Role Org {session_id}",
            "description": "Org for role update"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        self.current_org_id = org_id
        
        # Add member
        add_data = {"email": member.email, "role": "Member"}
        add_response = self.make_request("POST", f"/organizations/{org_id}/members", data=add_data, token=owner.token)
        self.assert_response(add_response, 201)
        member_user_id = add_response.json()["member"]["user_id"]
        
        # Update role to admin
        update_data = {"role": "Admin"}
        response = self.make_request("PUT", f"/organizations/{org_id}/members/{member_user_id}", data=update_data, token=owner.token)
        self.assert_response(response, 200, "Failed to update role")
        
        data = response.json()
        self.verify_json_structure(data, ["member"])
        updated_member = data["member"]
        assert updated_member["role"] == "admin"
        
        self.logger.info("✅ Update member role test passed")
    
    def test_remove_organization_member(self):
        """Test removing organization member"""
        self.logger.info("Testing remove organization member")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        member = self.create_dynamic_member()
        
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"removeorg_{session_id}",
            "display_name": f"Remove Org {session_id}",
            "description": "Org for remove member"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        self.current_org_id = org_id
        
        # Add member
        add_data = {"email": member.email, "role": "Member"}
        add_response = self.make_request("POST", f"/organizations/{org_id}/members", data=add_data, token=owner.token)
        self.assert_response(add_response, 201)
        member_user_id = add_response.json()["member"]["user_id"]
        
        # Remove member
        response = self.make_request("DELETE", f"/organizations/{org_id}/members/{member_user_id}", token=owner.token)
        self.assert_response(response, 204, "Failed to remove member")
        
        # Verify removal
        get_members_response = self.make_request("GET", f"/organizations/{org_id}/members", token=owner.token)
        self.assert_response(get_members_response, 200)
        members = get_members_response.json()["members"]
        member_emails = [m["email"] for m in members]
        assert member.email not in member_emails
        
        # Test self-removal (but since member not added back, skip or add another)
        self.logger.info("✅ Remove member test passed")
    
    def test_organization_permissions(self):
        """Test basic organization permissions"""
        self.logger.info("Testing organization permissions")
        
        owner = self.create_dynamic_owner()
        self.current_owner = owner
        
        # Create org as owner
        session_id = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        org_data = {
            "name": f"permorg_{session_id}",
            "display_name": f"Perm Org {session_id}",
            "description": "Permission test org"
        }
        create_response = self.make_request("POST", "/organizations", data=org_data, token=owner.token)
        self.assert_response(create_response, 201)
        org_id = create_response.json()["organization"]["id"]
        
        # Non-owner try update (use another user)
        other_user = self.create_dynamic_member()
        update_data = {"display_name": "Unauthorized update"}
        unauthorized_response = self.make_request("PUT", f"/organizations/{org_id}", data=update_data, token=other_user.token)
        self.assert_response(unauthorized_response, 400, "Non-owner should not update org")  # Or 403 if implemented
        
        # Non-member try get members
        members_response = self.make_request("GET", f"/organizations/{org_id}/members", token=other_user.token)
        self.assert_response(members_response, 400, "Non-member should not access members")  # Or 403
        
        self.logger.info("✅ Permissions test passed")
    
    def run_all_tests(self):
        """Run all organization tests"""
        self.logger.info("=== Running Organization Tests ===")
        
        self.test_organization_creation()
        self.test_organization_long_names()
        self.test_list_organizations()
        self.test_get_organization()
        self.test_update_organization()
        self.test_delete_organization()
        self.test_add_organization_member()
        self.test_get_organization_members()
        self.test_update_member_role()
        self.test_remove_organization_member()
        self.test_organization_permissions()
        
        self.logger.info("✅ All organization tests passed")
