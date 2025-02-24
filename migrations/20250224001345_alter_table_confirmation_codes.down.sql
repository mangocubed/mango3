ALTER TABLE confirmation_codes DROP COLUMN user_id, DROP COLUMN action;

DROP TYPE confirmation_code_action;

ALTER TABLE users ADD COLUMN email_confirmation_code_id uuid NULL;

ALTER TABLE user_sessions ADD COLUMN confirmation_code_id uuid NULL, ADD COLUMN confirmed_at timestamptz NULL;

CREATE TABLE user_password_resets ();
