CREATE TABLE photographs (
	id UUID PRIMARY KEY REFERENCES digital_assets(id) ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('photographs') ON CONFLICT DO NOTHING;