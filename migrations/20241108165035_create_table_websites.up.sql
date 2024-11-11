CREATE TABLE websites (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    name citext NOT NULL,
    subdomain citext NOT NULL,
    description text NOT NULL DEFAULT '',
    icon_image_blob_id uuid NULL,
    cover_image_blob_id uuid NULL,
    published_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_websites PRIMARY KEY (id),
    CONSTRAINT fkey_websites_to_users FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fkey_websites_to_icon_image_blobs FOREIGN KEY (icon_image_blob_id) REFERENCES blobs (id),
    CONSTRAINT fkey_websites_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
);

CREATE UNIQUE INDEX index_websites_on_name ON websites USING btree (name);
CREATE UNIQUE INDEX index_websites_on_subdomain ON websites USING btree (subdomain);

SELECT manage_updated_at('websites');
SELECT manage_versions('websites');
