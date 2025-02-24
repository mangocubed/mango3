ALTER TABLE users DROP COLUMN email_confirmation_code_id;

ALTER TABLE user_sessions DROP COLUMN confirmation_code_id, DROP COLUMN confirmed_at;

DROP TABLE user_password_resets;

TRUNCATE TABLE confirmation_codes;

CREATE TYPE confirmation_code_action AS ENUM ('email_confirmation', 'login_confirmation', 'password_reset');

ALTER TABLE confirmation_codes ADD COLUMN user_id uuid NOT NULL, ADD COLUMN action confirmation_code_action NOT NULL,
ADD CONSTRAINT fkey_confirmation_codes_to_users FOREIGN KEY (user_id) REFERENCES users (id);

CREATE UNIQUE INDEX index_confirmation_codes_on_user_id_action ON confirmation_codes USING btree (user_id, action);
