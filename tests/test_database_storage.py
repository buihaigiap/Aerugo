#!/usr/bin/env python3
"""Test script to verify database storage for     # Step 2: Repository will     print(f"Expected digest: {expected_digest}")
    
    response = requests.put(
        f"{API_BASE_URL}/v2/{repo_name}/manifests/{tag}",
        headers={**HEADERS, "Content-Type": "application/vnd.docker.distribution.manifest.v2+json"},
        data=manifest_json
    )
    print(f"PUT manifest response: {response.status_code}")
    if response.headers.get('Docker-Content-Digest'):
        print(f"Returned digest: {response.headers['Docker-Content-Digest']}")
    
    # Step 3: Check database after push
    print(f"\n3. Database state after push:")
    push_repos, push_manifests, push_tags = check_database()
    
    # Step 4: Retrieve manifest by tagtically when pushing manifest
    repo_name = "test-db-storage"
    
    print(f"\n2. Pushing manifest to repository: {repo_name} (auto-create)")tags"""

import requests
import json
import hashlib
import psycopg2
from psycopg2.extras import RealDictCursor
from config import SERVER_URL, TEST_CONFIG

# Constants
API_BASE_URL = SERVER_URL
DB_CONFIG = TEST_CONFIG["database"]  
HEADERS = {"Content-Type": "application/json"}

def calculate_sha256(data):
    """Calculate SHA256 digest for manifest"""
    if isinstance(data, dict):
        data = json.dumps(data, separators=(',', ':')).encode('utf-8')
    elif isinstance(data, str):
        data = data.encode('utf-8')
    return f"sha256:{hashlib.sha256(data).hexdigest()}"

def check_database():
    """Check database contents"""
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        cursor = conn.cursor()
        
        print("\n🗄️ Checking database contents:")
        
        # Check repositories
        cursor.execute("SELECT id, name FROM repositories WHERE organization_id IS NULL")
        repos = cursor.fetchall()
        print(f"📁 Repositories: {len(repos)}")
        for repo_id, repo_name in repos:
            print(f"  - {repo_name} (ID: {repo_id})")
        
        # Check manifests
        cursor.execute("SELECT id, repository_id, digest, media_type, size FROM manifests")
        manifests = cursor.fetchall()
        print(f"📋 Manifests: {len(manifests)}")
        for manifest_id, repo_id, digest, media_type, size in manifests:
            print(f"  - {digest[:20]}... (repo: {repo_id}, size: {size})")
        
        # Check tags
        cursor.execute("SELECT t.name, m.digest FROM tags t JOIN manifests m ON t.manifest_id = m.id")
        tags = cursor.fetchall()
        print(f"🏷️ Tags: {len(tags)}")
        for tag_name, digest in tags:
            print(f"  - {tag_name} -> {digest[:20]}...")
            
        cursor.close()
        conn.close()
        
        return len(repos), len(manifests), len(tags)
        
    except Exception as e:
        print(f"❌ Database error: {e}")
        return 0, 0, 0

def test_manifest_push_and_retrieval():
    """Test pushing manifest and verifying database storage"""
    print("🚀 Testing manifest push and database storage...")
    
    # Step 1: Check initial database state
    print("\n1. Initial database state:")
    initial_repos, initial_manifests, initial_tags = check_database()
    
    # Step 2: Repository will be created automatically when pushing manifest
    repo_name = "test-db-storage"
    
    print(f"\n2. Pushing manifest to repository: {repo_name} (auto-create)")
    
    # Step 3: Push a manifest
    manifest = {
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
        "config": {
            "mediaType": "application/vnd.docker.container.image.v1+json",
            "size": 1469,
            "digest": "sha256:test-config-digest-12345"
        },
        "layers": [
            {
                "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip", 
                "size": 3208942,
                "digest": "sha256:test-layer-digest-67890"
            }
        ]
    }
    
    manifest_json = json.dumps(manifest, separators=(',', ':'))
    expected_digest = calculate_sha256(manifest_json)
    tag = "v1.0"
    
    print(f"\n3. Pushing manifest with tag: {tag}")
    print(f"Expected digest: {expected_digest}")
    
    response = requests.put(
        f"{API_BASE_URL}/v2/{repo_name}/manifests/{tag}",
        headers={**HEADERS, "Content-Type": "application/vnd.docker.distribution.manifest.v2+json"},
        data=manifest_json
    )
    print(f"PUT manifest response: {response.status_code}")
    if response.headers.get('Docker-Content-Digest'):
        print(f"Returned digest: {response.headers['Docker-Content-Digest']}")
    
    # Step 4: Check database after push
    print("\n4. Database state after push:")
    new_repos, new_manifests, new_tags = check_database()
    
    # Step 5: Retrieve manifest by tag
    print(f"\n5. Retrieving manifest by tag: {tag}")
    response = requests.get(f"{API_BASE_URL}/v2/{repo_name}/manifests/{tag}")
    print(f"GET manifest by tag: {response.status_code}")
    if response.status_code == 200:
        retrieved_manifest = response.json()
        print(f"Retrieved manifest mediaType: {retrieved_manifest.get('mediaType')}")
        print(f"Docker-Content-Digest header: {response.headers.get('Docker-Content-Digest')}")
    
    # Step 6: Retrieve manifest by digest
    if response.headers.get('Docker-Content-Digest'):
        digest = response.headers['Docker-Content-Digest']
        print(f"\n6. Retrieving manifest by digest: {digest}")
        response = requests.get(f"{API_BASE_URL}/v2/{repo_name}/manifests/{digest}")
        print(f"GET manifest by digest: {response.status_code}")
    
    # Step 7: List tags
    print(f"\n7. Listing tags for {repo_name}")
    response = requests.get(f"{API_BASE_URL}/v2/{repo_name}/tags/list")
    print(f"List tags: {response.status_code}")
    if response.status_code == 200:
        tags_data = response.json()
        print(f"Tags: {tags_data.get('tags', [])}")
    
    # Summary
    print(f"\n📊 Summary:")
    print(f"Repositories: {initial_repos} -> {new_repos} (+{new_repos - initial_repos})")
    print(f"Manifests: {initial_manifests} -> {new_manifests} (+{new_manifests - initial_manifests})")  
    print(f"Tags: {initial_tags} -> {new_tags} (+{new_tags - initial_tags})")
    
    success = (
        response.status_code == 200 and
        new_manifests > initial_manifests and
        new_tags > initial_tags
    )
    
    if success:
        print("✅ Database storage test PASSED!")
    else:
        print("❌ Database storage test FAILED!")
    
    return success

if __name__ == "__main__":
    try:
        test_manifest_push_and_retrieval()
    except Exception as e:
        print(f"❌ Test failed with error: {e}")
        import traceback
        traceback.print_exc()
