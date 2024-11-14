ALTER TABLE users DROP COLUMN email_confirmation_code_id, DROP COLUMN email_confirmed_at,
DROP COLUMN password_reset_confirmation_code_id;

DROP TABLE confirmation_codes;
