-- Table storing users, extending owners
CREATE TABLE users (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE
);