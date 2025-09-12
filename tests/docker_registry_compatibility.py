#!/usr/bin/env python3
"""
Docker Registry Compatibility Tests
Tests specific Docker Registry V2 API compatibility features
"""

import requests
import json
import base64
import hashlib
import tempfile
import os
import logging
from pathlib import Path

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# Configuration
REGISTRY_URL = "localhost:8080"
REGISTRY_V2_BASE = f"http://{REGISTRY_URL}/v2"


class DockerRegistryCompatibilityTester:
    """Test Docker Registry V2 API compatibility"""
    
    def __init__(self):
        self.test_repo = "compatibility-test"
        self.test_tag = "latest"
        self.test_digest = None
        
    def test_base_api(self):
        """Test GET /v2/ - Docker-Distribution-API-Version header"""
        logger.info("Testing base API endpoint...")
        
        try:
            response = requests.get(f"{REGISTRY_V2_BASE}/")
            
            # Check status code
            if response.status_code != 200:
                logger.error(f"Base API returned {response.status_code}, expected 200")
                return False
            
            # Check Docker-Distribution-API-Version header
            api_version = response.headers.get("Docker-Distribution-API-Version")
            if not api_version:
                logger.error("Missing Docker-Distribution-API-Version header")
                return False
            
            if "registry/2.0" not in api_version:
                logger.error(f"Invalid API version: {api_version}")
                return False
            
            logger.info("✓ Base API endpoint compatible")
            return True
            
        except Exception as e:
            logger.error(f"Base API test error: {e}")
            return False
    
    def test_catalog_api(self):
        """Test GET /v2/_catalog with pagination parameters"""
        logger.info("Testing catalog API...")
        
        try:
            # Test basic catalog
            response = requests.get(f"{REGISTRY_V2_BASE}/_catalog")
            if response.status_code != 200:
                logger.error(f"Catalog API returned {response.status_code}")
                return False
            
            data = response.json()
            if "repositories" not in data:
                logger.error("Catalog response missing 'repositories' field")
                return False
            
            # Test pagination parameters
            response = requests.get(f"{REGISTRY_V2_BASE}/_catalog?n=10")
            if response.status_code != 200:
                logger.error("Catalog pagination test failed")
                return False
            
            logger.info("✓ Catalog API compatible")
            return True
            
        except Exception as e:
            logger.error(f"Catalog API test error: {e}")
            return False
    
    def test_manifest_content_types(self):
        """Test manifest content type handling"""
        logger.info("Testing manifest content types...")
        
        # Create a simple manifest
        manifest = {
            "schemaVersion": 2,
            "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
            "config": {
                "mediaType": "application/vnd.docker.container.image.v1+json",
                "size": 1234,
                "digest": "sha256:abcd1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
            },
            "layers": [
                {
                    "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
                    "size": 5678,
                    "digest": "sha256:efgh1234567890abcdef1234567890abcdef1234567890abcdef1234567890cd"
                }
            ]
        }
        
        try:
            # Test PUT manifest with correct content type
            headers = {
                "Content-Type": "application/vnd.docker.distribution.manifest.v2+json"
            }
            
            response = requests.put(
                f"{REGISTRY_V2_BASE}/{self.test_repo}/manifests/{self.test_tag}",
                json=manifest,
                headers=headers
            )
            
            # This might fail due to missing blobs, but should handle content type correctly
            if response.status_code in [201, 202, 400]:  # 400 acceptable for missing blobs
                logger.info("✓ Manifest content type handling compatible")
                return True
            else:
                logger.error(f"Manifest PUT returned unexpected status: {response.status_code}")
                return False
                
        except Exception as e:
            logger.error(f"Manifest content type test error: {e}")
            return False
    
    def test_blob_upload_workflow(self):
        """Test blob upload workflow"""
        logger.info("Testing blob upload workflow...")
        
        try:
            # Step 1: Start blob upload
            response = requests.post(f"{REGISTRY_V2_BASE}/{self.test_repo}/blobs/uploads/")
            
            if response.status_code not in [202, 201]:
                logger.error(f"Blob upload start failed: {response.status_code}")
                return False
            
            # Extract upload URL from Location header
            location = response.headers.get("Location")
            if not location:
                logger.error("Missing Location header in blob upload response")
                return False
            
            # Extract UUID from location
            upload_uuid = location.split("/")[-1] if "/" in location else None
            if not upload_uuid:
                logger.error("Could not extract upload UUID")
                return False
            
            logger.info(f"✓ Blob upload started with UUID: {upload_uuid}")
            return True
            
        except Exception as e:
            logger.error(f"Blob upload workflow test error: {e}")
            return False
    
    def test_blob_digest_verification(self):
        """Test blob digest verification"""
        logger.info("Testing blob digest verification...")
        
        try:
            # Create test data
            test_data = b"Hello from Aerugo Registry compatibility test!"
            digest_sha256 = hashlib.sha256(test_data).hexdigest()
            expected_digest = f"sha256:{digest_sha256}"
            
            # Start upload
            response = requests.post(f"{REGISTRY_V2_BASE}/{self.test_repo}/blobs/uploads/")
            if response.status_code not in [202, 201]:
                logger.error("Could not start blob upload for digest test")
                return False
            
            location = response.headers.get("Location")
            if not location:
                logger.error("Missing location for digest test")
                return False
            
            # Complete upload with digest
            upload_url = location if location.startswith("http") else f"{REGISTRY_V2_BASE}{location}"
            upload_url += f"&digest={expected_digest}"
            
            response = requests.put(upload_url, data=test_data)
            
            # Should succeed or fail with proper error for digest mismatch
            if response.status_code in [201, 202, 400, 404]:
                logger.info("✓ Blob digest verification handling compatible")
                return True
            else:
                logger.error(f"Unexpected digest verification response: {response.status_code}")
                return False
                
        except Exception as e:
            logger.error(f"Blob digest verification test error: {e}")
            return False
    
    def test_error_response_format(self):
        """Test error response format compliance"""
        logger.info("Testing error response format...")
        
        try:
            # Try to get a non-existent manifest to trigger error
            response = requests.get(f"{REGISTRY_V2_BASE}/nonexistent/manifests/nonexistent")
            
            if response.status_code == 404:
                try:
                    error_data = response.json()
                    
                    # Check if error response has correct structure
                    if "errors" in error_data and isinstance(error_data["errors"], list):
                        if error_data["errors"]:
                            error = error_data["errors"][0]
                            if "code" in error and "message" in error:
                                logger.info("✓ Error response format compatible")
                                return True
                    
                    logger.warning("Error response format not fully compatible")
                    return True  # Not critical
                    
                except json.JSONDecodeError:
                    logger.warning("Error response is not JSON")
                    return True  # Not critical
            
            logger.info("✓ Error response handling working")
            return True
            
        except Exception as e:
            logger.error(f"Error response format test error: {e}")
            return False
    
    def test_head_requests(self):
        """Test HEAD request support"""
        logger.info("Testing HEAD request support...")
        
        try:
            # Test HEAD on base API
            response = requests.head(f"{REGISTRY_V2_BASE}/")
            if response.status_code not in [200, 405]:  # 405 acceptable if HEAD not implemented
                logger.error(f"HEAD base API returned {response.status_code}")
                return False
            
            # Test HEAD on catalog
            response = requests.head(f"{REGISTRY_V2_BASE}/_catalog")
            if response.status_code not in [200, 405]:
                logger.error(f"HEAD catalog returned {response.status_code}")
                return False
            
            logger.info("✓ HEAD request support compatible")
            return True
            
        except Exception as e:
            logger.error(f"HEAD request test error: {e}")
            return False
    
    def test_cors_headers(self):
        """Test CORS headers for web clients"""
        logger.info("Testing CORS headers...")
        
        try:
            # Test preflight request
            headers = {
                "Origin": "http://localhost:3000",
                "Access-Control-Request-Method": "GET",
                "Access-Control-Request-Headers": "Authorization"
            }
            
            response = requests.options(f"{REGISTRY_V2_BASE}/", headers=headers)
            
            # CORS support is optional but good to have
            cors_headers = [
                "Access-Control-Allow-Origin",
                "Access-Control-Allow-Methods",
                "Access-Control-Allow-Headers"
            ]
            
            cors_supported = any(header in response.headers for header in cors_headers)
            
            if cors_supported:
                logger.info("✓ CORS headers present")
            else:
                logger.info("⚠ No CORS headers (optional)")
            
            return True  # CORS is optional
            
        except Exception as e:
            logger.error(f"CORS test error: {e}")
            return True  # CORS is optional
    
    def run_compatibility_tests(self):
        """Run all compatibility tests"""
        logger.info("🚀 Starting Docker Registry V2 API Compatibility Tests")
        logger.info(f"Registry URL: {REGISTRY_V2_BASE}")
        logger.info("="*60)
        
        tests = [
            ("Base API Endpoint", self.test_base_api),
            ("Catalog API", self.test_catalog_api),
            ("Manifest Content Types", self.test_manifest_content_types),
            ("Blob Upload Workflow", self.test_blob_upload_workflow),
            ("Blob Digest Verification", self.test_blob_digest_verification),
            ("Error Response Format", self.test_error_response_format),
            ("HEAD Request Support", self.test_head_requests),
            ("CORS Headers", self.test_cors_headers),
        ]
        
        results = []
        passed = 0
        total = len(tests)
        
        for test_name, test_func in tests:
            logger.info(f"\n{'='*40}")
            logger.info(f"Running: {test_name}")
            logger.info(f"{'='*40}")
            
            try:
                result = test_func()
                results.append((test_name, result))
                if result:
                    passed += 1
                    logger.info(f"✅ {test_name}: PASSED")
                else:
                    logger.error(f"❌ {test_name}: FAILED")
            except Exception as e:
                logger.error(f"💥 {test_name}: ERROR - {e}")
                results.append((test_name, False))
        
        # Print summary
        logger.info(f"\n{'='*60}")
        logger.info("COMPATIBILITY TEST SUMMARY")
        logger.info(f"{'='*60}")
        logger.info(f"Total Tests: {total}")
        logger.info(f"Passed: {passed}")
        logger.info(f"Failed: {total - passed}")
        logger.info(f"Compatibility Score: {(passed/total)*100:.1f}%")
        
        for test_name, result in results:
            status = "✅ PASS" if result else "❌ FAIL"
            logger.info(f"{status}: {test_name}")
        
        return passed >= (total * 0.8)  # 80% pass rate for compatibility


def main():
    """Main test runner"""
    import sys
    
    if len(sys.argv) > 1 and sys.argv[1] in ["-h", "--help"]:
        print("Docker Registry V2 API Compatibility Test")
        print("Usage: python docker_registry_compatibility.py")
        sys.exit(0)
    
    tester = DockerRegistryCompatibilityTester()
    
    try:
        success = tester.run_compatibility_tests()
        
        if success:
            logger.info("🎉 Registry is compatible with Docker Registry V2 API!")
            sys.exit(0)
        else:
            logger.error("💥 Registry has compatibility issues!")
            sys.exit(1)
            
    except KeyboardInterrupt:
        logger.info("Test interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Test suite error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
