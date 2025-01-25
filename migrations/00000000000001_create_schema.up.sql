DELETE FROM _sqlx_migrations;

CREATE EXTENSION IF NOT EXISTS citext;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS void AS $$
BEGIN
    EXECUTE format(
        'CREATE OR REPLACE TRIGGER set_updated_at BEFORE UPDATE ON %s FOR EACH ROW EXECUTE PROCEDURE set_updated_at()',
        _tbl
    );
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS versions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    action varchar NOT NULL,
    record_type varchar NOT NULL,
    record_id uuid NOT NULL,
    data jsonb,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_versions PRIMARY KEY (id)
);

SELECT manage_updated_at('versions');

CREATE OR REPLACE FUNCTION insert_into_versions() RETURNS trigger AS $$
DECLARE
    record_id uuid;
    data jsonb;
BEGIN
    IF (NEW IS DISTINCT FROM OLD) THEN
        IF (TG_OP IS DISTINCT FROM 'DELETE') THEN
            SELECT NEW.id INTO record_id;
            SELECT to_jsonb(NEW) INTO data;
        ELSE
            SELECT OLD.id INTO record_id;
        END IF;

        INSERT INTO versions (action, record_type, record_id, data)
            VALUES (LOWER(TG_OP), TG_TABLE_NAME, record_id, data);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION manage_versions(_tbl regclass) RETURNS void AS $$
BEGIN
    EXECUTE format(
        'CREATE OR REPLACE TRIGGER insert_into_versions AFTER INSERT OR UPDATE OR DELETE ON %s FOR EACH ROW
            EXECUTE FUNCTION insert_into_versions()',
        _tbl
   );
