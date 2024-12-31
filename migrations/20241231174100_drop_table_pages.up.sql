DROP INDEX IF EXISTS index_posts_on_search;

CREATE INDEX index_posts_on_search ON posts USING gin (search);

DROP INDEX IF EXISTS index_websites_on_search;

CREATE INDEX index_websites_on_search ON websites USING gin (search);

INSERT INTO posts (
    id, website_id, user_id, title, slug, content, cover_image_blob_id, published_at, created_at, updated_at
) SELECT
    id,
    website_id,
    user_id,
    title,
    'page-' || slug AS slug,
    content,
    cover_image_blob_id,
    published_at,
    created_at,
    updated_at
FROM pages;

DROP TABLE pages;
