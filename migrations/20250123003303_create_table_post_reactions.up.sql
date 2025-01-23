CREATE TABLE post_reactions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    post_id uuid NOT NULL,
    user_id uuid NOT NULL,
    emoji varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_post_reactions PRIMARY KEY (id),
    CONSTRAINT fkey_post_reactions_to_posts FOREIGN KEY (post_id) REFERENCES posts (id),
    CONSTRAINT fkey_post_reactions_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX index_post_reactions_on_post_id_user_id ON post_reactions USING btree (post_id, user_id);
CREATE INDEX index_post_reactions_on_emoji ON post_reactions USING btree (emoji);

SELECT manage_updated_at('post_reactions');
SELECT manage_versions('post_reactions');
