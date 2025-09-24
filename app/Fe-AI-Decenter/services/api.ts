import axios from "axios";
import { API_BASE_URL } from "../config";
import {
  Organization,
  Repository,
  VerifyOtpRequest,
  CreateOrganizationRequest,
  UpdateOrganizationRequest,
  OrganizationMember,
  AddMemberRequest,
  CreateRepositoryRequest,
  User,
  OrganizationRole,
  ChangePasswordRequest,
  ForgotPasswordRequest,
} from "../types";

interface OrganizationsApiResponse {
  organizations: Organization[];
}

interface OrganizationDetailsApiResponse {
  organization: Organization;
}

interface OrganizationMembersApiResponse {
  members: OrganizationMember[];
}

interface RepositoriesApiResponse {
  repositories: Repository[];
}

class ApiError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = "ApiError";
  }
}

const handleError = (error: any): never => {
  if (axios.isAxiosError(error)) {
    const status = error.response?.status || 500;
    const message =
      error.response?.data?.message ||
      error.response?.data?.error ||
      error.message;
    throw new ApiError(message, status);
  }
  throw new ApiError("An unexpected error occurred", 500);
};

const getAuthHeaders = (token: string) => ({
  headers: { Authorization: `Bearer ${token}` },
});

export const loginUser = async (
  data: AuthRequest
): Promise<{ token: string }> => {
  try {
    const { email, password } = data;
    const response = await axios.post<{ token: string }>(
      `${API_BASE_URL}/api/v1/auth/login`,
      { email, password }
    );
    return response.data;
  } catch (error) {
    handleError(error);
    throw error;
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
    const response = await axios.get<User>(
      `${API_BASE_URL}/api/v1/auth/me`,
      getAuthHeaders(token)
    );
    return response.data;
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const changePassword = async (
  data: ChangePasswordRequest,
  token: string
): Promise<void> => {
  try {
    await axios.put(
      `${API_BASE_URL}/api/v1/auth/change-password`,
      data,
      getAuthHeaders(token)
    );
  } catch (error) {
    handleError(error);
  }
};

export const forgotPassword = async (
  data: ForgotPasswordRequest
): Promise<void> => {
  try {
    await axios.post(`${API_BASE_URL}/api/v1/auth/forgot-password`, data);
  } catch (error) {
    handleError(error);
  }
};

export const VerifyOtpAndResetPassword = async (
  data: VerifyOtpRequest
): Promise<void> => {
  try {
    await axios.post(`${API_BASE_URL}/api/v1/auth/verify-otp`, data);
  } catch (error) {
    handleError(error);
  }
};

//Organization APIs
export const fetchOrganizations = async (
  token: string
): Promise<Organization[]> => {
  try {
    const response = await axios.get<OrganizationsApiResponse>(
      `${API_BASE_URL}/api/v1/organizations`,
      getAuthHeaders(token)
    );
    return response.data?.organizations || [];
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const fetchOrganizationDetails = async (
  orgId: number,
  token: string
): Promise<Organization> => {
  try {
    const response = await axios.get<OrganizationDetailsApiResponse>(
      `${API_BASE_URL}/api/v1/organizations/${orgId}`,
      getAuthHeaders(token)
    );
    return response.data.organization;
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const createOrganization = async (
  data: CreateOrganizationRequest,
  token: string
): Promise<Organization> => {
  try {
    const response = await axios.post<Organization>(
      `${API_BASE_URL}/api/v1/organizations`,
      data,
      getAuthHeaders(token)
    );
    return response.data;
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const updateOrganization = async (
  orgId: number,
  data: UpdateOrganizationRequest,
  token: string
): Promise<Organization> => {
  try {
    const reponse = await axios.put<Organization>(
      `${API_BASE_URL}/api/v1/organizations/${orgId}`,
      data,
      getAuthHeaders(token)
    );
    return reponse.data;
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const deleteOrganization = async (
  orgId: number,
  token: string
): Promise<void> => {
  try {
    await axios.delete(
      `${API_BASE_URL}/api/v1/organizations/${orgId}`,
      getAuthHeaders(token)
    );
  } catch (error) {
    handleError(error);
  }
};

// Repository APIs
export const fetchRepositories = async (
  token: string
): Promise<Repository[]> => {
  try {
    const response = await axios.get<RepositoriesApiResponse>(
      `${API_BASE_URL}/api/v1/repos/repositories`,
      getAuthHeaders(token)
    );
    return response.data?.repositories || [];
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const fetchRepositoriesByNamespace = async (
  namespace: string,
  token: string
): Promise<Repository[]> => {
  try {
    const response = await axios.get<Repository[]>(
      `${API_BASE_URL}/api/v1/repos/repositories/${namespace}`,
      getAuthHeaders(token)
    );
    return response.data || [];
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const createRepository = async (
  namespace: string,
  data: CreateRepositoryRequest,
  token: string
): Promise<Repository> => {
  return fetchWithAuth<Repository>(`/api/v1/repos/${namespace}`, token, {
    method: "POST",
    body: JSON.stringify(data),
  });
};

export const deleteRepository = async (
  namespace: string,
  repoName: string,
  token: string
): Promise<void> => {
  try {
    await axios.delete(
      `${API_BASE_URL}/api/v1/repos/${namespace}/${repoName}`,
      getAuthHeaders(token)
    );
  } catch (error) {
    handleError(error);
  }
};

// Organization Members APIs
export const fetchOrganizationMembers = async (
  orgId: number,
  token: string
): Promise<OrganizationMember[]> => {
  try {
    const response = await axios.get<OrganizationMembersApiResponse>(
      `${API_BASE_URL}/api/v1/organizations/${orgId}/members`,
      getAuthHeaders(token)
    );
    return response.data?.members || [];
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const addOrganizationMember = async (
  orgId: number,
  data: AddMemberRequest,
  token: string
): Promise<OrganizationMember> => {
  try {
    const response = await axios.post<OrganizationMember>(
      `${API_BASE_URL}/api/v1/organizations/${orgId}/members`,
      data,
      getAuthHeaders(token)
    );
    return response.data;
  } catch (error) {
    handleError(error);
    throw error;
  }
};

export const updateMemberRole = async (
  orgId: number,
  memberId: number,
  role: OrganizationRole,
  token: string
): Promise<void> => {
  try {
    await axios.put(
      `${API_BASE_URL}/api/v1/organizations/${orgId}/members/${memberId}`,
      { role },
      getAuthHeaders(token)
    );
  } catch (error) {
    handleError(error);
  }
};

export const deleteMember = async (
  orgId: number,
  memberId: number,
  token: string
): Promise<void> => {
  try {
    await axios.delete(
      `${API_BASE_URL}/api/v1/organizations/${orgId}/members/${memberId}`,
      getAuthHeaders(token)
    );
  } catch (error) {
    handleError(error);
  }
};
