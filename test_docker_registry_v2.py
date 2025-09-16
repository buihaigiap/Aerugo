#!/usr/bin/env python3
"""
Simple Docker Registry v2 API test script
Tests the core Docker Registry v2 endpoints
"""

import requests
import json
import base64
import hashlib
import time

# Configuration
SERVER_URL = "http://localhost:8080"
REGISTRY_URL = f"{SERVER_URL}/v2"

def test_version_check():
    """Test Docker Registry v2 version endpoint"""
    print("🔍 Testing Docker Registry v2 version endpoint...")
    try:
        response = requests.get(f"{REGISTRY_URL}/")
        print(f"   Status: {response.status_code}")
        print(f"   Headers: {dict(response.headers)}")
        if response.status_code == 200:
            print("   ✅ Version check passed")
            return True
        else:
            print(f"   ❌ Version check failed: {response.text}")
            return False
    except Exception as e:
        print(f"   ❌ Version check error: {e}")
        return False

def test_catalog():
    """Test Docker Registry catalog endpoint"""
    print("🔍 Testing Docker Registry catalog endpoint...")
    try:
        response = requests.get(f"{REGISTRY_URL}/_catalog")
        print(f"   Status: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            print(f"   Catalog: {data}")
            print("   ✅ Catalog test passed")
            return True
        else:
            print(f"   ❌ Catalog test failed: {response.text}")
            return False
    except Exception as e:
        print(f"   ❌ Catalog test error: {e}")
        return False

def test_blob_upload():
    """Test Docker Registry blob upload process"""
    print("🔍 Testing Docker Registry blob upload...")
    try:
        # Step 1: Start blob upload
        response = requests.post(f"{REGISTRY_URL}/hello-world/blobs/uploads/")
        print(f"   Upload start status: {response.status_code}")
        print(f"   Upload start headers: {dict(response.headers)}")
        
        if response.status_code in [201, 202]:
            upload_uuid = response.headers.get('Docker-Upload-UUID')
            location = response.headers.get('Location')
            print(f"   Upload UUID: {upload_uuid}")
            print(f"   Location: {location}")
            print("   ✅ Blob upload start passed")
            return True
        else:
            print(f"   ❌ Blob upload start failed: {response.text}")
            return False
    except Exception as e:
        print(f"   ❌ Blob upload error: {e}")
        return False

def test_manifest_operations():
    """Test Docker Registry manifest operations"""
    print("🔍 Testing Docker Registry manifest operations...")
    try:
        # Test GET manifest (should return 404 for non-existent)
        response = requests.get(f"{REGISTRY_URL}/hello-world/manifests/latest")
        print(f"   GET manifest status: {response.status_code}")
        
        if response.status_code in [200, 404]:
            print("   ✅ Manifest GET test passed (200 or 404 expected)")
            return True
        else:
            print(f"   ❌ Manifest GET test failed: {response.text}")
            return False
    except Exception as e:
        print(f"   ❌ Manifest test error: {e}")
        return False

def main():
    """Main test function"""
    print("🐳 Docker Registry v2 API Test Suite")
    print("=" * 50)
    
    # Wait a bit for server to be ready
    print("⏳ Waiting for server to be ready...")
    time.sleep(2)
    
    tests = [
        ("Version Check", test_version_check),
        ("Catalog", test_catalog),
        ("Blob Upload", test_blob_upload),
        ("Manifest Operations", test_manifest_operations),
    ]
    
    results = []
    for test_name, test_func in tests:
        print(f"\n🧪 Running: {test_name}")
        result = test_func()
        results.append((test_name, result))
        print()
    
    print("📊 Test Results Summary:")
    print("=" * 30)
    passed = 0
    for test_name, result in results:
        status = "✅ PASSED" if result else "❌ FAILED"
        print(f"   {test_name}: {status}")
        if result:
            passed += 1
    
    print(f"\n🎯 Overall: {passed}/{len(tests)} tests passed")
    
    if passed == len(tests):
        print("🎉 All Docker Registry v2 API tests passed!")
        return True
    else:
        print("⚠️  Some tests failed. Check the server logs.")
        return False

if __name__ == "__main__":
    main()
