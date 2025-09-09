-- Manifests table for container image metadata
CREATE TABLE manifests (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    digest VARCHAR(255) NOT NULL,
    media_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, digest)
);

-- Blobs table for storing layer data
CREATE TABLE blobs (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    digest VARCHAR(255) NOT NULL,
    media_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, digest)
);

-- Tags table for image references
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    repository_id BIGINT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    manifest_id BIGINT NOT NULL REFERENCES manifests(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, name)
);

-- Manifest layers relationship table
CREATE TABLE manifest_layers (
    manifest_id BIGINT NOT NULL REFERENCES manifests(id) ON DELETE CASCADE,
    blob_id BIGINT NOT NULL REFERENCES blobs(id) ON DELETE CASCADE,
    layer_order INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (manifest_id, blob_id),
    UNIQUE(manifest_id, layer_order)
);

-- Create indexes for better query performance
CREATE INDEX idx_manifests_repository_digest ON manifests(repository_id, digest);
CREATE INDEX idx_blobs_repository_digest ON blobs(repository_id, digest);
CREATE INDEX idx_tags_repository_name ON tags(repository_id, name);
CREATE INDEX idx_manifest_layers_manifest_id ON manifest_layers(manifest_id);
