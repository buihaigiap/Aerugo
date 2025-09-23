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
  avatar_url: string | null;
  website_url: string | null;
  created_at?: string;
  updated_at?: string;
}

export interface CreateOrganizationRequest {
  name: string;
  display_name: string;
  description: string;
  avatar_url?: string;
  website_url?: string;
}

export interface UpdateOrganizationRequest {
  display_name: string;
  description: string;
  avatar_url?: string;
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

export interface ImageLayer {
  digest: string;
  size: string;
}

export interface ImageHistoryItem {
  command: string;
  details: string;
}

export interface ImageConfig {
  created: string;
  dockerVersion: string;
  osArch: string;
  labels: Record<string, string>;
}

export interface ImageTag {
  name: string;
  digest: string;
  osArch: string;
  size: string;
  pushedAt: string;
  // Detailed information for the tag detail view
  config: ImageConfig;
  layers: ImageLayer[];
  history: ImageHistoryItem[];
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
  tags: ImageTag[];
  user_permissions: UserPermission[];
  org_permissions: OrgPermission[];
}
