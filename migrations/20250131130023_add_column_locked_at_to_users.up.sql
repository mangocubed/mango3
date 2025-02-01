ALTER TABLE users ADD COLUMN locked_at timestamptz;

ALTER TABLE blobs DROP CONSTRAINT fkey_blobs_to_websites, DROP CONSTRAINT fkey_blobs_to_users;

ALTER TABLE blobs ADD CONSTRAINT fkey_blobs_to_websites FOREIGN KEY (website_id) REFERENCES websites (id)
ON DELETE SET NULL,
ADD CONSTRAINT fkey_blobs_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL;

ALTER TABLE users DROP CONSTRAINT fkey_users_to_email_confirmation_codes,
DROP CONSTRAINT fkey_users_to_password_reset_confirmation_codes, DROP CONSTRAINT fkey_users_to_avatar_image_blobs;

ALTER TABLE users ADD CONSTRAINT fkey_users_to_email_confirmation_codes
FOREIGN KEY (email_confirmation_code_id) REFERENCES confirmation_codes (id) ON DELETE SET NULL,
ADD CONSTRAINT fkey_users_to_password_reset_confirmation_codes FOREIGN KEY (password_reset_confirmation_code_id)
REFERENCES confirmation_codes (id) ON DELETE SET NULL,
ADD CONSTRAINT fkey_users_to_avatar_image_blobs FOREIGN KEY (avatar_image_blob_id) REFERENCES blobs (id)
ON DELETE SET NULL;

ALTER TABLE user_sessions DROP CONSTRAINT fkey_user_sessions_to_users;

ALTER TABLE user_sessions ADD CONSTRAINT fkey_user_sessions_to_users FOREIGN KEY (user_id) REFERENCES users (id)
ON DELETE CASCADE;

ALTER TABLE websites DROP CONSTRAINT fkey_websites_to_users, DROP CONSTRAINT fkey_websites_to_icon_image_blobs,
DROP CONSTRAINT fkey_websites_to_cover_image_blobs;

ALTER TABLE websites ADD CONSTRAINT fkey_websites_to_users FOREIGN KEY (user_id) REFERENCES users (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_websites_to_icon_image_blobs FOREIGN KEY (icon_image_blob_id) REFERENCES blobs (id)
ON DELETE SET NULL,
ADD CONSTRAINT fkey_websites_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
ON DELETE SET NULL;

ALTER TABLE navigation_items DROP CONSTRAINT fkey_navigation_items_to_websites;

ALTER TABLE navigation_items ADD CONSTRAINT fkey_navigation_items_to_websites FOREIGN KEY (website_id)
REFERENCES websites (id) ON DELETE CASCADE;

ALTER TABLE posts DROP CONSTRAINT fkey_posts_to_websites, DROP CONSTRAINT fkey_posts_to_users,
DROP CONSTRAINT fkey_posts_to_cover_image_blobs;

ALTER TABLE posts ADD CONSTRAINT fkey_posts_to_websites FOREIGN KEY (website_id) REFERENCES websites (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_posts_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
ADD CONSTRAINT fkey_posts_to_cover_image_blobs FOREIGN KEY (cover_image_blob_id) REFERENCES blobs (id)
ON DELETE SET NULL;

ALTER TABLE post_comments DROP CONSTRAINT fkey_post_comments_to_posts, DROP CONSTRAINT fkey_post_comments_to_users;

ALTER TABLE post_comments ADD CONSTRAINT fkey_post_comments_to_posts FOREIGN KEY (post_id) REFERENCES posts (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_post_comments_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE post_reactions DROP CONSTRAINT fkey_post_reactions_to_posts, DROP CONSTRAINT fkey_post_reactions_to_users;

ALTER TABLE post_reactions ADD CONSTRAINT fkey_post_reactions_to_posts FOREIGN KEY (post_id) REFERENCES posts (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_post_reactions_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE post_views DROP CONSTRAINT fkey_post_views_to_posts, DROP CONSTRAINT fkey_post_views_to_users;

ALTER TABLE post_views ADD CONSTRAINT fkey_post_views_to_posts FOREIGN KEY (post_id) REFERENCES posts (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_post_views_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;
