
-- Function to get all owner IDs that a user has access to
-- This relies on the fact that `team_members` is flattened by triggers
CREATE OR REPLACE FUNCTION get_accessible_owner_ids(user_uuid UUID) RETURNS SETOF UUID AS $$
BEGIN
    RETURN QUERY
    WITH my_identities AS (
        -- The user themselves
        SELECT user_uuid AS id
        UNION
        -- Teams the user is a member of (includes parent teams due to flattening triggers)
        SELECT team_id AS id FROM team_members WHERE member_id = user_uuid
    )
    -- 1. Owners the user IS (themselves + their teams)
    SELECT id FROM my_identities
    UNION
    -- 2. Owners the user (or their team) has been granted access to
    SELECT granted_owner_id 
    FROM owner_grants 
    WHERE grantee_owner_id IN (SELECT id FROM my_identities);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

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
    owner_id IN (
        SELECT get_accessible_owner_ids(auth_current_user_id())
    )
);