END;
$$ LANGUAGE plpgsql;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('user', 'creator', 'admin', 'superuser');
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS blobs (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    website_id uuid NULL,
    user_id uuid NOT NULL,
    file_name varchar NOT NULL,
    content_type varchar NOT NULL,
    byte_size bigint NOT NULL,
    md5_checksum varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_blobs PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_blobs_on_user_id_website_id_content_type_byte_size_md5_checksum ON blobs
USING btree (user_id, website_id, content_type, byte_size, md5_checksum);

SELECT manage_updated_at('blobs');
SELECT manage_versions('blobs');

CREATE TABLE IF NOT EXISTS confirmation_codes (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    encrypted_code varchar NOT NULL,
    failed_attempts smallint NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_confirmation_codes PRIMARY KEY (id)
);

SELECT manage_updated_at('confirmation_codes');
SELECT manage_versions('confirmation_codes');

CREATE TABLE IF NOT EXISTS hashtags (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    name citext NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_hashtags PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_hashtags_on_name ON hashtags USING btree (name);

SELECT manage_updated_at('hashtags');
SELECT manage_versions('hashtags');

CREATE TABLE IF NOT EXISTS invitation_codes (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    email citext NOT NULL,
    code citext NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_invitation_codes PRIMARY KEY (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_invitation_codes_on_email ON invitation_codes USING btree (email);
CREATE UNIQUE INDEX IF NOT EXISTS index_invitation_codes_on_code ON invitation_codes USING btree (code);

SELECT manage_updated_at('invitation_codes');
SELECT manage_versions('invitation_codes');

CREATE TABLE IF NOT EXISTS users (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    username citext NOT NULL,
    email citext NOT NULL,
    email_confirmation_code_id uuid NULL,
    email_confirmed_at timestamptz NULL,
    encrypted_password varchar NOT NULL,
    password_reset_confirmation_code_id uuid NULL,
    display_name varchar NOT NULL,
    full_name varchar NOT NULL,
    birthdate date NOT NULL,
    language_code varchar NOT NULL DEFAULT 'en',
    country_alpha2 varchar NOT NULL,
    bio text NOT NULL DEFAULT '',
    hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [],
    avatar_image_blob_id uuid NULL,
    role user_role NOT NULL DEFAULT 'user',
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_users PRIMARY KEY (id),
    CONSTRAINT fkey_users_to_email_confirmation_codes FOREIGN KEY (email_confirmation_code_id)
    REFERENCES confirmation_codes (id),
    CONSTRAINT fkey_users_to_password_reset_confirmation_codes FOREIGN KEY (password_reset_confirmation_code_id)
    REFERENCES confirmation_codes (id),
    CONSTRAINT fkey_users_to_avatar_image_blobs FOREIGN KEY (avatar_image_blob_id) REFERENCES blobs (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_users_on_username ON users USING btree (username);
CREATE UNIQUE INDEX IF NOT EXISTS index_users_on_email ON users USING btree (email);
CREATE INDEX IF NOT EXISTS index_users_on_hashtag_ids ON users USING gin (hashtag_ids);

SELECT manage_updated_at('users');
SELECT manage_versions('users');

CREATE TABLE IF NOT EXISTS user_sessions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_user_sessions PRIMARY KEY (id),
    CONSTRAINT fkey_user_sessions_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

SELECT manage_updated_at('user_sessions');
SELECT manage_versions('user_sessions');

CREATE TABLE IF NOT EXISTS websites (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    name citext NOT NULL,
    subdomain citext NOT NULL,
    description text NOT NULL DEFAULT '',
    hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [],
    icon_image_blob_id uuid NULL,
    cover_image_blob_id uuid NULL,
    light_theme varchar NOT NULL DEFAULT 'light',
    dark_theme varchar NOT NULL DEFAULT 'dark',
    language regconfig NOT NULL DEFAULT 'english',
    search tsvector GENERATED ALWAYS AS (to_tsvector(language, name || ' ' || description)) STORED,
    published_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_websites PRIMARY KEY (id),
    CONSTRAINT fkey_websites_to_users FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fkey_websites_to_icon_image_blobs FOREIGN KEY (icon_image_blob_id) REFERENCES blobs (id),
    CONSTRAINT fkey_websites_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_websites_on_name ON websites USING btree (name);
CREATE UNIQUE INDEX IF NOT EXISTS index_websites_on_subdomain ON websites USING btree (subdomain);
CREATE INDEX IF NOT EXISTS index_websites_on_hashtag_ids ON websites USING gin (hashtag_ids);
CREATE INDEX IF NOT EXISTS index_websites_on_search ON websites USING gin (search);

SELECT manage_updated_at('websites');
SELECT manage_versions('websites');

CREATE TABLE IF NOT EXISTS navigation_items (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    website_id uuid NOT NULL,
    position smallint NOT NULL DEFAULT 0,
    title varchar NOT NULL,
    url varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_navigation_items PRIMARY KEY (id),
    CONSTRAINT fkey_navigation_items_to_websites FOREIGN KEY (website_id) REFERENCES websites (id)
);

SELECT manage_updated_at('navigation_items');
SELECT manage_versions('navigation_items');

CREATE TABLE IF NOT EXISTS posts (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    website_id uuid NOT NULL,
    user_id uuid NOT NULL,
    title citext NOT NULL,
    slug citext NOT NULL,
    content text NOT NULL DEFAULT '',
    hashtag_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [],
    variables jsonb NOT NULL DEFAULT '{}'::jsonb,
    cover_image_blob_id uuid NULL,
    blob_ids uuid [] NOT NULL DEFAULT ARRAY[]::uuid [],
    language regconfig NOT NULL DEFAULT 'english',
    search tsvector GENERATED ALWAYS AS (to_tsvector(language, title || ' ' || content)) STORED,
    published_at timestamptz NULL,
    modified_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_posts PRIMARY KEY (id),
    CONSTRAINT fkey_posts_to_websites FOREIGN KEY (website_id) REFERENCES websites (id),
    CONSTRAINT fkey_posts_to_users FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fkey_posts_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS index_posts_on_website_id_slug ON posts USING btree (website_id, slug);
CREATE INDEX IF NOT EXISTS index_posts_on_hashtag_ids ON posts USING gin (hashtag_ids);
CREATE INDEX IF NOT EXISTS index_posts_on_blob_ids ON posts USING gin (blob_ids);
CREATE INDEX IF NOT EXISTS index_posts_on_search ON posts USING gin (search);

SELECT manage_updated_at('posts');
SELECT manage_versions('posts');

CREATE TABLE IF NOT EXISTS post_comments (
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

CREATE INDEX IF NOT EXISTS index_post_comments_on_search ON post_comments USING gin (search);

SELECT manage_updated_at('post_comments');
SELECT manage_versions('post_comments');

CREATE TABLE IF NOT EXISTS post_reactions (
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

CREATE UNIQUE INDEX IF NOT EXISTS index_post_reactions_on_post_id_user_id ON post_reactions
USING btree (post_id, user_id);
CREATE INDEX IF NOT EXISTS index_post_reactions_on_emoji ON post_reactions USING btree (emoji);

SELECT manage_updated_at('post_reactions');
SELECT manage_versions('post_reactions');

CREATE TABLE IF NOT EXISTS post_views (
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

CREATE UNIQUE INDEX IF NOT EXISTS index_post_views_on_post_id_user_id ON post_views USING btree (post_id, user_id)
WHERE user_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS index_post_views_on_post_id_ip_address ON post_views USING btree (post_id, ip_address)
WHERE user_id IS NULL;

SELECT manage_updated_at('post_views');
SELECT manage_versions('post_views');

ALTER TABLE blobs DROP CONSTRAINT IF EXISTS fkey_blobs_to_websites, DROP CONSTRAINT IF EXISTS fkey_blobs_to_users;

ALTER TABLE blobs ADD CONSTRAINT fkey_blobs_to_websites FOREIGN KEY (website_id) REFERENCES websites (id),
ADD CONSTRAINT fkey_blobs_to_users FOREIGN KEY (user_id) REFERENCES users (id);
