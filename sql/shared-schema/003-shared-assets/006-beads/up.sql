CREATE TABLE bead_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Diameter in millimeters
	diameter REAL NOT NULL CHECK (diameter > 0.0)
);
INSERT INTO table_names (id) VALUES ('bead_models') ON CONFLICT DO NOTHING;
