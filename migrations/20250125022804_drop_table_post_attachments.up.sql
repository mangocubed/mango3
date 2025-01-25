ALTER TABLE posts ADD COLUMN blob_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [];

UPDATE posts SET blob_ids = ARRAY(
    SELECT pa.blob_id FROM post_attachments AS pa
    WHERE pa.post_id = posts.id
);

DROP TABLE post_attachments;
