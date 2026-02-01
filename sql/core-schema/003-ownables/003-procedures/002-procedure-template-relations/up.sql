CREATE TABLE parent_procedure_templates (
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	-- The parent_id procedure_id template
	parent_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The child_id procedure_id template
	child_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE CHECK (parent_id <> child_id),
	-- There can only be a single relationship between a parent_id and a child_id
	UNIQUE (parent_id, child_id)
);
INSERT INTO table_names (id) VALUES ('parent_procedure_templates') ON CONFLICT DO NOTHING;

CREATE TABLE next_procedure_templates (
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	-- The parent_id procedure_id template
	parent_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The predecessor_id procedure_id template
	predecessor_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The successor_id procedure_id template
	successor_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- Composite unique constraint to prevent duplicate sequential relationships
	UNIQUE (parent_id, predecessor_id, successor_id),
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the predecessor_id procedure
	FOREIGN KEY (parent_id, predecessor_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the successor_id procedure
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that the parent is not the same as the predecessor.
	CHECK (parent_id <> predecessor_id),
	-- We enforce that the parent is not the same as the successor.
	CHECK (parent_id <> successor_id),
	-- We enforce that the predecessor and successor are not the same
	CHECK (predecessor_id <> successor_id)
);
INSERT INTO table_names (id) VALUES ('next_procedure_templates') ON CONFLICT DO NOTHING;

-- Trigger function to ensure parent-child relationships exist for sequential steps.
--
-- When a sequential relationship (next_procedure) is created within a parent procedure,
-- this function ensures that both the predecessor and successor are explicitly 
-- registered as children of the parent procedure in the `parent_procedure_templates` table.
--
-- This maintains consistency between the sequential ordering of steps and the 
-- hierarchical structure of the procedure.
CREATE OR REPLACE FUNCTION ensure_parent_procedure_templates() RETURNS TRIGGER LANGUAGE plpgsql AS $$ 
DECLARE
    v_new_id UUID;
    v_owner_id UUID;
    v_creator_id UUID;
    v_minimum_role_id SMALLINT;
BEGIN 
    -- Fetch context from the ownable/entity associated with the sequential step being created
    SELECT o.owner_id, o.creator_id, e.minimum_role_id
    INTO v_owner_id, v_creator_id, v_minimum_role_id
    FROM ownables o
    JOIN entities e ON o.id = e.id
    WHERE o.id = NEW.id;

    -- Insert predecessor parent relationship if it doesn't already exist.
    IF NOT EXISTS (
        SELECT 1 
        FROM parent_procedure_templates 
        WHERE parent_id = NEW.parent_id AND child_id = NEW.predecessor_id
    ) THEN
        v_new_id := uuidv7();

        -- Create Entity
        INSERT INTO entities (id, table_name_id, minimum_role_id) 
        VALUES (v_new_id, 'parent_procedure_templates', v_minimum_role_id);

        -- Create Ownable
        INSERT INTO ownables (id, owner_id, creator_id, editor_id)
        VALUES (v_new_id, v_owner_id, v_creator_id, v_creator_id);

        -- Create Relation
        INSERT INTO parent_procedure_templates (id, parent_id, child_id)
        VALUES (v_new_id, NEW.parent_id, NEW.predecessor_id);
    END IF;

    -- Insert successor parent relationship if it doesn't already exist.
    IF NOT EXISTS (
        SELECT 1 
        FROM parent_procedure_templates 
        WHERE parent_id = NEW.parent_id AND child_id = NEW.successor_id
    ) THEN
        v_new_id := uuidv7();

        -- Create Entity
        INSERT INTO entities (id, table_name_id, minimum_role_id) 
        VALUES (v_new_id, 'parent_procedure_templates', v_minimum_role_id);

        -- Create Ownable
        INSERT INTO ownables (id, owner_id, creator_id, editor_id)
        VALUES (v_new_id, v_owner_id, v_creator_id, v_creator_id);

        -- Create Relation
        INSERT INTO parent_procedure_templates (id, parent_id, child_id)
        VALUES (v_new_id, NEW.parent_id, NEW.successor_id);
    END IF;
    
    RETURN NEW;
END;
$$;
/*
Expected Translation:

WITH context AS (
    SELECT o.owner_id, o.creator_id, e.minimum_role_id
    FROM ownables o
    JOIN entities e ON o.id = e.id
    WHERE o.id = NEW.id
)
INSERT INTO entities (id, table_name_id, minimum_role_id)
SELECT uuidv7(), 'parent_procedure_templates', context.minimum_role_id
FROM context
WHERE NOT EXISTS (
    SELECT 1 
    FROM parent_procedure_templates 
    WHERE parent_id = NEW.parent_id AND child_id = NEW.predecessor_id
);

... (Followed by INSERTs for ownables and parent_procedure_templates using the generated ID)
*/

-- Trigger to execute `ensure_parent_procedure_templates` before insertion into `next_procedure_templates`.
-- It runs BEFORE INSERT so that the parent-child relationships are guaranteed to exist 
-- (or be created) before the sequential ordering is finalized.
CREATE OR REPLACE TRIGGER before_insert_next_procedure_templates 
BEFORE INSERT ON next_procedure_templates 
FOR EACH ROW EXECUTE FUNCTION ensure_parent_procedure_templates();

-- Trigger function to propagate asset requirements from child procedures to parent procedures.
--
-- When a procedure is added as a child to a parent (via `parent_procedure_templates`),
-- this function identifies all asset models required by the child procedure 
-- and adds them as requirements to the parent procedure.
--
-- The new entries in `procedure_template_asset_models` will be linked to the original
-- entries from the child via the `based_on_id` field, creating a traceability chain.
--
-- Note: This trigger assumes that the `procedure_template_asset_models` table exists.
-- If this migration runs before the table creation, it will fail.
CREATE OR REPLACE FUNCTION inherit_procedure_template_asset_models() RETURNS TRIGGER AS $$ 
BEGIN
    INSERT INTO procedure_template_asset_models (
            name,
            procedure_template_id,
            based_on_id,
            asset_model_id
        )
    SELECT 
        pam.name,
        NEW.parent_id,
        pam.id,
        pam.asset_model_id
    FROM procedure_template_asset_models pam
    WHERE pam.procedure_template_id = NEW.child_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to execute `inherit_procedure_template_asset_models` after insertion into `parent_procedure_templates`.
-- It runs AFTER INSERT to ensure the parent property is established before copying assets.
CREATE OR REPLACE TRIGGER trg_inherit_procedure_template_asset_models
AFTER INSERT ON parent_procedure_templates 
FOR EACH ROW EXECUTE FUNCTION inherit_procedure_template_asset_models();