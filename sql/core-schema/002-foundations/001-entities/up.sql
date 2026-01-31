-- Table storing all table names for type identification
CREATE TABLE table_names (
    id TEXT NOT NULL PRIMARY KEY CHECK (
        id <> ''
        AND length(id) < 255
    )
);
-- Roles
CREATE TABLE roles (
    id SMALLINT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE CHECK (
        name <> ''
        AND length(name) < 255
    )
);
INSERT INTO roles (id, name)
VALUES (1, 'anonymous'),
    (2, 'viewer'),
    (3, 'editor'),
    (4, 'admin');
-- Base Table for all entities (Owners, Ownables, etc.)
CREATE TABLE entities (
    -- Universal UUIDv7 ID
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    -- The specific table this entity belongs to (e.g., 'users', 'assets')
    table_name_id TEXT NOT NULL REFERENCES table_names(id) CHECK (
        table_name_id <> ''
        AND length(table_name_id) < 255
    ),
    -- Privacy Level (1=Public, 2=Viewer, 3=Editor, 4=Admin)
    -- Defines the minimum role required to view the entity.
    minimum_role_id SMALLINT NOT NULL DEFAULT 1 CHECK (
        minimum_role_id BETWEEN 1 AND 4
    ) REFERENCES roles(id),
    -- Time of creation, using UTC timezone
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Time of last edit, using UTC timezone
    edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Consistency check
    CHECK (created_at <= edited_at)
);
-- Trigger to maintain updated_at
CREATE OR REPLACE FUNCTION update_entities_edited_at() RETURNS TRIGGER AS $$ BEGIN NEW.edited_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_update_entities_edited_at BEFORE
UPDATE ON entities FOR EACH ROW EXECUTE FUNCTION update_entities_edited_at();
-- Privacy Dependencies Table (replaces entity_hierarchy)
-- Used to enforce privacy inheritance: Child.minimum_role_id >= Parent.minimum_role_id
CREATE TABLE privacy_dependencies (
    child_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    parent_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    PRIMARY KEY (child_id, parent_id),
    CHECK (child_id <> parent_id)
);
CREATE INDEX idx_privacy_dependencies_child ON privacy_dependencies(child_id);
CREATE INDEX idx_privacy_dependencies_parent ON privacy_dependencies(parent_id);
-- Trigger 1: On privacy_dependencies (Relation creation)
-- Ensure new links don't violate privacy
CREATE OR REPLACE FUNCTION check_dependency_compliance() RETURNS TRIGGER AS $$
DECLARE c_role SMALLINT;
p_role SMALLINT;
BEGIN
SELECT COALESCE(minimum_role_id, 0) INTO c_role
FROM entities
WHERE id = NEW.child_id;
SELECT COALESCE(minimum_role_id, 0) INTO p_role
FROM entities
WHERE id = NEW.parent_id;
IF c_role < p_role THEN RAISE EXCEPTION 'Privacy Violation: Child cannot be less restricted than Parent.';
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_check_dependency_compliance BEFORE
INSERT
    OR
UPDATE ON privacy_dependencies FOR EACH ROW EXECUTE FUNCTION check_dependency_compliance();
-- Trigger 2: On entities (Role update)
-- Ensure privacy updates don't violate existing hierarchy
CREATE OR REPLACE FUNCTION check_entity_privacy_on_update() RETURNS TRIGGER AS $$ BEGIN -- Check Parents (I cannot become more public than any parent)
    -- Violation if: Any Parent > New Self
    IF EXISTS (
        SELECT 1
        FROM privacy_dependencies pd
            JOIN entities p ON pd.parent_id = p.id
        WHERE pd.child_id = NEW.id
            AND COALESCE(p.minimum_role_id, 0) > COALESCE(NEW.minimum_role_id, 0)
    ) THEN RAISE EXCEPTION 'Privacy Violation: Entity cannot be less restricted than its parents.';
END IF;
-- Check Children (I cannot become more private than any child)
-- Violation if: Any Child < New Self
IF EXISTS (
    SELECT 1
    FROM privacy_dependencies pd
        JOIN entities c ON pd.child_id = c.id
    WHERE pd.parent_id = NEW.id
        AND COALESCE(c.minimum_role_id, 0) < COALESCE(NEW.minimum_role_id, 0)
) THEN RAISE EXCEPTION 'Privacy Violation: Entity cannot be more restricted than its children.';
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_check_entity_privacy_on_update BEFORE
UPDATE OF minimum_role_id ON entities FOR EACH ROW EXECUTE FUNCTION check_entity_privacy_on_update();