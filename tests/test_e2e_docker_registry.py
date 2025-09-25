#!/usr/bin/env python3
"""
End-to-End Docker Registry Tests
Tests actual docker push and pull operations with real Docker client
"""

import subprocess
import requests
import json
import tempfile
import os
import sys
import logging
import time
import threading
from pathlib import Path
from urllib.parse import urlparse

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# Configuration
REGISTRY_URL = "localhost:8080"
API_BASE = f"http://{REGISTRY_URL}/api/v1"
REGISTRY_V2_BASE = f"http://{REGISTRY_URL}/v2"


class DockerE2ETester:
    """End-to-End Docker Registry Tester"""
    
    def __init__(self):
        self.temp_dir = None
        self.test_image = "aerugo-e2e-test"
        self.test_tag = "latest"
        self.full_image_name = f"{REGISTRY_URL}/{self.test_image}:{self.test_tag}"
        
    def setup(self):
        """Setup test environment"""
        self.temp_dir = tempfile.mkdtemp()
        logger.info(f"Created temp directory: {self.temp_dir}")
        
    def teardown(self):
        """Cleanup test environment"""
        try:
            # Cleanup docker images
            self._run_docker_command(["rmi", self.full_image_name], allow_fail=True)
            self._run_docker_command(["rmi", f"{self.test_image}:{self.test_tag}"], allow_fail=True)
            
            # Cleanup temp directory
            if self.temp_dir and os.path.exists(self.temp_dir):
                import shutil
                shutil.rmtree(self.temp_dir)
                logger.info(f"Cleaned up temp directory: {self.temp_dir}")
        except Exception as e:
            logger.warning(f"Cleanup warning: {e}")
    
    def check_server_health(self):
        """Check if Aerugo server is running"""
        try:
            response = requests.get(f"http://{REGISTRY_URL}/health", timeout=5)
            if response.status_code == 200:
                logger.info("✓ Aerugo server is running")
                return True
            else:
                logger.error(f"❌ Server health check failed: {response.status_code}")
                return False
        except Exception as e:
            logger.error(f"❌ Server health check error: {e}")
            return False
    
    def check_docker_available(self):
        """Check if Docker is available"""
        try:
            result = subprocess.run(["docker", "version"], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0:
                logger.info("✓ Docker is available")
                return True
            else:
                logger.error(f"❌ Docker not available: {result.stderr}")
                return False
        except Exception as e:
            logger.error(f"❌ Docker check error: {e}")
            return False
    
    def _run_docker_command(self, cmd, allow_fail=False, timeout=60):
        """Run a docker command"""
        full_cmd = ["docker"] + cmd
        logger.info(f"Running: {' '.join(full_cmd)}")
        
        try:
            result = subprocess.run(full_cmd, capture_output=True, text=True, timeout=timeout)
            if result.returncode != 0 and not allow_fail:
                logger.error(f"Docker command failed: {result.stderr}")
                return None
            return result
        except subprocess.TimeoutExpired:
            logger.error(f"Docker command timed out: {' '.join(full_cmd)}")
            return None
        except Exception as e:
            logger.error(f"Docker command error: {e}")
            return None
    
    def create_test_dockerfile(self):
        """Create a simple test Dockerfile"""
        dockerfile_content = """FROM alpine:latest
LABEL maintainer="aerugo-test"
LABEL test.version="1.0"
RUN echo "Hello from Aerugo  Test" > /hello.txt
CMD ["cat", "/hello.txt"]
"""
        dockerfile_path = os.path.join(self.temp_dir, "Dockerfile")
        with open(dockerfile_path, "w") as f:
            f.write(dockerfile_content)
        
        logger.info(f"Created test Dockerfile at: {dockerfile_path}")
        return self.temp_dir
    
    def test_registry_v2_api(self):
        """Test Docker Registry V2 API endpoints"""
        logger.info("Testing Registry V2 API...")
        
        try:
            # Test base API
            response = requests.get(f"{REGISTRY_V2_BASE}/", timeout=10)
            if response.status_code == 200:
                logger.info("✓ Registry V2 base API working")
                logger.debug(f"API response: {response.json()}")
            else:
                logger.error(f"❌ Registry V2 base API failed: {response.status_code}")
                return False
            
            # Test catalog endpoint
            response = requests.get(f"{REGISTRY_V2_BASE}/_catalog", timeout=10)
            if response.status_code == 200:
                logger.info("✓ Registry V2 catalog API working")
                catalog = response.json()
                logger.debug(f"Catalog: {catalog}")
            else:
                logger.error(f"❌ Registry V2 catalog API failed: {response.status_code}")
                return False
            
            return True
            
        except Exception as e:
            logger.error(f"❌ Registry V2 API test error: {e}")
            return False
    
    def test_docker_build_image(self):
        """Build a test Docker image"""
        logger.info("Building test Docker image...")
        
        try:
            build_context = self.create_test_dockerfile()
            
            # Build the image
            result = self._run_docker_command([
                "build", "-t", f"{self.test_image}:{self.test_tag}", 
                build_context
            ], timeout=120)
            
            if result and result.returncode == 0:
                logger.info("✓ Docker image built successfully")
                return True
            else:
                logger.error("❌ Docker image build failed")
                return False
                
        except Exception as e:
            logger.error(f"❌ Docker build error: {e}")
            return False
    
    def test_docker_tag_image(self):
        """Tag the image for our registry"""
        logger.info("Tagging image for registry...")
        
        try:
            result = self._run_docker_command([
                "tag", f"{self.test_image}:{self.test_tag}", 
                self.full_image_name
            ])
            
            if result and result.returncode == 0:
                logger.info("✓ Docker image tagged successfully")
                return True
            else:
                logger.error("❌ Docker image tagging failed")
                return False
                
        except Exception as e:
            logger.error(f"❌ Docker tagging error: {e}")
            return False
    
    def configure_registry_insecure(self):
        """Configure Docker daemon to allow insecure registry"""
        logger.info("Note: Make sure Docker daemon is configured for insecure registry")
        logger.info(f"Add '{REGISTRY_URL}' to insecure-registries in Docker daemon config")
        logger.info("Or run: docker run --add-host=host.docker.internal:host-gateway ...")
        
        # Check if we can reach the registry
        try:
            result = self._run_docker_command(["info"], timeout=10)
            if result:
                logger.info("Docker daemon is running")
            return True
        except Exception as e:
            logger.error(f"Docker daemon check error: {e}")
            return False
    
    def test_docker_push(self):
        """Test docker push to our registry"""
        logger.info(f"Testing docker push to {self.full_image_name}...")
        
        try:
            # First, configure for insecure registry if needed
            self.configure_registry_insecure()
            
            # Push the image
            result = self._run_docker_command([
                "push", self.full_image_name
            ], timeout=300)  # 5 minute timeout for push
            
            if result and result.returncode == 0:
                logger.info("✓ Docker push successful")
                return True
            else:
                logger.error("❌ Docker push failed")
                if result:
                    logger.error(f"Push stderr: {result.stderr}")
                    logger.error(f"Push stdout: {result.stdout}")
                return False
                
        except Exception as e:
            logger.error(f"❌ Docker push error: {e}")
            return False
    
    def test_docker_pull(self):
        """Test docker pull from our registry"""
        logger.info(f"Testing docker pull from {self.full_image_name}...")
        
        try:
            # Remove local image first
            self._run_docker_command(["rmi", self.full_image_name], allow_fail=True)
            
            # Pull the image
            result = self._run_docker_command([
                "pull", self.full_image_name
            ], timeout=300)  # 5 minute timeout for pull
            
            if result and result.returncode == 0:
                logger.info("✓ Docker pull successful")
                return True
            else:
                logger.error("❌ Docker pull failed")
                if result:
                    logger.error(f"Pull stderr: {result.stderr}")
                    logger.error(f"Pull stdout: {result.stdout}")
                return False
                
        except Exception as e:
            logger.error(f"❌ Docker pull error: {e}")
            return False
    
    def test_docker_run_pulled_image(self):
        """Test running the pulled image"""
        logger.info("Testing pulled image execution...")
        
        try:
            result = self._run_docker_command([
                "run", "--rm", self.full_image_name
            ], timeout=30)
            
            if result and result.returncode == 0:
                logger.info("✓ Docker run successful")
                logger.info(f"Container output: {result.stdout.strip()}")
                
                # Check if our test content is there
                if "Hello from Aerugo  Test" in result.stdout:
                    logger.info("✓ Container content verified")
                    return True
                else:
                    logger.warning("⚠ Container output unexpected")
                    return False
            else:
                logger.error("❌ Docker run failed")
                if result:
                    logger.error(f"Run stderr: {result.stderr}")
                return False
                
        except Exception as e:
            logger.error(f"❌ Docker run error: {e}")
            return False
    
    def check_registry_contents_via_api(self):
        """Check registry contents using Registry V2 API"""
        logger.info("Checking registry contents via API...")
        
        try:
            # Check catalog
            response = requests.get(f"{REGISTRY_V2_BASE}/_catalog", timeout=10)
            if response.status_code == 200:
                catalog = response.json()
                logger.info(f"Registry catalog: {catalog}")
                
                if self.test_image in catalog.get("repositories", []):
                    logger.info("✓ Test image found in catalog")
                else:
                    logger.warning("⚠ Test image not found in catalog")
                    
            # Check tags for our repository
            response = requests.get(f"{REGISTRY_V2_BASE}/{self.test_image}/tags/list", timeout=10)
            if response.status_code == 200:
                tags = response.json()
                logger.info(f"Image tags: {tags}")
                
                if self.test_tag in tags.get("tags", []):
                    logger.info("✓ Test tag found")
                    return True
                else:
                    logger.warning("⚠ Test tag not found")
                    return False
            else:
                logger.error(f"❌ Tag list API failed: {response.status_code}")
                return False
                
        except Exception as e:
            logger.error(f"❌ Registry API check error: {e}")
            return False
    
    def run_comprehensive_test(self):
        """Run comprehensive end-to-end test"""
        logger.info("🚀 Starting comprehensive Docker Registry E2E tests...")
        
        tests = [
            ("Server Health Check", self.check_server_health),
            ("Docker Available Check", self.check_docker_available),
            ("Registry V2 API Test", self.test_registry_v2_api),
            ("Docker Build Test Image", self.test_docker_build_image),
            ("Docker Tag for Registry", self.test_docker_tag_image),
            ("Docker Push Test", self.test_docker_push),
            ("Registry API Content Check", self.check_registry_contents_via_api),
            ("Docker Pull Test", self.test_docker_pull),
            ("Docker Run Pulled Image", self.test_docker_run_pulled_image),
        ]
        
        results = []
        passed = 0
        total = len(tests)
        
        for test_name, test_func in tests:
            logger.info(f"\n{'='*60}")
            logger.info(f"Running: {test_name}")
            logger.info(f"{'='*60}")
            
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
        logger.info("TEST SUMMARY")
        logger.info(f"{'='*60}")
        logger.info(f"Total Tests: {total}")
        logger.info(f"Passed: {passed}")
        logger.info(f"Failed: {total - passed}")
        logger.info(f"Success Rate: {(passed/total)*100:.1f}%")
        
        for test_name, result in results:
            status = "✅ PASS" if result else "❌ FAIL"
            logger.info(f"{status}: {test_name}")
        
        return passed == total


def main():
    """Main test runner"""
    if len(sys.argv) > 1 and sys.argv[1] in ["-h", "--help"]:
        print("Docker Registry E2E Test")
        print("Usage: python test_e2e_docker_registry.py [--quick]")
        print("  --quick: Run quick tests only")
        sys.exit(0)
    
    quick_mode = len(sys.argv) > 1 and sys.argv[1] == "--quick"
    
    tester = DockerE2ETester()
    
    try:
        tester.setup()
        
        if quick_mode:
            logger.info("Running quick tests...")
            success = (tester.check_server_health() and 
                      tester.check_docker_available() and
                      tester.test_registry_v2_api())
        else:
            success = tester.run_comprehensive_test()
        
        if success:
            logger.info("🎉 All tests passed!")
            sys.exit(0)
        else:
            logger.error("💥 Some tests failed!")
            sys.exit(1)
            
    finally:
        tester.teardown()


if __name__ == "__main__":
    main()
