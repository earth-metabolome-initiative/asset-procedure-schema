-- Table storing users, extending owners
CREATE TABLE users (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE
);
-- Insert in the owner_tables table the 'users' owner table
INSERT INTO owner_tables (table_name) VALUES ('users');