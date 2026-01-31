CREATE TABLE procedure_templates (
	-- Identifier of the procedure_id template
	id UUID PRIMARY KEY DEFAULT uuidv7() REFERENCES ownables(id) ON DELETE CASCADE,
	-- Version of the procedure_id template.
	version INTEGER NOT NULL DEFAULT 1,
	-- Human-readable name of the procedure_id template
	name TEXT UNIQUE NOT NULL CHECK (name <> '' AND length(name) < 255),
	-- Human-readable description of the procedure_id template
	description TEXT NOT NULL CHECK (description <> '' AND length(description) < 8192),
	-- Whether this procedure_id template is deprecated and should not be used for new procedures
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	-- We enforce that the name and description are distinct to avoid lazy duplicates
	CHECK (name <> description)
);
