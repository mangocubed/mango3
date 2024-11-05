CREATE TABLE users (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    username citext NOT NULL,
    email citext NOT NULL,
    encrypted_password varchar NOT NULL,
    display_name varchar NOT NULL,
    full_name varchar NOT NULL,
    birthdate date NOT NULL,
    language_code varchar NOT NULL DEFAULT 'en',
    country_alpha2 varchar NOT NULL,
    bio text NOT NULL DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_users PRIMARY KEY (id)
);

CREATE UNIQUE INDEX index_users_on_username ON users USING btree (username);
CREATE UNIQUE INDEX index_users_on_email ON users USING btree (email);

SELECT manage_updated_at('users');
SELECT manage_versions('users');
