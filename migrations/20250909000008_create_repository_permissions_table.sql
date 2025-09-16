CREATE TABLE IF NOT EXISTS repository_permissions (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    team_id BIGINT, -- Will add foreign key constraint after teams table is created
    permission VARCHAR(10) NOT NULL CHECK (permission IN ('read', 'write', 'admin')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_or_team_permission CHECK (
        (user_id IS NOT NULL AND team_id IS NULL) OR 
        (user_id IS NULL AND team_id IS NOT NULL)
    ),
    CONSTRAINT unique_user_repo_permission UNIQUE(repository_id, user_id),
    CONSTRAINT unique_team_repo_permission UNIQUE(repository_id, team_id)
);