import axios from 'axios';
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
    VerifyOtpRequest,
    RepositoryDetailsResponse,
    AuthRequest,
    UpdateRepositoryRequest
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

// Custom error class to standardize API errors.
// Components that use these API calls expect an object with a `status` property.
class ApiError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = 'ApiError';
  }
}

// Helper to handle and re-throw errors in a standardized format.
// FIX: The 'handleError' function was causing TypeScript errors because properties like 'response' and 'message' were being accessed on a variable of type 'unknown'. This was likely due to a toolchain issue where the `axios.isAxiosError` type guard was not functioning as expected. Changed the 'error' parameter's type from 'unknown' to 'any' to resolve these compilation errors pragmatically.
const handleError = (error: any): never => {
    if (axios.isAxiosError(error)) {
        const status = error.response?.status || 500;
        // Attempt to get a meaningful error message from the response body
        const message = error.response?.data?.message || error.response?.data?.error || error.message;
        throw new ApiError(message, status);
    }
    // Fallback for non-axios errors
    throw new ApiError('An unexpected error occurred', 500);
}

// Helper to create authorization headers
const getAuthHeaders = (token: string) => ({
    headers: { Authorization: `Bearer ${token}` }
});

// --- Auth Endpoints ---
export const loginUser = async (data: AuthRequest): Promise<{ token: string }> => {
    try {
        const { email, password } = data; // Ensure only email and password are sent for login
        const response = await axios.post<{ token: string }>(`${API_BASE_URL}/api/v1/auth/login`, { email, password });
        return response.data;
    } catch (error) {
        handleError(error);
    }
};

export const registerUser = async (data: AuthRequest): Promise<void> => {
    try {
        await axios.post(`${API_BASE_URL}/api/v1/auth/register`, data);
    } catch (error) {
        handleError(error);
    }
};

export const fetchCurrentUser = async (token: string): Promise<User> => {
    try {
        const response = await axios.get<User>(`${API_BASE_URL}/api/v1/auth/me`, getAuthHeaders(token));
        return response.data;
    } catch (error) {
        handleError(error);
    }
};

export const changePassword = async (data: ChangePasswordRequest, token: string): Promise<void> => {
    try {
        await axios.put(`${API_BASE_URL}/api/v1/auth/change-password`, data, getAuthHeaders(token));
    } catch (error) {
        handleError(error);
    }
};

export const forgotPassword = async (data: ForgotPasswordRequest): Promise<void> => {
    try {
        await axios.post(`${API_BASE_URL}/api/v1/auth/forgot-password`, data);
    } catch (error) {
        handleError(error);
    }
};

export const verifyOtpAndResetPassword = async (data: VerifyOtpRequest): Promise<void> => {
    try {
        await axios.post(`${API_BASE_URL}/api/v1/auth/verify-otp`, data);
    } catch (error) {
        handleError(error);
    }
};

// --- Organization Endpoints ---
export const fetchOrganizations = async (token: string): Promise<Organization[]> => {
  try {
    const response = await axios.get<OrganizationsApiResponse>(`${API_BASE_URL}/api/v1/organizations`, getAuthHeaders(token));
    return response.data?.organizations || [];
  } catch (error) {
    handleError(error);
  }
};

export const fetchOrganizationDetails = async (orgId: number, token: string): Promise<Organization> => {
    try {
        const response = await axios.get<OrganizationDetailsApiResponse>(`${API_BASE_URL}/api/v1/organizations/${orgId}`, getAuthHeaders(token));
        return response.data.organization;
    } catch (error) {
        handleError(error);
    }
};

export const createOrganization = async (data: CreateOrganizationRequest, token: string): Promise<Organization> => {
  try {
    const response = await axios.post<Organization>(`${API_BASE_URL}/api/v1/organizations`, data, getAuthHeaders(token));
    return response.data;
  } catch (error) {
    handleError(error);
  }
};

