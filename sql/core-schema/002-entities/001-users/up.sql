-- Table storing users of the system
CREATE TABLE users (
	-- Surrogate primary key for the user entity
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	-- Time of account creation
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- Time of last account update
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The creation time is expected to be before or equal to the update time
	CHECK (created_at <= edited_at)
);