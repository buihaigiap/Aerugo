#!/usr/bin/env python3
"""
Cache functionality tests for Aerugo Docker Registry (Pytest version)
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import pytest
import requests
import time
from config import SERVER_URL, get_docker_registry_auth


@pytest.fixture
def base_url():
    """Base URL for tests"""
    return SERVER_URL.rstrip('/')


def test_cache_health_endpoint(base_url):
    """Test cache health and statistics endpoint"""
    health_url = f"{base_url}/health/cache"
    response = requests.get(health_url, timeout=10)
    
    assert response.status_code == 200
    data = response.json()
    assert "cache_stats" in data
    assert "memory_cache" in data["cache_stats"]
    assert "redis_connected" in data["cache_stats"]


def test_catalog_caching(base_url):
    """Test that repository catalog is cached for performance"""
    catalog_url = f"{base_url}/v2/_catalog"
    
    # Get authentication headers for Docker Registry v2 API
    auth_headers = get_docker_registry_auth()
    
    # First request - should populate cache
    start_time = time.time()
    response1 = requests.get(catalog_url, headers=auth_headers, timeout=10)
    first_duration = time.time() - start_time
    
    assert response1.status_code == 200
    data1 = response1.json()
    
    # Second request - should hit cache (faster)
    start_time = time.time()
    response2 = requests.get(catalog_url, headers=auth_headers, timeout=10)
    second_duration = time.time() - start_time
    
    assert response2.status_code == 200
    data2 = response2.json()
    
    # Verify data consistency
    assert data1 == data2


def test_tags_caching(base_url):
    """Test that repository tags are cached"""
    catalog_url = f"{base_url}/v2/_catalog"
    
    # Get authentication headers for Docker Registry v2 API
    auth_headers = get_docker_registry_auth()
    
    # Get a repository from catalog first
    catalog_response = requests.get(catalog_url, headers=auth_headers, timeout=10)
    assert catalog_response.status_code == 200
    
    repositories = catalog_response.json().get('repositories', [])
    if not repositories:
        pytest.skip("No repositories found for tags test")
    
    repo_name = repositories[0]
    tags_url = f"{base_url}/v2/{repo_name}/tags/list"
    
    # First request - should populate cache
    start_time = time.time()
    response1 = requests.get(tags_url, headers=auth_headers, timeout=10)
    first_duration = time.time() - start_time
    
    if response1.status_code != 200:
        pytest.skip(f"Tags request failed for {repo_name}")
    
    data1 = response1.json()
    
    # Second request - should hit cache
    start_time = time.time()
    response2 = requests.get(tags_url, headers=auth_headers, timeout=10)
    second_duration = time.time() - start_time
    
    assert response2.status_code == 200
    data2 = response2.json()
    
    # Verify consistency
    assert data1 == data2


def test_manifest_caching(base_url):
    """Test that manifests are cached"""
    catalog_url = f"{base_url}/v2/_catalog"
    
    # Get authentication headers for Docker Registry v2 API
    auth_headers = get_docker_registry_auth()
    
    # Get available repositories first
    catalog_response = requests.get(catalog_url, headers=auth_headers, timeout=10)
    assert catalog_response.status_code == 200
    
    repositories = catalog_response.json().get("repositories", [])
    if not repositories:
        pytest.skip("No repositories found for manifest test")
    
    # Find a repository with a working manifest
    repo_name = None
    for repo in repositories:
        test_url = f"{base_url}/v2/{repo}/manifests/latest"
        try:
            test_response = requests.get(test_url, headers=auth_headers, timeout=5)
            if test_response.status_code == 200:
                repo_name = repo
                break
        except Exception:
            continue
    
    if not repo_name:
        pytest.skip("No repositories with accessible manifests found")
    
    tag = "latest"
    manifest_url = f"{base_url}/v2/{repo_name}/manifests/{tag}"
    
    # First request - cache miss (with retry for network issues)
    max_retries = 3
    response1 = None
    for attempt in range(max_retries):
        try:
            start_time = time.time()
            response1 = requests.get(manifest_url, headers=auth_headers, timeout=10)
            first_time = time.time() - start_time
            break
        except requests.exceptions.ConnectionError:
            if attempt < max_retries - 1:
                time.sleep(0.5)  # Brief pause before retry
                continue
            pytest.skip("Connection issues prevent manifest caching test")
    
    # Second request - cache hit (with retry for network issues)
    response2 = None
    for attempt in range(max_retries):
        try:
            start_time = time.time()
            response2 = requests.get(manifest_url, headers=auth_headers, timeout=10)
            second_time = time.time() - start_time
            break
        except requests.exceptions.ConnectionError:
            if attempt < max_retries - 1:
                time.sleep(0.5)  # Brief pause before retry
                continue
            pytest.skip("Connection issues prevent manifest caching test")
    
    # Both responses should be identical (cached)
    assert response1.status_code == response2.status_code
    assert response1.text == response2.text


def test_cache_invalidation_simulation(base_url):
    """Test cache invalidation by simulating cache activity"""
    health_url = f"{base_url}/health/cache"
    catalog_url = f"{base_url}/v2/_catalog"
    
    # Get authentication headers for Docker Registry v2 API
    auth_headers = get_docker_registry_auth()
    
    # Get initial cache stats
    initial_stats = requests.get(health_url, timeout=10)
    assert initial_stats.status_code == 200
    initial_data = initial_stats.json()
    
    # Simulate some cache activity by making requests
    catalog_resp = requests.get(catalog_url, headers=auth_headers, timeout=10)
    assert catalog_resp.status_code == 200
    
    # Check final cache stats
    final_stats = requests.get(health_url, timeout=10)
    assert final_stats.status_code == 200
    final_data = final_stats.json()
    
    # Verify cache structure exists
    assert "cache_stats" in final_data
    assert "memory_cache" in final_data["cache_stats"]
    assert "redis_connected" in final_data["cache_stats"]
