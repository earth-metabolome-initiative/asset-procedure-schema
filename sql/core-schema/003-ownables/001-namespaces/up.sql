CREATE TABLE namespaces (
	-- Surrogate primary key for the namespace entity
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	-- Name of the namespace
	name TEXT UNIQUE NOT NULL CHECK (
		name <> ''
		AND length(name) <= 255
	),
	-- Description of the namespace
	description TEXT CHECK (
		description <> ''
		AND length(description) < 8192
	),
	-- We check that name is not equal to description
	CHECK (name <> description)
);

INSERT INTO table_names (id)
VALUES ('namespaces') ON CONFLICT DO NOTHING;

CREATE TABLE namespaced_ownables (
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	namespace_id UUID NOT NULL REFERENCES namespaces(id) ON DELETE CASCADE,
	name TEXT NOT NULL CHECK (
		name <> ''
		AND length(name) <= 255
	),
	description TEXT NOT NULL CHECK (description <> '' AND length(description) < 8192),
	UNIQUE (namespace_id, name),
	CHECK (name <> description)
);
INSERT INTO table_names (id)
VALUES ('namespaced_ownables') ON CONFLICT DO NOTHING;