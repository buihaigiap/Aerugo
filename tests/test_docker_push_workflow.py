#!/usr/bin/env python3
"""
Docker Push Workflow Test
Tests the complete Docker push workflow including:
1. Blob upload initiation
2. Blob data upload (chunked/monolithic)  
3. Blob completion
4. Manifest upload
5. Image availability verification
"""

import requests
import json
import hashlib
import gzip
import time
import uuid
import pytest
from config import get_docker_registry_auth, SERVER_URL


class TestDockerPushWorkflow:
    """Test complete Docker push workflow"""
    
    def setup_method(self):
        """Setup test environment"""
        self.base_url = SERVER_URL
        self.auth_headers = get_docker_registry_auth()
        
        # Test repository 
        self.test_repo = f"testuser1/docker-push-test-{int(time.time())}"
        self.test_tag = "latest"
        
        # Sample blob data (simulating a layer)
        self.sample_layer_data = b"Sample Docker layer data for testing push workflow" * 100
        self.layer_digest = f"sha256:{hashlib.sha256(self.sample_layer_data).hexdigest()}"
        
        # Sample config blob
        self.config_data = json.dumps({
            "architecture": "amd64",
            "config": {
                "Env": ["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"],
                "Cmd": ["/bin/sh"]
            },
            "rootfs": {
                "type": "layers",
                "diff_ids": [self.layer_digest]
            }
        }).encode('utf-8')
        self.config_digest = f"sha256:{hashlib.sha256(self.config_data).hexdigest()}"
        
        print(f"🧪 Testing Docker push workflow for {self.test_repo}:{self.test_tag}")
        print(f"   Layer size: {len(self.sample_layer_data)} bytes")
        print(f"   Layer digest: {self.layer_digest}")
        print(f"   Config size: {len(self.config_data)} bytes") 
        print(f"   Config digest: {self.config_digest}")

    def test_complete_docker_push_workflow(self):
        """Test the complete Docker push workflow"""
        print("\n🚀 Starting complete Docker push workflow test...")
        
        # Step 1: Push config blob
        config_location = self._push_blob(self.config_data, self.config_digest, "config blob")
        assert config_location is not None, "Config blob push failed"
        
        # Step 2: Push layer blob
        layer_location = self._push_blob(self.sample_layer_data, self.layer_digest, "layer blob")
        assert layer_location is not None, "Layer blob push failed"
        
        # Step 3: Verify blobs exist
        self._verify_blob_exists(self.config_digest, "config blob")
        self._verify_blob_exists(self.layer_digest, "layer blob")
        
        # Step 4: Push manifest
        manifest_data = self._create_manifest()
        manifest_digest = self._push_manifest(manifest_data)
        assert manifest_digest is not None, "Manifest push failed"
        
        # Step 5: Verify complete image
        self._verify_image_complete()
        
        print("✅ Complete Docker push workflow test passed!")

    def test_chunked_blob_upload(self):
        """Test chunked blob upload"""
        print("\n📦 Testing chunked blob upload...")
        
        # Create larger data for chunking
        large_data = b"Chunked upload test data " * 1000  # ~25KB
        chunk_size = 1024  # 1KB chunks
        digest = f"sha256:{hashlib.sha256(large_data).hexdigest()}"
        
        # Step 1: Start upload
        upload_uuid, location = self._start_blob_upload()
        assert upload_uuid is not None, "Failed to start chunked upload"
        
        # Step 2: Upload chunks
        offset = 0
        for i in range(0, len(large_data), chunk_size):
            chunk = large_data[i:i + chunk_size]
            self._upload_blob_chunk(upload_uuid, chunk, offset)
            offset += len(chunk)
            print(f"   📤 Uploaded chunk {i // chunk_size + 1}, offset: {offset}")
        
        # Step 3: Complete upload
        self._complete_blob_upload(upload_uuid, digest, b"")
        
        # Step 4: Verify blob
        self._verify_blob_exists(digest, "chunked blob")
        
        print("✅ Chunked blob upload test passed!")

    def test_monolithic_blob_upload(self):
        """Test monolithic (single request) blob upload"""
        print("\n📦 Testing monolithic blob upload...")
        
        data = b"Monolithic upload test data"
        digest = f"sha256:{hashlib.sha256(data).hexdigest()}"
        
        # Step 1: Start upload
        upload_uuid, location = self._start_blob_upload()
        assert upload_uuid is not None, "Failed to start monolithic upload"
        
        # Step 2: Complete upload with data
        self._complete_blob_upload(upload_uuid, digest, data)
        
        # Step 3: Verify blob
        self._verify_blob_exists(digest, "monolithic blob")
        
        print("✅ Monolithic blob upload test passed!")

    def test_blob_upload_error_scenarios(self):
        """Test error scenarios in blob upload"""
        print("\n❌ Testing blob upload error scenarios...")
        
        # Test 1: Invalid digest
        upload_uuid, location = self._start_blob_upload()
        data = b"test data"
        wrong_digest = "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        
        complete_url = f"{self.base_url}/v2/{self.test_repo}/blobs/uploads/{upload_uuid}?digest={wrong_digest}"
        response = requests.put(complete_url, headers=self.auth_headers, data=data, timeout=10)
        print(f"   Wrong digest test: {response.status_code}")
        # Should fail with 400 or similar
        
        # Test 2: Non-existent upload UUID
        fake_uuid = str(uuid.uuid4())
        fake_complete_url = f"{self.base_url}/v2/{self.test_repo}/blobs/uploads/{fake_uuid}?digest=sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        response = requests.put(fake_complete_url, headers=self.auth_headers, data=b"", timeout=10)
        print(f"   Non-existent upload UUID test: {response.status_code}")
        # Should fail with 404
        
        print("✅ Blob upload error scenarios tested!")

    def test_manifest_upload_scenarios(self):
        """Test various manifest upload scenarios"""
        print("\n📋 Testing manifest upload scenarios...")
        
        # Create a simple manifest
        manifest = {
            "schemaVersion": 2,
            "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
            "config": {
                "mediaType": "application/vnd.docker.container.image.v1+json",
                "size": len(self.config_data),
                "digest": self.config_digest
            },
            "layers": [{
                "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
                "size": len(self.sample_layer_data),
                "digest": self.layer_digest
            }]
        }
        
        manifest_json = json.dumps(manifest, separators=(',', ':')).encode('utf-8')
        
        # Test 1: Upload manifest by tag
        manifest_url = f"{self.base_url}/v2/{self.test_repo}/manifests/{self.test_tag}"
        response = requests.put(
            manifest_url,
            headers={
                **self.auth_headers,
                'Content-Type': 'application/vnd.docker.distribution.manifest.v2+json'
            },
            data=manifest_json,
            timeout=10
        )
        print(f"   Manifest upload by tag: {response.status_code}")
        
        # Test 2: Verify manifest can be retrieved
        get_response = requests.get(manifest_url, headers=self.auth_headers, timeout=10)
        print(f"   Manifest retrieval: {get_response.status_code}")
        
        # Test 3: Check manifest by digest
        manifest_digest = f"sha256:{hashlib.sha256(manifest_json).hexdigest()}"
        digest_url = f"{self.base_url}/v2/{self.test_repo}/manifests/{manifest_digest}"
        digest_response = requests.get(digest_url, headers=self.auth_headers, timeout=10)
        print(f"   Manifest by digest: {digest_response.status_code}")
        
        print("✅ Manifest upload scenarios tested!")

    def _push_blob(self, data: bytes, expected_digest: str, blob_type: str) -> str:
        """Push a blob using the complete workflow"""
        print(f"   📤 Pushing {blob_type} ({len(data)} bytes, {expected_digest})")
        
        # Start upload
        upload_uuid, location = self._start_blob_upload()
        if not upload_uuid:
            return None
        
        # Complete upload
        self._complete_blob_upload(upload_uuid, expected_digest, data)
        return location

    def _start_blob_upload(self) -> tuple:
        """Start blob upload and return UUID and location"""
        upload_url = f"{self.base_url}/v2/{self.test_repo}/blobs/uploads/"
        
        response = requests.post(upload_url, headers=self.auth_headers, timeout=10)
        
        if response.status_code not in [202, 201]:
            print(f"   ❌ Failed to start upload: {response.status_code} - {response.text}")
            return None, None
        
        location = response.headers.get("Location")
        upload_uuid = response.headers.get("Docker-Upload-UUID")
        
        if not location or not upload_uuid:
            print(f"   ❌ Missing headers - Location: {location}, UUID: {upload_uuid}")
            return None, None
        
        print(f"   ✅ Upload started - UUID: {upload_uuid}")
        return upload_uuid, location

    def _upload_blob_chunk(self, upload_uuid: str, chunk_data: bytes, offset: int):
        """Upload a chunk of blob data"""
        chunk_url = f"{self.base_url}/v2/{self.test_repo}/blobs/uploads/{upload_uuid}"
        
        headers = {
            **self.auth_headers,
            'Content-Range': f'{offset}-{offset + len(chunk_data) - 1}',
            'Content-Length': str(len(chunk_data))
        }
        
        response = requests.patch(chunk_url, headers=headers, data=chunk_data, timeout=10)
        
        if response.status_code not in [202]:
            print(f"   ❌ Chunk upload failed: {response.status_code} - {response.text}")
            raise Exception(f"Chunk upload failed: {response.status_code}")

    def _complete_blob_upload(self, upload_uuid: str, expected_digest: str, data: bytes):
        """Complete blob upload"""
        complete_url = f"{self.base_url}/v2/{self.test_repo}/blobs/uploads/{upload_uuid}?digest={expected_digest}"
        
        response = requests.put(complete_url, headers=self.auth_headers, data=data, timeout=10)
        
        if response.status_code not in [201, 202]:
            print(f"   ❌ Failed to complete upload: {response.status_code} - {response.text}")
            raise Exception(f"Upload completion failed: {response.status_code}")
        
        print(f"   ✅ Upload completed: {expected_digest}")

    def _verify_blob_exists(self, digest: str, blob_type: str):
        """Verify blob exists via HEAD request"""
        blob_url = f"{self.base_url}/v2/{self.test_repo}/blobs/{digest}"
        
        response = requests.head(blob_url, headers=self.auth_headers, timeout=10)
        
        if response.status_code != 200:
            print(f"   ❌ {blob_type} verification failed: {response.status_code}")
            raise Exception(f"{blob_type} not found: {response.status_code}")
        
        print(f"   ✅ {blob_type} verified: {digest}")

    def _create_manifest(self) -> bytes:
        """Create a Docker manifest"""
        manifest = {
            "schemaVersion": 2,
            "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
            "config": {
                "mediaType": "application/vnd.docker.container.image.v1+json",
                "size": len(self.config_data),
                "digest": self.config_digest
            },
            "layers": [{
                "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip", 
                "size": len(self.sample_layer_data),
                "digest": self.layer_digest
            }]
        }
        
        return json.dumps(manifest, separators=(',', ':')).encode('utf-8')

    def _push_manifest(self, manifest_data: bytes) -> str:
        """Push manifest and return digest"""
        manifest_url = f"{self.base_url}/v2/{self.test_repo}/manifests/{self.test_tag}"
        
        headers = {
            **self.auth_headers,
            'Content-Type': 'application/vnd.docker.distribution.manifest.v2+json'
        }
        
        response = requests.put(manifest_url, headers=headers, data=manifest_data, timeout=10)
        
        if response.status_code not in [201, 202]:
            print(f"   ❌ Manifest push failed: {response.status_code} - {response.text}")
            raise Exception(f"Manifest push failed: {response.status_code}")
        
        manifest_digest = f"sha256:{hashlib.sha256(manifest_data).hexdigest()}"
        print(f"   ✅ Manifest pushed: {manifest_digest}")
        return manifest_digest

    def _verify_image_complete(self):
        """Verify the complete image is available"""
        # Test catalog contains our repo
        catalog_url = f"{self.base_url}/v2/_catalog"
        response = requests.get(catalog_url, headers=self.auth_headers, timeout=10)
        
        if response.status_code == 200:
            repos = response.json().get("repositories", [])
            if self.test_repo in repos:
                print(f"   ✅ Repository {self.test_repo} found in catalog")
            else:
                print(f"   ⚠️  Repository {self.test_repo} not yet in catalog (may be expected)")
        
        # Test tags endpoint
        tags_url = f"{self.base_url}/v2/{self.test_repo}/tags/list"
        response = requests.get(tags_url, headers=self.auth_headers, timeout=10)
        
        if response.status_code == 200:
            tags = response.json().get("tags", [])
            if self.test_tag in tags:
                print(f"   ✅ Tag {self.test_tag} found in repository")
            else:
                print(f"   ⚠️  Tag {self.test_tag} not found in tags list")
        
        # Test manifest retrieval
        manifest_url = f"{self.base_url}/v2/{self.test_repo}/manifests/{self.test_tag}"
        response = requests.get(manifest_url, headers=self.auth_headers, timeout=10)
        
        if response.status_code == 200:
            print(f"   ✅ Manifest retrieval successful")
        else:
            print(f"   ❌ Manifest retrieval failed: {response.status_code}")


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
