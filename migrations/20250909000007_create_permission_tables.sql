-- Repository permissions
CREATE TYPE repository_permission AS ENUM ('read', 'write', 'admin');

-- Repository access control table
CREATE TABLE repository_permissions (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    -- Either user_id or organization_id should be set, not both
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    organization_id BIGINT REFERENCES organizations(id) ON DELETE CASCADE,
    permission repository_permission NOT NULL,
    granted_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Ensure either user_id or organization_id is set, but not both
    CHECK ((user_id IS NOT NULL AND organization_id IS NULL) OR 
           (user_id IS NULL AND organization_id IS NOT NULL)),
    -- Ensure unique permission per user/org and repository
    UNIQUE(repository_id, user_id),
    UNIQUE(repository_id, organization_id)
);

-- Create indexes for faster permission lookups
CREATE INDEX idx_repository_permissions_user ON repository_permissions(repository_id, user_id) 
    WHERE user_id IS NOT NULL;
CREATE INDEX idx_repository_permissions_org ON repository_permissions(repository_id, organization_id) 
    WHERE organization_id IS NOT NULL;

-- Table for tracking repository access audit logs
CREATE TABLE repository_access_logs (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL, -- e.g., 'pull', 'push', 'delete'
    details JSONB,
    ip_address VARCHAR(45), -- IPv4 or IPv6 address
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_repository_access_logs_repo ON repository_access_logs(repository_id);
CREATE INDEX idx_repository_access_logs_user ON repository_access_logs(user_id);
CREATE INDEX idx_repository_access_logs_time ON repository_access_logs(created_at);
