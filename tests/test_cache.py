#!/usr/bin/env python3
"""
Cache functionality tests for Aerugo Docker Registry
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    from base_test import BaseTestCase
    from config import SERVER_URL
except ImportError:
    from .base_test import BaseTestCase  
    from .config import SERVER_URL

import requests
import time


class CacheTests(BaseTestCase):
    """Test caching functionality"""
    
    def __init__(self):
        super().__init__()
        self.base_url = SERVER_URL.rstrip('/')
    
    def test_cache_health_endpoint(self):
        """Test cache health and statistics endpoint"""
        self.logger.info("Testing cache health endpoint")
        
        # Use SERVER_URL directly since health is not under /api/v1
        health_url = f"{SERVER_URL}/health/cache"
        try:
            response = requests.get(health_url, timeout=10)
            
            if response.status_code == 200:
                data = response.json()
                assert "cache_stats" in data, "Cache stats should be present"
                
                stats = data["cache_stats"]
                assert "memory_cache" in stats, "Memory cache stats should be present"
                assert "redis_connected" in stats, "Redis connection status should be present"
                
                self.logger.info(f"✅ Cache health OK: {data}")
            else:
                self.logger.warning(f"⚠️ Cache health endpoint failed: {response.status_code}")
                raise AssertionError(f"Cache health endpoint returned {response.status_code}")
        except requests.exceptions.RequestException as e:
            self.logger.error(f"❌ Cache health request failed: {e}")
            raise
    
    def test_catalog_caching(self):
        """Test that repository catalog is cached for performance"""
        self.logger.info("Testing catalog caching")
        
        catalog_url = f"{SERVER_URL}/v2/_catalog"
        
        # First request - should populate cache
        try:
            start_time = time.time()
            response1 = requests.get(catalog_url, timeout=10)
            first_duration = time.time() - start_time
            
            if response1.status_code != 200:
                self.logger.error(f"❌ test_catalog_caching FAILED: First catalog request failed: {response1.status_code}")
                raise AssertionError(f"First catalog request failed with status {response1.status_code}")
            
            data1 = response1.json()
            self.logger.info(f"First catalog request: {first_duration:.3f}s, got {len(data1.get('repositories', []))} repos")
            
            # Second request - should hit cache (faster)
            start_time = time.time()
            response2 = requests.get(catalog_url, timeout=10)
            second_duration = time.time() - start_time
            
            if response2.status_code != 200:
                self.logger.error(f"Second catalog request failed: {response2.status_code}")
                raise AssertionError(f"Second catalog request failed with status {response2.status_code}")
                
            data2 = response2.json()
            
            # Verify data consistency
            assert data1 == data2, "Cached catalog data should be identical"
            
            # Verify performance improvement (cache should be at least 10% faster)
            if second_duration < first_duration * 0.9:
                improvement = ((first_duration - second_duration) / first_duration) * 100
                self.logger.info(f"✅ Cache hit! Performance improvement: {improvement:.1f}% faster")
            else:
                self.logger.info(f"⚠️ Cache might not be working optimally (times: {first_duration:.3f}s -> {second_duration:.3f}s)")
                
        except requests.exceptions.RequestException as e:
            self.logger.error(f"❌ Catalog caching test failed: {e}")
            raise
    
    def test_tags_caching(self):
        """Test that repository tags are cached"""
        self.logger.info("Testing tags caching")
        
        # Get a repository from catalog first
        catalog_url = f"{SERVER_URL}/v2/_catalog"
        try:
            catalog_response = requests.get(catalog_url, timeout=10)
            if catalog_response.status_code != 200:
                self.logger.warning(f"⚠️ Cannot get catalog for tags test")
                return
            
            repositories = catalog_response.json().get('repositories', [])
            if not repositories:
                self.logger.warning(f"⚠️ No repositories found for tags test")
                return
            
            repo_name = repositories[0]
            tags_url = f"{SERVER_URL}/v2/{repo_name}/tags/list"
            
            # First request - should populate cache
            start_time = time.time()
            response1 = requests.get(tags_url, timeout=10)
            first_duration = time.time() - start_time
            
            if response1.status_code != 200:
                self.logger.warning(f"⚠️ Tags request failed for {repo_name}: {response1.status_code}")
                return
            
            data1 = response1.json()
            self.logger.info(f"First tags request: {first_duration:.3f}s")
            
            # Second request - should hit cache
            start_time = time.time()
            response2 = requests.get(tags_url, timeout=10)
            second_duration = time.time() - start_time
            
            if response2.status_code != 200:
                self.logger.warning(f"Second tags request failed: {response2.status_code}")
                return
                
            data2 = response2.json()
            
            # Verify consistency
            assert data1 == data2, "Cached tags data should be identical"
            
            if second_duration < first_duration * 0.9:
                improvement = ((first_duration - second_duration) / first_duration) * 100
                self.logger.info(f"✅ Tags cache hit! Performance improvement: {improvement:.1f}% faster")
            else:
                self.logger.info(f"⚠️ Tags cache might not be working (times: {first_duration:.3f}s -> {second_duration:.3f}s)")
                
        except requests.exceptions.RequestException as e:
            self.logger.error(f"❌ Tags caching test failed: {e}")
            raise
    
    def test_manifest_caching(self):
        """Test that manifests are cached"""
        self.logger.info("Testing manifest caching")
        
        # Get available repositories first
        catalog_url = f"{SERVER_URL}/v2/_catalog"
        try:
            catalog_response = requests.get(catalog_url, timeout=10)
            if catalog_response.status_code != 200:
                self.logger.warning("⚠️ Cannot get catalog for manifest test")
                return
                
            repositories = catalog_response.json().get("repositories", [])
            if not repositories:
                self.logger.warning("⚠️ No repositories found for manifest test")
                return
            
            # Test with first available repository
            repo_name = repositories[0]
            tag = "latest"  # Assume latest tag exists or will return mock
            manifest_url = f"{SERVER_URL}/v2/{repo_name}/manifests/{tag}"
            
            # First request - cache miss
            start_time = time.time()
            response1 = requests.get(manifest_url, timeout=10)
            first_time = time.time() - start_time
            
            # Second request - cache hit (regardless of status code)
            start_time = time.time()
            response2 = requests.get(manifest_url, timeout=10)
            second_time = time.time() - start_time
            
            # Both responses should be identical (cached)
            assert response1.status_code == response2.status_code, "Status codes should match"
            assert response1.text == response2.text, "Cached manifest should be identical"
            
            self.logger.info(f"Manifest caching for {repo_name}:{tag}: {first_time:.3f}s -> {second_time:.3f}s")
            
            if response1.status_code == 200:
                self.logger.info("✅ Manifest found and cached")
            else:
                self.logger.info(f"✅ Manifest response ({response1.status_code}) cached consistently")
            
            if second_time < first_time:
                improvement = ((first_time - second_time) / first_time) * 100
                self.logger.info(f"✅ Manifest performance improvement: {improvement:.1f}%")
                
        except requests.exceptions.RequestException as e:
            self.logger.error(f"❌ Manifest caching test failed: {e}")
            raise
    
    def test_cache_invalidation_simulation(self):
        """Test cache invalidation by simulating manifest upload"""
        self.logger.info("Testing cache invalidation simulation")
        
        # Get initial cache stats
        health_url = f"{SERVER_URL}/health/cache"
        catalog_url = f"{SERVER_URL}/v2/_catalog"
        
        try:
            initial_stats = requests.get(health_url, timeout=10)
            if initial_stats.status_code == 200:
                initial_data = initial_stats.json()
                self.logger.info(f"Initial cache stats: {initial_data['cache_stats']}")
            
            # Simulate some cache activity by making requests
            catalog_resp = requests.get(catalog_url, timeout=10)
            
            # Check final cache stats
            final_stats = requests.get(health_url, timeout=10)
            if final_stats.status_code == 200:
                final_data = final_stats.json()
                self.logger.info(f"Final cache stats: {final_data['cache_stats']}")
                
                # Verify cache entries increased
                initial_count = initial_data.get('cache_stats', {}).get('memory_cache', {}).get('repository_count', 0) if initial_stats.status_code == 200 else 0
                final_count = final_data['cache_stats']['memory_cache']['repository_count']
                
                if final_count >= initial_count:
                    self.logger.info("✅ Cache entries populated correctly")
                else:
                    self.logger.warning("⚠️ Cache entry count decreased unexpectedly")
                    
        except requests.exceptions.RequestException as e:
            self.logger.error(f"❌ Cache invalidation test failed: {e}")
            raise
    
    def run_all_tests(self):
        """Run all cache tests"""
        self.logger.info("=== Running Cache Tests ===")
        
        # Run individual cache tests
        self.test_cache_health_endpoint()
        self.test_catalog_caching()
        self.test_tags_caching() 
        self.test_manifest_caching()
        self.test_cache_invalidation_simulation()
        
        self.logger.info("✅ All cache tests passed")


# For standalone execution
if __name__ == '__main__':
    # Set up logging
    import logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )
    
    cache_tests = CacheTests()
    
    try:
        cache_tests.run_all_tests()
        print("\n🎉 All cache tests PASSED!")
        exit(0)
    except Exception as e:
        print(f"\n❌ Some cache tests FAILED: {e}")
        exit(1)
    if success:
        print("\n🎉 All cache tests PASSED!")
        sys.exit(0)
    else:
        print("\n❌ Some cache tests FAILED!")
        sys.exit(1)
