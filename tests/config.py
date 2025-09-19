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

def get_docker_registry_auth():
    """Get authentication credentials for Docker Registry v2 API tests"""
    import base64
    import requests
    
    # Use the first test user
    user = TEST_USERS[0]
    
    # Try to register the user first (ignore if already exists)
    try:
        register_response = requests.post(
            f"{API_BASE}/auth/register",
            json={
                "username": user.username,
                "email": user.email,
                "password": user.password
            },
            timeout=10
        )
        # If successful, we get a token
        if register_response.status_code == 201:
            data = register_response.json()
            token = data.get("token")
            if token:
                return {"Authorization": f"Bearer {token}"}
    except:
        pass
    
    # If registration failed (user exists), try to login
    try:
        login_response = requests.post(
            f"{API_BASE}/auth/login",
            json={
                "username": user.username,
                "password": user.password
            },
            timeout=10
        )
        
        if login_response.status_code == 200:
            data = login_response.json()
            token = data.get("token")
            if token:
                return {"Authorization": f"Bearer {token}"}
    except:
        pass
    
    # If JWT auth fails, try Basic auth (docker client style)
    # Encode username:password in base64
    credentials = f"{user.username}:{user.password}"
    encoded_credentials = base64.b64encode(credentials.encode()).decode()
    return {"Authorization": f"Basic {encoded_credentials}"}
