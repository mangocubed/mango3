CREATE TABLE pages (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    website_id uuid NOT NULL,
    user_id uuid NOT NULL,
    title citext NOT NULL,
    slug citext NOT NULL,
    content text NOT NULL DEFAULT '',
    cover_image_blob_id uuid NULL,
    published_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_pages PRIMARY KEY (id),
    CONSTRAINT fkey_pages_to_websites FOREIGN KEY (website_id) REFERENCES websites (id),
    CONSTRAINT fkey_pages_to_users FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fkey_pages_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
);

CREATE UNIQUE INDEX index_pages_on_website_id_slug ON pages USING btree (website_id, slug);

SELECT manage_updated_at('pages');
SELECT manage_versions('pages');
