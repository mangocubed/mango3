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

CREATE UNIQUE INDEX index_post_attachments_on_post_id_blob_id ON post_attachments USING btree (post_id, blob_id);

SELECT manage_updated_at('post_attachments');
SELECT manage_versions('post_attachments');
