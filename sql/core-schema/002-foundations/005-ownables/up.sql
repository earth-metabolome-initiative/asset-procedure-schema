-- Table storing ownable tables
CREATE TABLE ownable_tables (
	-- Name of an ownable table
	id TEXT NOT NULL PRIMARY KEY CHECK (
		id <> ''
		AND length(id) < 255
	)
);
-- Table storing ownables (base entity for ownable assets, procedures, etc.)
CREATE TABLE ownables (
	-- Surrogate primary key for the ownable entity
	id UUID PRIMARY KEY DEFAULT uuidv7(),
	-- The type of ownable (e.g., 'asset', 'procedure', etc.)
	ownable_table_id TEXT NOT NULL REFERENCES ownable_tables(id) CHECK (
		ownable_table_id <> ''
		AND length(ownable_table_id) < 255
	),
	-- Owner of the ownable entity
	owner_id UUID NOT NULL REFERENCES owners(id) ON DELETE CASCADE,
	-- Creator of the ownable entity
	creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	-- Editor of the ownable entity
	editor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	-- Time of creation
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- Time of last update
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The creation time is expected to be before or equal to the update time
	CHECK (created_at <= edited_at)
);
-- Trigger to update edited_at
CREATE OR REPLACE FUNCTION update_ownables_edited_at() RETURNS TRIGGER AS $$ BEGIN NEW.edited_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- Trigger to update edited_at before updating ownables
CREATE TRIGGER trigger_update_ownables_edited_at BEFORE
UPDATE ON ownables FOR EACH ROW EXECUTE FUNCTION update_ownables_edited_at();