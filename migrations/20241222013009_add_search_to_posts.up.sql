ALTER TABLE posts ADD COLUMN language regconfig NOT NULL DEFAULT 'english',
ADD COLUMN search tsvector GENERATED ALWAYS AS (to_tsvector(language, title || ' ' || content)) STORED;

CREATE INDEX index_posts_on_search ON posts USING btree (search);
