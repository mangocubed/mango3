DROP TABLE user_password_resets;

ALTER TABLE users ADD COLUMN password_reset_confirmation_code_id uuid NULL;
