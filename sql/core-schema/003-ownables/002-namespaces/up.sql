CREATE TABLE namespaces (
	-- Surrogate primary key for the namespace entity
	id UUID PRIMARY KEY DEFAULT uuidv7() REFERENCES ownables(id) ON DELETE CASCADE,
	-- Name of the namespace
	name TEXT UNIQUE NOT NULL CHECK (
		name <> ''
		AND length(name) <= 255
	),
	-- Description of the namespace
	description TEXT NOT NULL CHECK (
		description <> ''
		AND length(description) < 8192
	),
	-- We check that name is not equal to description
	CHECK (name <> description)
);