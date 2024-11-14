CREATE TABLE confirmation_codes (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    encrypted_code varchar NOT NULL,
    failed_attempts smallint NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_confirmation_codes PRIMARY KEY (id)
);

SELECT manage_updated_at('confirmation_codes');
SELECT manage_versions('confirmation_codes');

ALTER TABLE users
ADD COLUMN email_confirmation_code_id uuid NULL,
ADD COLUMN email_confirmed_at timestamptz NULL,
ADD COLUMN password_reset_confirmation_code_id uuid NULL,
ADD CONSTRAINT fkey_users_to_email_confirmation_codes FOREIGN KEY (email_confirmation_code_id)
REFERENCES confirmation_codes (id),
ADD CONSTRAINT fkey_users_to_password_reset_confirmation_codes FOREIGN KEY (password_reset_confirmation_code_id)
REFERENCES confirmation_codes (id);
