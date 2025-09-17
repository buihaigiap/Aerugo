#!/usr/bin/env python3
"""
Test script for delete repository API
"""
import requests
import json

BASE_URL = "http://localhost:8080"
JWT_TOKEN = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyMCIsImV4cCI6MTc1ODE1ODA5N30.RAaR4DoZxqYNeNCirrfd98H2MdH2wSNDeOpT_9PSk4Y"

def test_delete_repository():
    """Test deleting a repository"""
    
    headers = {
        "Authorization": f"Bearer {JWT_TOKEN}",
        "Content-Type": "application/json"
    }
    
    # First create a test repository
    print("Creating test repository...")
    create_data = {
        "name": "test-repo-delete",
        "description": "Test repository for delete testing",
        "is_public": False
    }
    
    response = requests.post(
        f"{BASE_URL}/api/v1/repos/testorg",
        headers=headers,
        json=create_data
    )
    
    if response.status_code == 200 or response.status_code == 201:
        repo_data = response.json()
        print(f"✅ Repository created: {repo_data}")
        
        # Now try to delete it
        print("\nDeleting repository...")
        delete_response = requests.delete(
            f"{BASE_URL}/api/v1/repos/testorg/test-repo-delete",
            headers=headers
        )
        
        print(f"Delete response status: {delete_response.status_code}")
        print(f"Delete response body: {delete_response.text}")
        
        if delete_response.status_code == 200:
            print("✅ Repository deleted successfully!")
        else:
            print("❌ Repository deletion failed!")
        
    else:
        print(f"❌ Failed to create repository: {response.status_code} - {response.text}")

if __name__ == "__main__":
    test_delete_repository()