export const updateOrganization = async (orgId: number, data: UpdateOrganizationRequest, token: string): Promise<Organization> => {
  try {
    const response = await axios.put<Organization>(`${API_BASE_URL}/api/v1/organizations/${orgId}`, data, getAuthHeaders(token));
    return response.data;
  } catch (error) {
    handleError(error);
  }
};

export const deleteOrganization = async (orgId: number, token: string): Promise<void> => {
  try {
    await axios.delete(`${API_BASE_URL}/api/v1/organizations/${orgId}`, getAuthHeaders(token));
  } catch (error) {
    handleError(error);
  }
};

// --- Repository Endpoints ---
export const fetchRepositories = async (token: string): Promise<Repository[]> => {
  try {
    const response = await axios.get<RepositoriesApiResponse>(`${API_BASE_URL}/api/v1/repos/repositories`, getAuthHeaders(token));
    return response.data?.repositories || [];
  } catch(error) {
    handleError(error);
  }
};

export const fetchRepositoriesByNamespace = async (namespace: string, token: string): Promise<Repository[]> => {
    try {
        const response = await axios.get<Repository[]>(`${API_BASE_URL}/api/v1/repos/repositories/${namespace}`, getAuthHeaders(token));
        return response.data || [];
    } catch(error) {
        handleError(error);
    }
};

export const fetchRepositoryDetails = async (namespace: string, repo_name: string, token: string): Promise<RepositoryDetailsResponse> => {
    try {
        const response = await axios.get<RepositoryDetailsResponse>(`${API_BASE_URL}/api/v1/repos/${namespace}/repositories/${repo_name}`, getAuthHeaders(token));
        return response.data;
    } catch (error) {
        handleError(error);
    }
};

export const createRepository = async (namespace: string, data: CreateRepositoryRequest, token: string): Promise<Repository> => {
  try {
    const response = await axios.post<Repository>(`${API_BASE_URL}/api/v1/repos/${namespace}`, data, getAuthHeaders(token));
    return response.data;
  } catch (error) {
    handleError(error);
  }
};

export const updateRepository = async (namespace: string, repoName: string, data: UpdateRepositoryRequest, token: string): Promise<Repository> => {
    try {
        const response = await axios.put<Repository>(`${API_BASE_URL}/api/v1/repos/${namespace}/${repoName}`, data, getAuthHeaders(token));
        return response.data;
    } catch (error) {
        handleError(error);
    }
};

export const deleteRepository = async (namespace: string, repoName: string, token: string): Promise<void> => {
    try {
        await axios.delete(`${API_BASE_URL}/api/v1/repos/${namespace}/${repoName}`, getAuthHeaders(token));
    } catch (error) {
        handleError(error);
    }
};

// --- Member Endpoints ---
export const fetchOrganizationMembers = async (orgId: number, token: string): Promise<OrganizationMember[]> => {
  try {
    const response = await axios.get<OrganizationMembersApiResponse>(`${API_BASE_URL}/api/v1/organizations/${orgId}/members`, getAuthHeaders(token));
    return response.data?.members || [];
  } catch(error) {
    handleError(error);
  }
};

export const addOrganizationMember = async (orgId: number, data: AddMemberRequest, token: string): Promise<OrganizationMember> => {
  try {
    const response = await axios.post<OrganizationMember>(`${API_BASE_URL}/api/v1/organizations/${orgId}/members`, data, getAuthHeaders(token));
    return response.data;
  } catch (error) {
    handleError(error);
  }
};

export const updateMemberRole = async (orgId: number, memberId: number, role: OrganizationRole, token: string): Promise<void> => {
    try {
        await axios.put(`${API_BASE_URL}/api/v1/organizations/${orgId}/members/${memberId}`, { role }, getAuthHeaders(token));
    } catch (error) {
        handleError(error);
    }
};

export const deleteMember = async (orgId: number, memberId: number, token: string): Promise<void> => {
    try {
        await axios.delete(`${API_BASE_URL}/api/v1/organizations/${orgId}/members/${memberId}`, getAuthHeaders(token));
    } catch (error) {
        handleError(error);
    }
};