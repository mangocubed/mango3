CREATE EXTENSION IF NOT EXISTS citext;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS void AS $$
BEGIN
    EXECUTE format(
        'CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl
    );
END;
$$ LANGUAGE plpgsql;

CREATE TABLE versions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    action varchar NOT NULL,
    record_type varchar NOT NULL,
    record_id uuid NOT NULL,
    data jsonb,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_versions PRIMARY KEY (id)
);

SELECT manage_updated_at('versions');

CREATE OR REPLACE FUNCTION insert_into_versions() RETURNS trigger AS $$
DECLARE
    record_id uuid;
    data jsonb;
BEGIN
    IF (NEW IS DISTINCT FROM OLD) THEN
        IF (TG_OP IS DISTINCT FROM 'DELETE') THEN
            SELECT NEW.id INTO record_id;
            SELECT to_jsonb(NEW) INTO data;
        ELSE
            SELECT OLD.id INTO record_id;
        END IF;

        INSERT INTO versions (action, record_type, record_id, data)
            VALUES (LOWER(TG_OP), TG_TABLE_NAME, record_id, data);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION manage_versions(_tbl regclass) RETURNS void AS $$
BEGIN
    EXECUTE format(
        'CREATE OR REPLACE TRIGGER insert_into_versions AFTER INSERT OR UPDATE OR DELETE ON %s FOR EACH ROW
            EXECUTE FUNCTION insert_into_versions()',
        _tbl
   );
END;
$$ LANGUAGE plpgsql;
