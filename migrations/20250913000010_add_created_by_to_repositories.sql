-- Add created_by field to repositories table to track who created each repository
ALTER TABLE repositories 
ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL;

-- Add index for faster lookups by creator
CREATE INDEX idx_repositories_created_by ON repositories(created_by);

-- Update existing repositories to have created_by = NULL (will be handled by application logic)
-- In a production environment, you might want to set this to a specific admin user