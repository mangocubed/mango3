ALTER TABLE user_sessions ADD COLUMN confirmation_code_id uuid NULL, ADD COLUMN confirmed_at timestamptz NULL,
ADD CONSTRAINT fkey_user_sessions_to_confirmation_codes FOREIGN KEY (confirmation_code_id)
REFERENCES confirmation_codes (id) ON DELETE SET NULL;

CREATE UNIQUE INDEX index_user_sessions_on_confirmation_code_id ON user_sessions (confirmation_code_id);
