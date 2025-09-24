import { API_BASE_URL } from '../config';
import {
  Organization,
  Repository,
  CreateOrganizationRequest,
  UpdateOrganizationRequest,
  OrganizationMember,
  AddMemberRequest,
  CreateRepositoryRequest,
  User,
  OrganizationRole,
  ChangePasswordRequest,
  ForgotPasswordRequest,
  ResetPasswordRequest
} from '../types';

// Interface to match the structure of the API response for organizations
interface OrganizationsApiResponse {
  organizations: Organization[];
}

// Interface for a single organization API response
interface OrganizationDetailsApiResponse {
  organization: Organization;
}

interface OrganizationMembersApiResponse {
  members: OrganizationMember[];
}

// Interface for repositories API response
interface RepositoriesApiResponse {
  repositories: Repository[];
}


class ApiError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = 'ApiError';
  }
}

async function fetchWithAuth<T>(
  endpoint: string,
  token: string,
  options: RequestInit = {}
): Promise<T> {
  const headers = new Headers(options.headers || {});
  headers.set('Authorization', `Bearer ${token}`);
  if (options.method === 'POST' || options.method === 'PUT' || options.method === 'PATCH' || options.method === 'DELETE') {
    headers.set('Content-Type', 'application/json');
  }

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const errorText = await response.text().catch(() => 'Failed to read error response');
    console.error(`API Error: ${response.status} ${response.statusText}`, errorText);
    throw new ApiError(`Request failed with status ${response.status}`, response.status);
  }

  // Handle cases with no response body (e.g., 204 No Content)
  if (response.status === 204) {
    return null as T;
  }

  return response.json() as Promise<T>;
}

// Placeholder for public fetch calls (no auth token needed)
async function fetchPublic<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const headers = new Headers(options.headers || {});
  if (options.method === 'POST' || options.method === 'PUT') {
    headers.set('Content-Type', 'application/json');
  }

  const response = await fetch(`${API_BASE_URL}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const errorText = await response.text().catch(() => 'Failed to read error response');
    console.error(`API Error: ${response.status} ${response.statusText}`, errorText);
    throw new ApiError(`Request failed with status ${response.status}`, response.status);
  }

  if (response.status === 204) {
    return null as T;
  }

  return response.json() as Promise<T>;
}


export const fetchCurrentUser = (token: string): Promise<User> => {
  return fetchWithAuth<User>('/api/v1/auth/me', token);
};

export const changePassword = (data: ChangePasswordRequest, token: string): Promise<void> => {
  return fetchWithAuth<void>('/api/v1/auth/change-password', token, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
};

// Placeholder for requesting a password reset email
export const forgotPassword = (data: ForgotPasswordRequest): Promise<void> => {
  // This should eventually be a real API call to a corrected backend
  console.log('Simulating sending password reset for', data.email);
  return new Promise(resolve => setTimeout(resolve, 1000));
  // Example of real implementation:
  // return fetchPublic<void>('/api/v1/auth/request-password-reset', {
  //     method: 'POST',
  //     body: JSON.stringify(data),
  // });
};

// Placeholder for resetting the password with a token
export const resetPassword = (data: ResetPasswordRequest): Promise<void> => {
  // This should eventually be a real API call to a corrected backend
  console.log('Simulating resetting password with token', data.token);
  return new Promise(resolve => setTimeout(resolve, 1000));
  // Example of real implementation:
  // return fetchPublic<void>('/api/v1/auth/reset-password', {
  //     method: 'POST',
  //     body: JSON.stringify(data),
  // });
};


export const fetchOrganizations = async (token: string): Promise<Organization[]> => {
  const data = await fetchWithAuth<OrganizationsApiResponse>('/api/v1/organizations', token);
  return data?.organizations || [];
};

export const fetchOrganizationDetails = async (orgId: number, token: string): Promise<Organization> => {
  const data = await fetchWithAuth<OrganizationDetailsApiResponse>(`/api/v1/organizations/${orgId}`, token);
  return data.organization;
};

export const createOrganization = (data: CreateOrganizationRequest, token: string): Promise<Organization> => {
  return fetchWithAuth<Organization>('/api/v1/organizations', token, {
    method: 'POST',
    body: JSON.stringify(data),
  });
};

export const updateOrganization = (orgId: number, data: UpdateOrganizationRequest, token: string): Promise<Organization> => {
  return fetchWithAuth<Organization>(`/api/v1/organizations/${orgId}`, token, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
};

export const deleteOrganization = (orgId: number, token: string): Promise<void> => {
  return fetchWithAuth<void>(`/api/v1/organizations/${orgId}`, token, {
    method: 'DELETE',
  });
};

export const fetchRepositories = async (token: string): Promise<Repository[]> => {
  const data = await fetchWithAuth<RepositoriesApiResponse>(`/api/v1/repos/repositories`, token);
  return data?.repositories || [];
};

export const fetchRepositoriesByNamespace = async (namespace: string, token: string): Promise<Repository[]> => {
  const data = await fetchWithAuth<Repository[]>(`/api/v1/repos/repositories/${namespace}`, token);
  return data || [];
};

export const createRepository = (namespace: string, data: CreateRepositoryRequest, token: string): Promise<Repository> => {
  return fetchWithAuth<Repository>(`/api/v1/repos/${namespace}`, token, {
    method: 'POST',
    body: JSON.stringify(data),
  });
};

export const deleteRepository = (namespace: string, repoName: string, token: string): Promise<void> => {
  return fetchWithAuth<void>(`/api/v1/repos/${namespace}/${repoName}`, token, {
    method: 'DELETE',
  });
};

export const fetchOrganizationMembers = async (orgId: number, token: string): Promise<OrganizationMember[]> => {
  // The API returns an object { members: [...] }, so we extract the array.
  const data = await fetchWithAuth<OrganizationMembersApiResponse>(`/api/v1/organizations/${orgId}/members`, token);
  return data?.members || [];
};

export const addOrganizationMember = (orgId: number, data: AddMemberRequest, token: string): Promise<OrganizationMember> => {
  return fetchWithAuth<OrganizationMember>(`/api/v1/organizations/${orgId}/members`, token, {
    method: 'POST',
    body: JSON.stringify(data),
  });
};

export const updateMemberRole = (orgId: number, memberId: number, role: OrganizationRole, token: string): Promise<void> => {
  return fetchWithAuth<void>(`/api/v1/organizations/${orgId}/members/${memberId}`, token, {
    method: 'PUT',
    body: JSON.stringify({ role }),
  });
};

export const deleteMember = (orgId: number, memberId: number, token: string): Promise<void> => {
  return fetchWithAuth<void>(`/api/v1/organizations/${orgId}/members/${memberId}`, token, {
    method: 'DELETE',
  });
};