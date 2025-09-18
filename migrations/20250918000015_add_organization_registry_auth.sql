-- Add registry authentication fields to organizations table
-- This supports organization-level registry credentials for Docker login

ALTER TABLE organizations 
ADD COLUMN IF NOT EXISTS registry_username VARCHAR(255) UNIQUE,
ADD COLUMN IF NOT EXISTS registry_password_hash VARCHAR(255);

-- Create index for registry username lookups
CREATE INDEX IF NOT EXISTS idx_organizations_registry_username 
ON organizations(registry_username) 
WHERE registry_username IS NOT NULL;

-- Add comments to document the purpose of these fields
COMMENT ON COLUMN organizations.registry_username IS 'Username for Docker Registry authentication at organization level';
COMMENT ON COLUMN organizations.registry_password_hash IS 'Bcrypt hash of Docker Registry password for organization level authentication';
