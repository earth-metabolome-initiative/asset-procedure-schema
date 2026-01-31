
-- Function to get the user's maximum role on a specific owner
-- Returns: 0=None, 1=Anonymous (unused), 2=Viewer, 3=Editor, 4=Admin
CREATE OR REPLACE FUNCTION get_owner_role(user_uuid UUID, target_owner_id UUID)
RETURNS SMALLINT
LANGUAGE plpgsql
SECURITY DEFINER
STABLE
AS $$
DECLARE
    max_role SMALLINT := 0;
BEGIN
    -- 1. Direct Identity (Am I the owner?)
    IF user_uuid = target_owner_id THEN
        RETURN 4; -- Admin
    END IF;

    -- 2. Team Membership & Grants
    -- Calculate max role based on team membership (implicit Admin) and explicit grants
    SELECT COALESCE(MAX(role_val), 0) INTO max_role
    FROM (
        -- A. Permissions from being a member of the target owner (if it is a team)
        -- If I am a member of Team X, and Team X is the Target, I have Admin rights (4)
        SELECT 4 as role_val
        FROM team_members
        WHERE team_id = target_owner_id AND member_id = user_uuid
        
        UNION ALL
        
        -- B. Permissions from Grants
        -- Check grants given to Me or My Teams for the target_owner_id
        SELECT role_id as role_val
        FROM owner_grants
        WHERE granted_owner_id = target_owner_id
        AND grantee_owner_id IN (
            SELECT user_uuid -- Me
            UNION ALL
            SELECT team_id FROM team_members WHERE member_id = user_uuid -- My Teams
        )
    ) as sub;

    RETURN max_role;
END;
$$;

-- Enable RLS on Ownables
ALTER TABLE ownables ENABLE ROW LEVEL SECURITY;

-- -----------------------------------------------------------------------------
-- Policies (Permissions based on Role)
-- -----------------------------------------------------------------------------

-- SELECT: Requires Viewer (2) or higher
CREATE POLICY ownables_select_policy ON ownables
FOR SELECT
TO PUBLIC
USING (
    get_owner_role(auth_current_user_id(), owner_id) >= 2
);

-- INSERT: Requires Editor (3) or higher on the target owner
CREATE POLICY ownables_insert_policy ON ownables
FOR INSERT
TO PUBLIC
WITH CHECK (
    get_owner_role(auth_current_user_id(), owner_id) >= 3
);

-- UPDATE: Requires Editor (3) or higher
CREATE POLICY ownables_update_policy ON ownables
FOR UPDATE
TO PUBLIC
USING (
    get_owner_role(auth_current_user_id(), owner_id) >= 3
)
WITH CHECK (
    get_owner_role(auth_current_user_id(), owner_id) >= 3
);

-- DELETE: Requires Admin (4) (or Owner/Team Member)
CREATE POLICY ownables_delete_policy ON ownables
FOR DELETE
TO PUBLIC
USING (
    get_owner_role(auth_current_user_id(), owner_id) >= 4
);

-- -----------------------------------------------------------------------------
-- Admin Role & Policies (Postgres Only)
-- -----------------------------------------------------------------------------

-- Safely create the 'app_admin' role if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'app_admin') THEN
        CREATE ROLE app_admin WITH LOGIN;
    END IF;
END
$$;

-- Grant broad permissions to the admin role
-- Note: 'ALTER DEFAULT PRIVILEGES' might be needed for future tables in a production setup
GRANT ALL ON ALL TABLES IN SCHEMA public TO app_admin;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO app_admin;

-- Admin Policy: Allow full unrestricted access to ownables
CREATE POLICY admin_all_access ON ownables
    FOR ALL
    TO app_admin
    USING (TRUE)
    WITH CHECK (TRUE);