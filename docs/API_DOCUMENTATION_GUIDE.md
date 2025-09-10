# API Documentation Guide for Frontend Developers

## Introduction

This document provides guidance on how to use the Aerugo API documentation to help frontend developers understand and interact with the API endpoints.

## Accessing the API Documentation

The API documentation is available through Swagger UI, which can be accessed at:

```
http://<server-address>/docs
```

This interactive documentation provides:
- A complete list of all API endpoints
- Request and response schemas
- Authentication requirements
- Example requests and responses
- The ability to try out API calls directly from the documentation

## Authentication

Most API endpoints require authentication. To authenticate:

1. Create an account using the `/api/v1/auth/register` endpoint
2. Login using the `/api/v1/auth/login` endpoint to get a JWT token
3. Include the token in the `Authorization` header for subsequent requests:
   ```
   Authorization: Bearer <your-jwt-token>
   ```

## Common Workflows

### User Management
- Register a new user: `POST /api/v1/auth/register`
- Login: `POST /api/v1/auth/login`

### Organization Management
- Create an organization: `POST /api/v1/orgs`
- List organizations: `GET /api/v1/orgs`
- Get organization details: `GET /api/v1/orgs/{org_id}`
- Update organization: `PUT /api/v1/orgs/{org_id}`
- Delete organization: `DELETE /api/v1/orgs/{org_id}`

### Container Registry Operations
- List repositories: `GET /api/v1/orgs/{org_id}/repos`
- Get repository details: `GET /api/v1/orgs/{org_id}/repos/{repo_name}`
- List images in repository: `GET /api/v1/orgs/{org_id}/repos/{repo_name}/images`

## Best Practices

1. Always check the response status codes for error handling
2. Use proper error handling for failed API requests
3. Implement token refresh mechanisms for long-running sessions
4. Cache responses when appropriate to reduce API load

## Example Integration (JavaScript)

```javascript
// Example: Login and fetch organizations
async function login(username, password) {
  const response = await fetch('/api/v1/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ username, password })
  });
  
  if (!response.ok) {
    throw new Error('Login failed');
  }
  
  const data = await response.json();
  return data.token;
}

async function getOrganizations(token) {
  const response = await fetch('/api/v1/orgs', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  
  if (!response.ok) {
    throw new Error('Failed to fetch organizations');
  }
  
  return response.json();
}
```

## Need Help?

If you encounter issues or have questions about the API:
- Refer to the detailed documentation in the Swagger UI
- Check the error messages and status codes returned by the API
- Contact the backend development team for assistance
