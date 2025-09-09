"""
Test configuration and utilities for Aerugo integration tests
"""

import os
from dataclasses import dataclass
from typing import Dict, Optional
from pathlib import Path

# Base directories
BASE_DIR = Path(__file__).parent.parent
SCRIPTS_DIR = BASE_DIR / "scripts"

# Server configuration
SERVER_URL = "http://localhost:8080"
API_BASE = f"{SERVER_URL}/api/v1"

# Database configuration
TEST_CONFIG = {
    "database": {
        "host": "localhost",
        "port": 5433,
        "user": "aerugo",
        "password": "development", 
        "database": "aerugo_dev"
    },
    "redis": {
        "host": "localhost",
        "port": 6380
    },
    "server": {
        "host": "localhost",
        "port": 8080
    },
    "minio": {
        "endpoint": "http://localhost:9001",
        "access_key": "minioadmin",
        "secret_key": "minioadmin",
        "bucket": "aerugo-registry",
        "region": "us-east-1"
    },
    "auth": {
        "jwt_secret": "test-integration-secret-key-do-not-use-in-production"
    }
}

@dataclass
class TestUser:
    """Test user data structure"""
    username: str
    email: str
    password: str
    id: Optional[int] = None
    token: Optional[str] = None

@dataclass 
class TestOrganization:
    """Test organization data structure"""
    name: str
    display_name: str
    description: str
    id: Optional[int] = None
    website_url: Optional[str] = None
    avatar_url: Optional[str] = None

@dataclass
class TestRepository:
    """Test repository data structure"""
    name: str
    organization: str
    description: str
    is_public: bool = True
    id: Optional[int] = None

# Test data fixtures
TEST_USERS = [
    TestUser("testuser1", "test1@example.com", "password123"),
    TestUser("testuser2", "test2@example.com", "password456"), 
    TestUser("orgowner", "owner@example.com", "ownerpass"),
    TestUser("maintainer", "maintainer@example.com", "maintpass"),
    TestUser("viewer", "viewer@example.com", "viewpass"),
]

TEST_ORGS = [
    TestOrganization("testorg", "Test Organization", "A test organization"),
    TestOrganization("publicorg", "Public Org", "A public organization"),
    TestOrganization("privateorg", "Private Org", "A private organization"),
]

TEST_REPOS = [
    TestRepository("hello-world", "testorg", "A simple hello world container"),
    TestRepository("web-app", "testorg", "A web application container"),
    TestRepository("database", "publicorg", "A database container", is_public=True),
    TestRepository("private-tool", "privateorg", "A private tool", is_public=False),
]

def get_environment_vars() -> Dict[str, str]:
    """Get environment variables for server startup"""
    return {
        "LISTEN_ADDRESS": f"{TEST_CONFIG['server']['host']}:{TEST_CONFIG['server']['port']}",
        "LOG_LEVEL": "debug",
        "DATABASE_URL": f"postgresql://{TEST_CONFIG['database']['user']}:{TEST_CONFIG['database']['password']}@{TEST_CONFIG['database']['host']}:{TEST_CONFIG['database']['port']}/{TEST_CONFIG['database']['database']}",
        "REDIS_URL": f"redis://{TEST_CONFIG['redis']['host']}:{TEST_CONFIG['redis']['port']}",
        "S3_ENDPOINT": TEST_CONFIG["minio"]["endpoint"],
        "S3_BUCKET": TEST_CONFIG["minio"]["bucket"], 
        "S3_ACCESS_KEY": TEST_CONFIG["minio"]["access_key"],
        "S3_SECRET_KEY": TEST_CONFIG["minio"]["secret_key"],
        "S3_REGION": TEST_CONFIG["minio"]["region"],
        "JWT_SECRET": TEST_CONFIG["auth"]["jwt_secret"],
        "API_PREFIX": "/api/v1",
        "RUST_LOG": "debug",
        "RUST_BACKTRACE": "1"
    }

def get_database_url() -> str:
    """Get database URL for migrations"""
    return f"postgresql://{TEST_CONFIG['database']['user']}:{TEST_CONFIG['database']['password']}@{TEST_CONFIG['database']['host']}:{TEST_CONFIG['database']['port']}/{TEST_CONFIG['database']['database']}"
