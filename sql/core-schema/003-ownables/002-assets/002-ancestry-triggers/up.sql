-- Function to maintain asset model ancestry on insert
CREATE OR REPLACE FUNCTION insert_asset_model_ancestry() RETURNS TRIGGER AS $$ 
BEGIN
    -- 1. Insert self-reference
    INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id)
    VALUES (NEW.id, NEW.id);

    -- 2. If there is a parent, inherit its ancestors
    IF NEW.parent_model_id IS NOT NULL THEN
        INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id)
        SELECT NEW.id, ancestor_model_id
        FROM asset_model_ancestors
        WHERE descendant_model_id = NEW.parent_model_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_insert_asset_model_ancestry
AFTER INSERT ON asset_models
FOR EACH ROW
EXECUTE FUNCTION insert_asset_model_ancestry();

-- -- Function to maintain asset model ancestry on update (parent change)
-- CREATE OR REPLACE FUNCTION update_asset_model_ancestry() RETURNS TRIGGER AS $$
-- DECLARE
--     curr_descendant UUID;
-- BEGIN
--     -- Only proceed if parent_model_id has changed
--     IF OLD.parent_model_id IS DISTINCT FROM NEW.parent_model_id THEN
        
--         -- 1. Eliminate all ancestry relationships for the subtree rooted at NEW.id
--         --    that originate from the OLD parents chain.
--         --    Actually, for safety and simplicity in this recursive structure, 
--         --    we can delete all non-self ancestors for the entire subtree and rebuild them.
        
--         -- Find all descendants of NEW.id (including NEW.id itself)
--         -- We can use the existing table before we modify it.
--         FOR curr_descendant IN 
--             SELECT descendant_model_id 
--             FROM asset_model_ancestors 
--             WHERE ancestor_model_id = NEW.id
--         LOOP
--             -- Delete old ancestors (ancestors of the moving node's parent)
--             -- We keep the link (curr_descendant, curr_descendant) and links that are within the subtree
--             -- The ones to remove are those where the ancestor is NOT in the subtree of NEW.id
--             DELETE FROM asset_model_ancestors
--             WHERE descendant_model_id = curr_descendant
--               AND ancestor_model_id NOT IN (
--                   SELECT descendant_model_id 
--                   FROM asset_model_ancestors 
--                   WHERE ancestor_model_id = NEW.id
--               );
              
--             -- Insert new ancestors from the NEW parent
--             IF NEW.parent_model_id IS NOT NULL THEN
--                 INSERT INTO asset_model_ancestors (descendant_model_id, ancestor_model_id)
--                 SELECT curr_descendant, ancestor_model_id
--                 FROM asset_model_ancestors
--                 WHERE descendant_model_id = NEW.parent_model_id;
--             END IF;
--         END LOOP;

--     END IF;
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER trigger_update_asset_model_ancestry
-- AFTER UPDATE ON asset_models
-- FOR EACH ROW
-- EXECUTE FUNCTION update_asset_model_ancestry();
