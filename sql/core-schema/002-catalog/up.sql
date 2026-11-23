-- Functions the APS owner role creates start without `EXECUTE` for PUBLIC,
-- as those the migration role creates do under connetto's default revoke.
ALTER DEFAULT PRIVILEGES FOR ROLE aps_writer REVOKE EXECUTE ON FUNCTIONS FROM PUBLIC;

-- Registry of APS tables, named by the `table_name_id` column of every root row
-- so a client can resolve the most concrete table holding an ownable.
CREATE TABLE table_names (
    -- Name of the APS table.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);

-- Reflexive transitive closure of the APS table tree, one row for a table and
-- every table it extends, so a row of a subtype table can be checked against
-- the `table_name_id` of its root row. Each slice seeds the rows of the
-- tables it creates.
CREATE TABLE table_name_ancestors (
    -- Table whose ancestors the row lists.
    descendant_table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Table the descendant extends, or the descendant itself.
    ancestor_table_name_id TEXT NOT NULL REFERENCES table_names (id),
    PRIMARY KEY (descendant_table_name_id, ancestor_table_name_id)
);
CREATE INDEX ON table_name_ancestors (ancestor_table_name_id);

-- Tables whose rows a model of another table may name as its parent model
-- without extending them, such as the product a lot model belongs to. A
-- model's parent sits on the model's table, a table it extends, or an
-- admitted parent table of one of those or a table extending it. Each slice
-- seeds the rows of the tables it creates.
CREATE TABLE table_name_admitted_parents (
    -- Table of the model naming the parent.
    table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Table the parent model may sit on.
    parent_table_name_id TEXT NOT NULL REFERENCES table_names (id),
    PRIMARY KEY (table_name_id, parent_table_name_id)
);
CREATE INDEX ON table_name_admitted_parents (parent_table_name_id);

-- The table holding the instances of a model or template table's rows, the
-- asset table whose model column references the model table and the
-- procedure table of a procedure template table. Each slice seeds the rows
-- of the tables it creates.
CREATE TABLE table_name_instances (
    -- Asset model or procedure template table.
    id TEXT PRIMARY KEY REFERENCES table_names (id) ON DELETE CASCADE,
    -- Asset or procedure table holding the instances.
    instance_table_name_id TEXT NOT NULL REFERENCES table_names (id)
);
CREATE INDEX ON table_name_instances (instance_table_name_id);

-- The table an asset or procedure of a row of `model_table` names, the most
-- concrete instance table of `model_table` or a table it extends, since the
-- foreign key of an instance table admits models of every table extending
-- its model table. NULL when no such table extends all the others.
CREATE FUNCTION instance_table_name(model_table TEXT) RETURNS TEXT
LANGUAGE plpgsql STABLE SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    candidate_count BIGINT;
    candidate_table TEXT;
BEGIN
    SELECT count(DISTINCT candidate.instance_table_name_id), min(candidate.instance_table_name_id)
    INTO candidate_count, candidate_table
    FROM public.table_name_ancestors path
    JOIN public.table_name_instances candidate ON candidate.id = path.ancestor_table_name_id
    WHERE path.descendant_table_name_id = model_table AND NOT EXISTS (
        SELECT 1 FROM public.table_name_ancestors other_path
        JOIN public.table_name_instances other ON other.id = other_path.ancestor_table_name_id
        JOIN public.table_name_ancestors deeper ON deeper.descendant_table_name_id = other.instance_table_name_id
        WHERE other_path.descendant_table_name_id = model_table
            AND deeper.ancestor_table_name_id = candidate.instance_table_name_id
            AND other.instance_table_name_id <> candidate.instance_table_name_id
    );
    IF candidate_count = 1 THEN
        RETURN candidate_table;
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION instance_table_name(TEXT) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION instance_table_name(TEXT) FROM PUBLIC;

-- Rejects a row of `table_name` whose root row names the most concrete table
-- `root_table_name_id`, unless `table_name` is that table or one it extends.
-- A missing root row is left to the foreign key.
CREATE FUNCTION require_table_path(root_table_name_id TEXT, row_id UUID, table_name TEXT) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF root_table_name_id IS NOT NULL AND NOT EXISTS (
        SELECT 1 FROM public.table_name_ancestors path
        WHERE path.descendant_table_name_id = root_table_name_id AND path.ancestor_table_name_id = table_name
    ) THEN
        RAISE EXCEPTION 'row % of % is not on the table path of %', row_id, table_name, root_table_name_id
            USING ERRCODE = 'check_violation';
    END IF;
END;
$$;
ALTER FUNCTION require_table_path(TEXT, UUID, TEXT) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_table_path(TEXT, UUID, TEXT) FROM PUBLIC;


