-- Create API keys table for API key authentication
CREATE TABLE api_keys (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL, -- User-defined name for the API key
    key_hash VARCHAR(255) NOT NULL UNIQUE, -- SHA-256 hash of the API key
    permissions TEXT[] DEFAULT '{"read"}', -- Array of permissions: read, write, admin
    last_used_at TIMESTAMP,
    expires_at TIMESTAMP, -- Optional expiration date
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- Index for faster lookups by key_hash
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);

-- Index for user_id lookups
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

-- Index for active keys only
CREATE INDEX idx_api_keys_active ON api_keys(is_active) WHERE is_active = true;

-- Update updated_at trigger
CREATE OR REPLACE FUNCTION update_api_keys_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_api_keys_updated_at
    BEFORE UPDATE ON api_keys
    FOR EACH ROW
    EXECUTE FUNCTION update_api_keys_updated_at();