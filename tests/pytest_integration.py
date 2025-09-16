#!/usr/bin/env python3

"""
Pytest wrapper for Aerugo integration tests
Converts BaseTestCase methods to pytest-compatible functions
"""

import sys
import os
import pytest
import time

# Add tests directory to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

# Import test classes
from test_health import HealthTests
from test_auth import AuthTests
from test_organizations import OrganizationTests  
from test_users import UserTests
from test_repositories import RepositoryTests

# Global test instances - will be recreated for each test run
health_tests = None
auth_tests = None
org_tests = None
user_tests = None
repo_tests = None

def setup_module():
    """Setup test instances - called once per module"""
    global health_tests, auth_tests, org_tests, user_tests, repo_tests
    
    print("\n🔧 Setting up test instances...")
    
    # Create fresh instances for each test run
    health_tests = HealthTests()
    auth_tests = AuthTests()
    org_tests = OrganizationTests()
    user_tests = UserTests()
    repo_tests = RepositoryTests()
    
    print("✅ Test instances ready")

@pytest.fixture(autouse=True)
def setup_test_data():
    """Setup fresh test data before each test"""
    # Ensure we have fresh instances
    if auth_tests is None:
        setup_module()
    
    # Reset test data to avoid conflicts
    # try:
    #     from base_test import test_data_manager
    #     test_data_manager.cleanup_test_data()
    #     time.sleep(0.1)  # Small delay to ensure cleanup
    # except Exception as e:
    #     print(f"⚠️ Cleanup warning: {e}")
    
    yield
    
    # Cleanup after test
    # try:
    #     from base_test import test_data_manager  
    #     test_data_manager.cleanup_test_data()
    # except Exception as e:
    #     print(f"⚠️ Post-test cleanup warning: {e}")

# Health Tests
def test_health_check():
    health_tests.test_health_check()

def test_health_check_headers():
    health_tests.test_health_check_headers()

def test_health_check_methods():
    health_tests.test_health_check_methods()

# Auth Tests  
def test_user_registration():
    auth_tests.test_user_registration()

def test_user_login():
    auth_tests.test_user_login()

def test_protected_endpoint():
    auth_tests.test_protected_endpoint()

def test_invalid_login():
    auth_tests.test_invalid_login()

def test_invalid_token():
    auth_tests.test_invalid_token()

def test_registration_validation():
    auth_tests.test_registration_validation()

def test_token_refresh():
    auth_tests.test_token_refresh()

def test_logout():
    auth_tests.test_logout()

# Additional Auth Edge Case Tests
def test_registration_invalid_email_formats():
    auth_tests.test_registration_invalid_email_formats()

def test_registration_short_password():
    auth_tests.test_registration_short_password()

def test_registration_duplicate_username():
    auth_tests.test_registration_duplicate_username()

def test_login_with_both_credentials():
    auth_tests.test_login_with_both_credentials()

def test_login_empty_fields():
    auth_tests.test_login_empty_fields()

def test_me_with_expired_token():
    auth_tests.test_me_with_expired_token()

def test_refresh_invalid_token():
    auth_tests.test_refresh_invalid_token()

def test_registration_special_characters():
    auth_tests.test_registration_special_characters()

def test_login_case_sensitivity():
    auth_tests.test_login_case_sensitivity()

def test_registration_max_length():
    auth_tests.test_registration_max_length()

def test_rapid_consecutive_registrations():
    auth_tests.test_rapid_consecutive_registrations()

# Organization Tests  
def test_organization_creation():
    org_tests.test_organization_creation()

def test_organization_long_names():
    org_tests.test_organization_long_names()    
    
def test_list_organizations():
    org_tests.test_list_organizations() 

def test_get_organization():
    org_tests.test_get_organization() 

def test_update_organization():
    org_tests.test_update_organization() 

def test_delete_organization():
    org_tests.test_delete_organization() 

def test_add_organization_member():
    org_tests.test_add_organization_member() 

def test_get_organization_members():
    org_tests.test_get_organization_members() 

def test_update_member_role():
    org_tests.test_update_member_role() 

def test_remove_organization_member():
    org_tests.test_remove_organization_member() 

def test_organization_permissions():
    org_tests.test_organization_permissions()     

# User Tests
def test_user_profile_retrieval():
    user_tests.test_user_profile_retrieval()

def test_user_profile_update():
    user_tests.test_user_profile_update()

def test_user_public_profile():
    user_tests.test_user_public_profile()

def test_user_search():
    user_tests.test_user_search()

def test_user_avatar_upload():
    user_tests.test_user_avatar_upload()

def test_user_password_change():
    user_tests.test_user_password_change()

def test_user_account_deletion():
    user_tests.test_user_account_deletion()

def test_user_email_verification():
    user_tests.test_user_email_verification()

def test_user_preferences():
    user_tests.test_user_preferences()

# Repository Tests
def test_repository_creation():
    repo_tests.test_repository_creation()

def test_repository_long_names():
    repo_tests.test_repository_long_names()

def test_list_repositories():
    repo_tests.test_list_repositories()

def test_get_repository():
    repo_tests.test_get_repository()

def test_delete_repository():
    repo_tests.test_delete_repository()

# def test_set_repository_permissions():
#     repo_tests.test_set_repository_permissions()

# def test_repository_permissions():
#     repo_tests.test_repository_permissions()

if __name__ == "__main__":
    # Can run this file directly with pytest
    pytest.main([__file__, "-v"])
