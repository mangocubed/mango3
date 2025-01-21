CREATE TABLE post_comments (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    post_id uuid NOT NULL,
    user_id uuid NOT NULL,
    content text NOT NULL,
    language regconfig NOT NULL DEFAULT 'english',
    search tsvector GENERATED ALWAYS AS (to_tsvector(language, content)) STORED,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_post_comments PRIMARY KEY (id),
    CONSTRAINT fkey_post_comments_to_posts FOREIGN KEY (post_id) REFERENCES posts (id),
    CONSTRAINT fkey_post_comments_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX index_post_comments_on_search ON post_comments USING gin (search);

SELECT manage_updated_at('post_comments');
SELECT manage_versions('post_comments');
