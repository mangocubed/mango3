CREATE TYPE user_role AS ENUM ('user', 'creator', 'admin', 'superuser');

ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'user';
