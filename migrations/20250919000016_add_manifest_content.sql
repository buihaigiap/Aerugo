-- Add content field to manifests table to store manifest JSON when S3 fails
ALTER TABLE manifests ADD COLUMN content TEXT;
