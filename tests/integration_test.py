#!/usr/bin/env python3
"""
Aerugo Integration Test Suite

This script performs end-to-end testing of the Aerugo container registry:
1. Sets up development environment from scratch
2. Runs Docker containers (PostgreSQL, Redis, MinIO)
3. Runs database migrations
4. Seeds test data
5. Builds and starts the Aerugo server
6. Tests all API endpoints organized by functionality
"""

import os
import sys
import subprocess
import time
import json
import requests
import psycopg2
import redis
import logging
from typing import Dict, Optional, List
from pathlib import Path

# Import test modules
from config import BASE_DIR, SCRIPTS_DIR, SERVER_URL, API_BASE, TEST_CONFIG, get_environment_vars, get_database_url
from base_test import test_data_manager
from test_health import HealthTests
from test_auth import AuthTests
from test_organizations import OrganizationTests
from test_users import UserTests
from test_repositories import RepositoryTests

class IntegrationTestSuite:
    def __init__(self):
        self.setup_logging()
        self.server_process = None

    def setup_logging(self):
        """Setup logging configuration"""
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('integration_test.log'),
                logging.StreamHandler(sys.stdout)
            ]
        )
        self.logger = logging.getLogger(__name__)

    def run_command(self, command: str, cwd: Optional[Path] = None, check: bool = True) -> subprocess.CompletedProcess:
        """Run a shell command"""
        self.logger.info(f"Running command: {command}")
        try:
            result = subprocess.run(
                command,
                shell=True,
                cwd=cwd or BASE_DIR,
                capture_output=True,
                text=True,
                check=check
            )
            if result.stdout:
                self.logger.debug(f"STDOUT: {result.stdout}")
            if result.stderr:
                self.logger.debug(f"STDERR: {result.stderr}")
            return result
        except subprocess.CalledProcessError as e:
            self.logger.error(f"Command failed: {e}")
            self.logger.error(f"STDOUT: {e.stdout}")
            self.logger.error(f"STDERR: {e.stderr}")
            raise

    def setup_environment(self):
        """Setup the development environment from scratch"""
        self.logger.info("=== Setting up development environment ===")
        
        # Clean up any existing setup
        self.logger.info("Cleaning up existing environment...")
        self.run_command(f"{SCRIPTS_DIR}/setup-dev-env.sh clean", check=False)
        
        # Setup fresh environment
        self.logger.info("Setting up fresh development environment...")
        self.run_command(f"{SCRIPTS_DIR}/setup-dev-env.sh setup")
        
        # Wait for services to be ready
        self.logger.info("Waiting for services to stabilize...")
        time.sleep(10)

    def verify_services(self):
        """Verify all required services are running"""
        self.logger.info("=== Verifying services ===")
        
        # Check PostgreSQL
        self.logger.info("Checking PostgreSQL...")
        try:
            conn = psycopg2.connect(
                host=TEST_CONFIG["database"]["host"],
                port=TEST_CONFIG["database"]["port"],
                user=TEST_CONFIG["database"]["user"],
                password=TEST_CONFIG["database"]["password"],
                database=TEST_CONFIG["database"]["database"]
            )
            conn.close()
            self.logger.info("✅ PostgreSQL is running")
        except Exception as e:
            self.logger.error(f"❌ PostgreSQL connection failed: {e}")
            raise

        # Check Redis
        self.logger.info("Checking Redis...")
        try:
            r = redis.Redis(
                host=TEST_CONFIG["redis"]["host"],
                port=TEST_CONFIG["redis"]["port"]
            )
            r.ping()
            self.logger.info("✅ Redis is running")
        except Exception as e:
            self.logger.error(f"❌ Redis connection failed: {e}")
            raise

        # Check MinIO
        self.logger.info("Checking MinIO...")
        try:
            response = requests.get("http://localhost:9001/minio/health/ready", timeout=5)
            if response.status_code == 200:
                self.logger.info("✅ MinIO is running")
            else:
                raise Exception(f"MinIO health check failed: {response.status_code}")
        except Exception as e:
            self.logger.error(f"❌ MinIO connection failed: {e}")
            raise

    def run_migrations(self):
        """Run database migrations"""
        self.logger.info("=== Running database migrations ===")
        
        # Install sqlx-cli if not present
        self.logger.info("Installing sqlx-cli...")
        self.run_command("cargo install sqlx-cli --no-default-features --features rustls,postgres", check=False)
        
        # Run migrations
        self.logger.info("Running migrations...")
        env = os.environ.copy()
        env["DATABASE_URL"] = get_database_url()
        
        result = subprocess.run(
            ["sqlx", "migrate", "run"],
            cwd=BASE_DIR,
            env=env,
            capture_output=True,
            text=True
        )
        
        if result.returncode != 0:
            self.logger.error(f"Migration failed: {result.stderr}")
            raise Exception("Database migration failed")
        
        self.logger.info("✅ Database migrations completed")

    def seed_test_data(self):
        """Seed database with test data"""
        self.logger.info("=== Seeding test data ===")
        
        # Skip seeding since environment is fresh each time
        # Test data will be created via API calls during tests
        self.logger.info("✅ Test data seeding completed (using API calls instead)")

    def build_and_start_server(self):
        """Build and start the Aerugo server"""
        self.logger.info("=== Building and starting server ===")
        
        # Build the application
        self.logger.info("Building Aerugo server...")
        self.run_command("cargo build")
        
        # Get environment variables
        env = os.environ.copy()
        env.update(get_environment_vars())
        
        # Start the server
        self.logger.info("Starting Aerugo server...")
        self.server_process = subprocess.Popen(
            ["cargo", "run"],
            cwd=BASE_DIR,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Wait for server to start
        self.logger.info("Waiting for server to start...")
        max_attempts = 30
        for attempt in range(max_attempts):
            try:
                response = requests.get(f"{SERVER_URL}/health", timeout=5)
                if response.status_code == 200:
                    self.logger.info("✅ Aerugo server is running")
                    return
            except requests.exceptions.RequestException:
                pass
            
            if attempt < max_attempts - 1:
                time.sleep(2)
        
        # If we get here, server failed to start
        if self.server_process.poll() is not None:
            stdout, stderr = self.server_process.communicate()
            self.logger.error(f"Server process exited: stdout={stdout}, stderr={stderr}")
        
        raise Exception("Server failed to start within timeout period")

    def stop_server(self):
        """Stop the Aerugo server"""
        if self.server_process:
            self.logger.info("Stopping Aerugo server...")
            self.server_process.terminate()
            try:
                self.server_process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.server_process.kill()
                self.server_process.wait()
            self.server_process = None

    def run_all_tests(self):
        """Run all integration tests"""
        self.logger.info("🚀 Starting Aerugo Integration Test Suite")
        
        try:
            # Setup phase
            self.setup_environment()
            self.verify_services()
            self.run_migrations()
            self.seed_test_data()
            self.build_and_start_server()
            
            # Test phase - run organized test suites
            health_tests = HealthTests()
            health_tests.run_all_tests()
            
            auth_tests = AuthTests()
            auth_tests.run_all_tests()
            
            org_tests = OrganizationTests()
            org_tests.run_all_tests()
            
            user_tests = UserTests()
            user_tests.run_all_tests()
            
            repo_tests = RepositoryTests()
            repo_tests.run_all_tests()
            
            self.logger.info("🎉 All integration tests passed successfully!")
            
        except Exception as e:
            self.logger.error(f"❌ Integration tests failed: {e}")
            raise
        finally:
            # Cleanup phase
            self.stop_server()
            test_data_manager.cleanup_all()

if __name__ == "__main__":
    test_suite = IntegrationTestSuite()
    test_suite.run_all_tests()
