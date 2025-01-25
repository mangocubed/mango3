ALTER TABLE blobs ADD COLUMN website_id uuid NULL;

UPDATE blobs AS b SET website_id = (
    SELECT p.website_id FROM posts AS p
    WHERE p.user_id = b.user_id AND b.id = ANY(p.blob_ids) LIMIT 1
);

DROP INDEX IF EXISTS index_blobs_on_user_id_content_type_byte_size_md5_checksum;

CREATE UNIQUE INDEX index_blobs_on_user_id_website_id_content_type_byte_size_md5_checksum ON blobs
USING btree (user_id, website_id, content_type, byte_size, md5_checksum);
