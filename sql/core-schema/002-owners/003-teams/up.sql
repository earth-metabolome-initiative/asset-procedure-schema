-- Table storing teams, extending owners
CREATE TABLE teams (
	-- Primary key references owners(id)
	id UUID PRIMARY KEY REFERENCES owners(id) ON DELETE CASCADE,
	-- The owner of this team
	owner_id UUID NOT NULL REFERENCES owners(id) ON DELETE CASCADE,
	-- The parent team of this team (nullable for top-level teams)
	parent_team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
	-- Team name
	name TEXT NOT NULL CHECK (
		name <> ''
		AND length(name) <= 255
	),
	-- Description of the team
	description TEXT NOT NULL CHECK (
		description <> ''
		AND length(description) < 8192
	),
	-- We check that name is not equal to description
	CHECK (name <> description)
);
-- Insert in the owner_tables table the 'teams' owner table
INSERT INTO owner_tables (table_name)
VALUES ('teams');
-- Table storing team members
CREATE TABLE team_members (
	-- The team to which the member belongs
	team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
	-- The member who is part of the team
	member_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	-- Composite primary key to ensure uniqueness of team-member pairs
	PRIMARY KEY (team_id, member_id)
);
-- Index to optimize queries filtering by member_id
CREATE INDEX idx_team_members_member_id ON team_members(member_id);
-- When a team member is added, it is also added as a member of the parent team (if any)
-- We write the trigger function as a query without IF statements for compatibility with SQLite
CREATE OR REPLACE FUNCTION add_team_member_to_parent_team() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO team_members (team_id, member_id)
SELECT parent_team_id,
	NEW.member_id
FROM teams
WHERE NEW.team_id = teams.id
	AND parent_team_id IS NOT NULL ON CONFLICT DO NOTHING;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_add_team_member_to_parent_team
AFTER
INSERT ON team_members FOR EACH ROW EXECUTE FUNCTION add_team_member_to_parent_team();
-- When a team member is removed, it is also removed from all child teams
CREATE OR REPLACE FUNCTION remove_team_member_from_child_teams() RETURNS TRIGGER AS $$ BEGIN
DELETE FROM team_members USING teams
WHERE teams.parent_team_id = OLD.team_id
	AND team_members.team_id = teams.id
	AND team_members.member_id = OLD.member_id;
RETURN OLD;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_remove_team_member_from_child_teams
AFTER DELETE ON team_members FOR EACH ROW EXECUTE FUNCTION remove_team_member_from_child_teams();