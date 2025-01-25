CREATE TABLE post_attachments (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    post_id uuid NOT NULL,
    blob_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_post_attachments PRIMARY KEY (id),
    CONSTRAINT fkey_post_attachments_to_posts FOREIGN KEY (post_id) REFERENCES posts (id),
    CONSTRAINT fkey_post_attachments_to_blobs FOREIGN KEY (blob_id) REFERENCES blobs (id)
);

INSERT INTO post_attachments (post_id, blob_id)
SELECT
    id AS post_id,
    unnest(blob_ids) AS blob_id
FROM posts
WHERE array_length(blob_ids, 1) > 0;

ALTER TABLE posts DROP COLUMN blob_ids;
