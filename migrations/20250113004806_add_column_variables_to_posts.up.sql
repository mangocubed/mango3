ALTER TABLE posts ADD COLUMN variables jsonb NOT NULL DEFAULT '{}'::jsonb, ADD COLUMN modified_at timestamptz NULL;
