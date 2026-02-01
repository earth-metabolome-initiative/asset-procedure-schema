-- Table storing ownables (base entity for ownable assets, procedures, etc.)
CREATE TABLE ownables (
	-- Surrogate primary key for the ownable entity
	id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE,
	-- Owner of the ownable entity
	owner_id UUID NOT NULL REFERENCES owners(id) ON DELETE CASCADE,
	-- Creator of the ownable entity
	creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	-- Editor of the ownable entity
	editor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
INSERT INTO table_names (id)
VALUES ('ownables') ON CONFLICT DO NOTHING;