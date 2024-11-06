CREATE TABLE user_sessions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_user_sessions PRIMARY KEY (id),
    CONSTRAINT fkey_user_sessions_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

SELECT manage_updated_at('user_sessions');
SELECT manage_versions('user_sessions');
