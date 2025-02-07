ALTER TABLE users DROP COLUMN password_reset_confirmation_code_id;

CREATE TABLE user_password_resets (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    confirmation_code_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_user_password_resets PRIMARY KEY (id),
    CONSTRAINT fkey_user_password_resets_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fkey_user_password_resets_to_confirmation_codes FOREIGN KEY (confirmation_code_id)
    REFERENCES confirmation_codes (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_user_password_resets_on_user_id ON user_password_resets USING btree (user_id);

SELECT manage_updated_at('user_password_resets');
SELECT manage_versions('user_password_resets');
