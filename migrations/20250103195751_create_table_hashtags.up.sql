CREATE TABLE hashtags (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    name citext NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_hashtags PRIMARY KEY (id)
);

CREATE UNIQUE INDEX index_hashtags_on_name ON hashtags USING btree (name);

SELECT manage_updated_at('hashtags');
SELECT manage_versions('hashtags');

ALTER TABLE posts ADD COLUMN hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [];

CREATE INDEX index_posts_on_hashtag_ids ON posts USING gin (hashtag_ids);

ALTER TABLE websites ADD COLUMN hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [];

CREATE INDEX index_websites_on_hashtag_ids ON websites USING gin (hashtag_ids);

ALTER TABLE users ADD COLUMN hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [];

CREATE INDEX index_users_on_hashtag_ids ON users USING gin (hashtag_ids);
