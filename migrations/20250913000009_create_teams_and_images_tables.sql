-- Create teams table
CREATE TABLE teams (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    organization_id BIGINT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, name)
);

-- Create images table
CREATE TABLE images (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    digest VARCHAR(255) NOT NULL,
    tag VARCHAR(255),
    size_bytes BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, digest)
);

-- Create indexes
CREATE INDEX idx_teams_org_id ON teams(organization_id);
CREATE INDEX idx_images_repo_id ON images(repository_id);
CREATE INDEX idx_images_digest ON images(digest);
CREATE INDEX idx_images_tag ON images(tag);

-- Now add the foreign key constraint for repository_permissions.team_id
ALTER TABLE repository_permissions 
ADD CONSTRAINT repository_permissions_team_id_fkey 
FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE;