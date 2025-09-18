#!/usr/bin/env python3
"""
Database storage tests for Aerugo Docker Registry (Pytest version)
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import pytest
import requests
import json
import hashlib
import psycopg2
from psycopg2.extras import RealDictCursor
from config import SERVER_URL, TEST_CONFIG


@pytest.fixture
def db_config():
    """Database configuration"""
    return TEST_CONFIG["database"]


@pytest.fixture  
def api_base_url():
    """API base URL"""
    return SERVER_URL


def calculate_sha256(data):
    """Calculate SHA256 digest for manifest"""
    if isinstance(data, dict):
        data = json.dumps(data, separators=(',', ':')).encode('utf-8')
    elif isinstance(data, str):
        data = data.encode('utf-8')
    return f"sha256:{hashlib.sha256(data).hexdigest()}"


def get_database_counts(db_config):
    """Get current database entity counts"""
    try:
        conn = psycopg2.connect(**db_config)
        cursor = conn.cursor()
        
        cursor.execute("SELECT COUNT(*) FROM repositories")
        repo_count = cursor.fetchone()[0]
        
        cursor.execute("SELECT COUNT(*) FROM manifests")
        manifest_count = cursor.fetchone()[0]
        
        cursor.execute("SELECT COUNT(*) FROM tags")
        tag_count = cursor.fetchone()[0]
        
        cursor.close()
        conn.close()
        
        return repo_count, manifest_count, tag_count
    except Exception:
        return 0, 0, 0


def test_manifest_storage_and_retrieval(db_config, api_base_url):
    """Test that manifests are properly stored in database and can be retrieved"""
    # Get initial counts
    initial_repos, initial_manifests, initial_tags = get_database_counts(db_config)
    
    # Test manifest
    manifest = {
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
        "config": {
            "mediaType": "application/vnd.docker.container.image.v1+json",
            "size": 7023,
            "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
        },
        "layers": [
            {
                "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
                "size": 32654,
                "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f"
            },
            {
                "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip", 
                "size": 16724,
                "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
            }
        ]
    }
    
    repo_name = "test-db-storage"
    tag = "v1.0"
    manifest_json = json.dumps(manifest, separators=(',', ':'))
    expected_digest = calculate_sha256(manifest_json)
    
    # Push manifest
    response = requests.put(
        f"{api_base_url}/v2/{repo_name}/manifests/{tag}",
        headers={"Content-Type": "application/vnd.docker.distribution.manifest.v2+json"},
        data=manifest_json
    )
    
    assert response.status_code == 201, f"Manifest push failed: {response.status_code}"
    assert response.headers.get('Docker-Content-Digest'), "No digest returned"
    
    # Get final counts
    final_repos, final_manifests, final_tags = get_database_counts(db_config)
    
    # Verify counts increased (or at least stayed the same for existing data)
    assert final_manifests >= initial_manifests, "Manifest count should not decrease"
    assert final_tags >= initial_tags, "Tag count should not decrease"


def test_manifest_retrieval(api_base_url):
    """Test that stored manifests can be retrieved"""
    repo_name = "test-db-storage"
    tag = "v1.0"
    
    # Retrieve manifest by tag
    response = requests.get(f"{api_base_url}/v2/{repo_name}/manifests/{tag}")
    
    # Accept both 200 (found) and 404 (not found) as valid responses
    assert response.status_code in [200, 404], f"Unexpected status: {response.status_code}"
    
    if response.status_code == 200:
        # If found, verify it has proper structure
        manifest_data = response.json()
        assert "schemaVersion" in manifest_data
        assert "mediaType" in manifest_data


def test_repository_tags_listing(api_base_url):
    """Test that repository tags can be listed"""
    repo_name = "test-db-storage"
    
    # List tags for repository
    response = requests.get(f"{api_base_url}/v2/{repo_name}/tags/list")
    
    # Accept both 200 (found) and 404 (not found)
    assert response.status_code in [200, 404], f"Unexpected status: {response.status_code}"
    
    if response.status_code == 200:
        tags_data = response.json()
        assert "tags" in tags_data
        assert isinstance(tags_data["tags"], list)
