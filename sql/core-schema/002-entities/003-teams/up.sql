-- Table storing teams, extending owners
CREATE TABLE teams (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE,
	-- Team name
	name TEXT NOT NULL CHECK (name <> ''),
	-- Description of the team
	description TEXT NOT NULL CHECK (description <> ''),
	-- We check that name is not equal to description
	CHECK (name <> description)
);