-- Table storing users, extending owners
CREATE TABLE users (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE
);
-- Insert in the table_names table the 'users' owner table
INSERT INTO table_names (id) VALUES ('users');

-- Function to get the current user ID from session settings
CREATE OR REPLACE FUNCTION current_app_user() RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_user_id', true)::UUID;
END;
$$ LANGUAGE plpgsql STABLE;