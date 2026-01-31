
-- Function to check if a user can access a specific owner's data
-- This effectively replaces get_accessible_owner_ids with a boolean check
-- optimized for RLS and supporting system-wide roles.
CREATE OR REPLACE FUNCTION can_access_owner(user_uuid UUID, target_owner_id UUID)
RETURNS BOOLEAN
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    u_role SMALLINT;
BEGIN
    -- 1. Check System Admin
    -- User with role_id >= 4 (Admin) in user_system_roles has full access
    SELECT role_id INTO u_role FROM user_system_roles WHERE user_id = user_uuid;
    
    IF COALESCE(u_role, 0) >= 4 THEN
        RETURN TRUE;
    END IF;

    -- 2. Direct Identity Check (Am I the owner?)
    IF user_uuid = target_owner_id THEN
        RETURN TRUE;
    END IF;

    -- 3. Team Membership & Grants
    -- Check if user is a member of the target owner (if it's a team)
    -- OR if access is granted via owner_grants
    RETURN EXISTS (
        WITH my_identities AS (
            SELECT user_uuid AS id
            UNION
            SELECT team_id AS id
            FROM team_members
            WHERE member_id = user_uuid
        )
        SELECT 1
        FROM my_identities
        WHERE id = target_owner_id -- I am the owner (member of the owning team)
        UNION
        SELECT 1
        FROM owner_grants
        WHERE granted_owner_id = target_owner_id
          AND grantee_owner_id IN (SELECT id FROM my_identities)
    );
END;
$$;

-- Enable RLS on Ownables
ALTER TABLE ownables ENABLE ROW LEVEL SECURITY;

-- Policy: Users can see/edit ownables if they are the owner OR have been granted access to the owner
-- Note: This controls SELECT, INSERT, UPDATE, DELETE.
-- Fine-grained control (Viewer vs Editor) requires checking the 'role' in owner_grants,
-- which matches the specific operation.
-- For now, this is a coarse "Access" policy.
CREATE POLICY ownables_access_policy ON ownables
FOR ALL
TO PUBLIC
USING (
    can_access_owner(auth_current_user_id(), owner_id)
);