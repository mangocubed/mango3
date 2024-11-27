CREATE TABLE navigation_items (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    website_id uuid NOT NULL,
    position smallint NOT NULL DEFAULT 0,
    title varchar NOT NULL,
    url varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_navigation_items PRIMARY KEY (id),
    CONSTRAINT fkey_navigation_items_to_websites FOREIGN KEY (website_id) REFERENCES websites (id)
);

SELECT manage_updated_at('navigation_items');
SELECT manage_versions('navigation_items');
