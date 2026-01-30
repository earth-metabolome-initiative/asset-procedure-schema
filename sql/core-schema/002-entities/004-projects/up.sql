-- Table storing projects, extending owners
CREATE TABLE projects (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE,
	-- Project name
	name TEXT NOT NULL CHECK (name <> ''),
	-- Description of the project
	description TEXT NOT NULL CHECK (description <> ''),
	-- Start date of the project
	start_date DATE DEFAULT CURRENT_DATE,
	-- End date of the project
	end_date DATE,
	-- End date must be after start date
	CHECK (
		end_date IS NULL
		OR start_date <= end_date
	)
);