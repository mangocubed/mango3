ALTER TABLE websites ADD COLUMN language regconfig NOT NULL DEFAULT 'english',
ADD COLUMN search tsvector GENERATED ALWAYS AS (to_tsvector(language, name || ' ' || description)) STORED;

CREATE INDEX index_websites_on_search ON websites USING btree (search);
