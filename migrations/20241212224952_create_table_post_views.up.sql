CREATE TABLE post_views (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    post_id uuid NOT NULL,
    user_id uuid NULL,
    ip_address cidr NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_post_views PRIMARY KEY (id),
    CONSTRAINT fkey_post_views_to_posts FOREIGN KEY (post_id) REFERENCES posts (id),
    CONSTRAINT fkey_post_views_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX index_post_views_on_post_id_user_id ON post_views USING btree (post_id, user_id)
WHERE user_id IS NOT NULL;

CREATE UNIQUE INDEX index_post_views_on_post_id_ip_address ON post_views USING btree (post_id, ip_address)
WHERE user_id IS NULL;

SELECT manage_updated_at('post_views');
SELECT manage_versions('post_views');