-- How a procedure treats the asset bound to a slot. `input` exists before and
-- after, `consumed` ceases to exist, `produced` is created by the procedure,
-- `intermediate` is created and used up by steps of the procedure, and
-- `instrument` performs it without entering the lineage of what is produced.
CREATE TABLE slot_roles (
    -- Role name.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);
INSERT INTO slot_roles (id) VALUES ('input'), ('consumed'), ('produced'), ('intermediate'), ('instrument');

-- Which role a step's child slot may play towards the parent slot it is bound
-- to, so the child's effect on the asset fits what the parent declares.
CREATE TABLE slot_role_pairs (
    -- Role of the child template's slot.
    child_role_id TEXT NOT NULL REFERENCES slot_roles (id),
    -- Role of the parent template's slot.
    parent_role_id TEXT NOT NULL REFERENCES slot_roles (id),
    PRIMARY KEY (child_role_id, parent_role_id)
);
CREATE INDEX ON slot_role_pairs (parent_role_id);
INSERT INTO slot_role_pairs (child_role_id, parent_role_id) VALUES
    ('input', 'input'), ('input', 'consumed'), ('input', 'produced'), ('input', 'intermediate'),
    ('consumed', 'consumed'), ('consumed', 'intermediate'),
    ('produced', 'produced'), ('produced', 'intermediate'),
    ('instrument', 'instrument');

-- Kind of value a procedure template parameter takes.
CREATE TABLE parameter_value_kinds (
    -- Kind name.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);
INSERT INTO parameter_value_kinds (id) VALUES ('number'), ('text'), ('boolean');

-- Lifecycle of a procedure template. Only a `draft` changes, a `published`
-- template may be executed and a `deprecated` one is kept for history.
CREATE TABLE procedure_template_states (
    -- State name.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);
INSERT INTO procedure_template_states (id) VALUES ('draft'), ('published'), ('deprecated');

-- Lifecycle of a procedure. The triggers in 005-procedures enforce which
-- transitions are allowed.
CREATE TABLE procedure_statuses (
    -- Status name.
    id TEXT PRIMARY KEY CHECK (id <> '' AND length(id) <= 63)
);
INSERT INTO procedure_statuses (id) VALUES ('planned'), ('in_progress'), ('completed'), ('aborted'), ('skipped');

ALTER TABLE table_names ENABLE ROW LEVEL SECURITY;
ALTER TABLE table_names FORCE ROW LEVEL SECURITY;
CREATE POLICY table_names_select ON table_names FOR SELECT TO app USING (true);
ALTER TABLE slot_roles ENABLE ROW LEVEL SECURITY;
ALTER TABLE slot_roles FORCE ROW LEVEL SECURITY;
CREATE POLICY slot_roles_select ON slot_roles FOR SELECT TO app USING (true);
ALTER TABLE slot_role_pairs ENABLE ROW LEVEL SECURITY;
ALTER TABLE slot_role_pairs FORCE ROW LEVEL SECURITY;
CREATE POLICY slot_role_pairs_select ON slot_role_pairs FOR SELECT TO app USING (true);
ALTER TABLE parameter_value_kinds ENABLE ROW LEVEL SECURITY;
ALTER TABLE parameter_value_kinds FORCE ROW LEVEL SECURITY;
CREATE POLICY parameter_value_kinds_select ON parameter_value_kinds FOR SELECT TO app USING (true);
ALTER TABLE procedure_template_states ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_template_states FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_template_states_select ON procedure_template_states FOR SELECT TO app USING (true);
ALTER TABLE procedure_statuses ENABLE ROW LEVEL SECURITY;
ALTER TABLE procedure_statuses FORCE ROW LEVEL SECURITY;
CREATE POLICY procedure_statuses_select ON procedure_statuses FOR SELECT TO app USING (true);
ALTER TABLE table_name_ancestors ENABLE ROW LEVEL SECURITY;
ALTER TABLE table_name_ancestors FORCE ROW LEVEL SECURITY;
CREATE POLICY table_name_ancestors_select ON table_name_ancestors FOR SELECT TO app USING (true);
ALTER TABLE table_name_admitted_parents ENABLE ROW LEVEL SECURITY;
ALTER TABLE table_name_admitted_parents FORCE ROW LEVEL SECURITY;
CREATE POLICY table_name_admitted_parents_select ON table_name_admitted_parents FOR SELECT TO app USING (true);
ALTER TABLE table_name_instances ENABLE ROW LEVEL SECURITY;
ALTER TABLE table_name_instances FORCE ROW LEVEL SECURITY;
CREATE POLICY table_name_instances_select ON table_name_instances FOR SELECT TO app USING (true);

GRANT SELECT ON table_names, table_name_ancestors, table_name_admitted_parents, table_name_instances, slot_roles, slot_role_pairs,
    parameter_value_kinds, procedure_template_states, procedure_statuses TO app;
GRANT SELECT ON table_name_ancestors, table_name_admitted_parents, table_name_instances TO aps_writer;
