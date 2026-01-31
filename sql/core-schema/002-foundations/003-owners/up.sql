-- Table storing owners (base entity for users, teams, projects)
CREATE TABLE owners (
	-- Surrogate primary key for the owner entity
	id UUID PRIMARY KEY DEFAULT uuidv7(),
	-- The type of owner (e.g., 'user', 'team', etc.)
	table_name_id TEXT NOT NULL REFERENCES table_names(id) CHECK (
		table_name_id <> ''
		AND length(table_name_id) < 255
	),
	-- Time of creation
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- Time of last update
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The creation time is expected to be before or equal to the update time
	CHECK (created_at <= edited_at)
);
-- Trigger to update edited_at
CREATE OR REPLACE FUNCTION update_owners_edited_at() RETURNS TRIGGER AS $$ BEGIN NEW.edited_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_update_owners_edited_at BEFORE
UPDATE ON owners FOR EACH ROW EXECUTE FUNCTION update_owners_edited_at();
-- Grants 
CREATE TABLE owner_grants (
	grantee_owner_id UUID NOT NULL REFERENCES owners(id),
	granted_owner_id UUID NOT NULL REFERENCES owners(id),
	role_id SMALLINT NOT NULL REFERENCES roles(id),
	PRIMARY KEY (grantee_owner_id, granted_owner_id),
	CHECK (grantee_owner_id <> granted_owner_id)
);