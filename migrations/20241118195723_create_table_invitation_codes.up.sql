CREATE TABLE invitation_codes (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    email citext NOT NULL,
    code citext NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_invitation_codes PRIMARY KEY (id)
);

CREATE UNIQUE INDEX index_invitation_codes_on_email ON invitation_codes USING btree (email);
CREATE UNIQUE INDEX index_invitation_codes_on_code ON invitation_codes USING btree (code);

SELECT manage_updated_at('invitation_codes');
SELECT manage_versions('invitation_codes');
