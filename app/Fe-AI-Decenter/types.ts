// Fix: Replaced incorrect file content with proper type definitions.
export enum AuthMode {
  Login = "login",
  Register = "register",
}

export enum OrganizationRole {
  Owner = "Owner",
  Admin = "Admin",
  Member = "Member",
}

export interface User {
  id: number;
  username: string;
  email: string;
}

// FIX: Replaced LoginRequest and RegisterRequest with a single AuthRequest type for consistency as requested by the user.
export interface AuthRequest {
  username?: string;
  email: string;
  password: string;
}

export interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
  confirm_password: string;
}

export interface ForgotPasswordRequest {
  email: string;
}

export interface VerifyOtpRequest {
  email: string;
  otp_code: string;
  new_password: string;
  confirm_password: string;
}

export interface Organization {
  id: number;
  name: string;
  display_name: string;
  description: string | null;
  website_url: string | null;
  created_at?: string;
  updated_at?: string;
}

export interface CreateOrganizationRequest {
  name: string;
  display_name: string;
  description: string;
  website_url?: string;
}

export interface UpdateOrganizationRequest {
  display_name: string;
  description: string;
  website_url?: string;
}

export interface OrganizationMember {
  id: number; // This is the membership ID
  user_id: number; // This is the user's ID
  username: string;
  email: string;
  role: string;
}

export interface AddMemberRequest {
  email: string;
  role: OrganizationRole;
}

export interface Repository {
  id: number;
  name: string;
  description: string | null;
  is_public: boolean;
  organization: Organization;
  organization_id: number;
  created_at: string;
  updated_at: string;
  created_by: number | null;
}

export interface CreateRepositoryRequest {
  name: string;
  description: string | null;
  is_public: boolean;
}

export interface UpdateRepositoryRequest {
  name: string;
  description: string | null;
  is_public: boolean;
}

export interface UserPermission {
  user_id: number;
  permission: string;
}

export interface OrgPermission {
  organization_id: number;
  permission: string;
}

export interface RepositoryDetailsResponse {
  repository: Repository;
  tags: string[];
  user_permissions: UserPermission[];
  org_permissions: OrgPermission[];
}

// FIX: Added missing Webhook type to fix compilation error in RepositoryWebhooks.tsx
export interface Webhook {
  id: number;
  url: string;
  lastDelivery?: {
    status: "success" | "failed";
    timestamp: string;
  };
}

export interface ApiKey {
  id: number;
  name: string;
  created_at: string;
  expires_at: string | null;
  last_used_at: string | null;
  is_active: boolean;
}

export interface CreateApiKeyRequest {
  name: string;
}

export interface CreateApiKeyResponse {
  id: number;
  api_key: string;
  created_at: string;
  expires_at: string | null;
  warning: string;
}
