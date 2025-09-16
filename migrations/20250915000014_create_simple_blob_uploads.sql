-- Create simplified blob_uploads table (only essential fields)
CREATE TABLE IF NOT EXISTS blob_uploads (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) NOT NULL UNIQUE,
    repository_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_blob_uploads_uuid ON blob_uploads(uuid);
CREATE INDEX IF NOT EXISTS idx_blob_uploads_repository ON blob_uploads(repository_id);
CREATE INDEX IF NOT EXISTS idx_blob_uploads_user ON blob_uploads(user_id);
CREATE INDEX IF NOT EXISTS idx_blob_uploads_created_at ON blob_uploads(created_at DESC);
