CREATE TABLE blobs (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    file_name varchar NOT NULL,
    content_type varchar NOT NULL,
    byte_size bigint NOT NULL,
    md5_checksum varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_blobs PRIMARY KEY (id),
    CONSTRAINT fkey_blobs_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX index_blobs_on_user_id_content_type_byte_size_md5_checksum ON blobs
USING btree (user_id, content_type, byte_size, md5_checksum);

SELECT manage_updated_at('blobs');
SELECT manage_versions('blobs');
