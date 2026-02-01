-- Table storing owners (base entity for users, teams, projects)
CREATE TABLE owners (
	-- Surrogate primary key for the owner entity
	id UUID PRIMARY KEY REFERENCES entities(id) ON DELETE CASCADE
);
INSERT INTO table_names (id)
VALUES ('owners') ON CONFLICT DO NOTHING;

-- Grants 
CREATE TABLE owner_grants (
	grantee_owner_id UUID NOT NULL REFERENCES owners(id),
	granted_owner_id UUID NOT NULL REFERENCES owners(id),
	role_id SMALLINT NOT NULL REFERENCES roles(id),
	PRIMARY KEY (grantee_owner_id, granted_owner_id),
	CHECK (grantee_owner_id <> granted_owner_id)
);