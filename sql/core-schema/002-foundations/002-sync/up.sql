-- Tombstone Table, to log deletions of entities and facilitate eventual consistency
-- synchronization in distributed systems.
CREATE TABLE tombstones (
    -- A deleted entity's ID, which cannot be a foreign key due to deletion.
    id UUID NOT NULL,
    -- The name of the table from which the entity was deleted.
    table_name TEXT NOT NULL REFERENCES table_names(id) NOT NULL CHECK (
        table_name <> ''
        AND length(table_name) < 255
    ),
    -- The timestamp when the entity was deleted.
    deleted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Composite primary key to prevent duplicate tombstones for the same entity.
    PRIMARY KEY (id, table_name)
);
-- Universal Deletion Trigger
-- Because 'entities' is the root table, we ONLY need this trigger on 'entities'.
CREATE OR REPLACE FUNCTION log_entity_deletion() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO tombstones (id, table_name)
VALUES (OLD.id, OLD.table_name_id);
RETURN OLD;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_log_deletion_entities
AFTER DELETE ON entities FOR EACH ROW EXECUTE FUNCTION log_entity_deletion();