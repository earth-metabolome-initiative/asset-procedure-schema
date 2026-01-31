-- Table storing users, extending owners
CREATE TABLE users (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE
);

-- Table storing system-wide roles for users
-- Separated from 'users' table to prevent privilege escalation via self-editing RLS policies.
CREATE TABLE user_system_roles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    role_id SMALLINT NOT NULL DEFAULT 2 REFERENCES roles(id)
);

-- Insert in the table_names table the 'users' owner table
INSERT INTO table_names (id) VALUES ('users');

-- Function to get the current user ID from session settings
CREATE OR REPLACE FUNCTION auth_current_user_id() RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_user_id', true)::UUID;
END;
$$ LANGUAGE plpgsql STABLE;