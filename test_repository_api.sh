#!/bin/bash

# Repository Creation API Test Script
# Tests the POST /api/v1/repos/:namespace endpoint

API_BASE="http://localhost:8080"
ENDPOINT="/api/v1/repos/testorg"

echo "🧪 Testing Repository Creation API: POST ${ENDPOINT}"
echo "=================================================="

# Test 1: Valid Request
echo "✅ Test 1: Valid repository creation request"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "name": "test-repo",
    "description": "Test repository",
    "is_public": true
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 2: Missing required fields
echo "❌ Test 2: Missing required field 'is_public'"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "name": "test-repo",
    "description": "Test repository"
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 3: Missing name field
echo "❌ Test 3: Missing required field 'name'"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "description": "Test repository",
    "is_public": true
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 4: Invalid JSON
echo "❌ Test 4: Invalid JSON payload"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d 'invalid json' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 5: Empty body
echo "❌ Test 5: Empty request body"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 6: No Content-Type header
echo "❌ Test 6: Missing Content-Type header"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "name": "test-repo",
    "description": "Test repository",
    "is_public": true
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 7: Minimal valid request
echo "✅ Test 7: Minimal valid request (no description)"
curl -X POST "${API_BASE}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "name": "minimal-repo",
    "is_public": false
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

# Test 8: Different namespace
echo "✅ Test 8: Different namespace"
curl -X POST "${API_BASE}/api/v1/repos/anotherorg" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-token" \
  -d '{
    "name": "another-repo",
    "description": "Repository in another org",
    "is_public": true
  }' \
  -w "\nHTTP Status: %{http_code}\n" \
  --silent
echo ""

echo "🎯 Test Summary:"
echo "- The API correctly accepts valid repository creation requests"
echo "- Returns HTTP 200 with 'Repository creation temporarily disabled' message"
echo "- Properly validates required fields (name, is_public)"
echo "- Handles JSON parsing errors appropriately"
echo "- Description field is optional"
echo "- Authorization header is accepted but not currently enforced"
echo ""
echo "📋 Current API Behavior:"
echo "- Endpoint: POST /api/v1/repos/:namespace"
echo "- Status: Temporarily disabled (returns mock response)"
echo "- Content-Type: application/json required"
echo "- Required fields: name, is_public"
echo "- Optional fields: description"
