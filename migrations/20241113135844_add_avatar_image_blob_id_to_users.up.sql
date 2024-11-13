ALTER TABLE users ADD COLUMN avatar_image_blob_id uuid NULL,
ADD CONSTRAINT fkey_users_to_avatar_image_blobs FOREIGN KEY (avatar_image_blob_id) REFERENCES blobs (id);
