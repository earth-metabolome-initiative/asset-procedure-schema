-- Asset models describe kinds of things (a tube model, a balance model, a
-- plant species), assets are the tracked things themselves. Both are connetto
-- `resources` of kind `'ownable'` living in one project, whose name is unique within that project.
CREATE TABLE asset_models (
    -- Identifier of the connetto `resource` of kind `'ownable'` this model extends.
    id UUID PRIMARY KEY REFERENCES resources (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- connetto project holding the model.
    project_id UUID NOT NULL,
    -- Most concrete APS table holding this model.
    table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Name, unique within the project.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- Free-text description.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    -- More general model this one specialises. Immutable, so the rows of
    -- `asset_model_ancestors` never go stale.
    parent_model_id UUID REFERENCES asset_models (id) CHECK (parent_model_id <> id),
    UNIQUE (project_id, name),
    UNIQUE (id, project_id),
    UNIQUE (id, parent_model_id),
    UNIQUE (id, parent_model_id, organization_id),
    FOREIGN KEY (id, project_id) REFERENCES resources (id, parent_id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Resource kind fixed to `'ownable'`.
    kind authz.resource_kind NOT NULL DEFAULT 'ownable' CHECK (kind = 'ownable'),
    -- Archive state of the row named by `parent_model_id`.
    parent_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (parent_model_id, parent_model_archived) REFERENCES asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, kind, archived) REFERENCES resources (organization_id, id, kind, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON asset_models (parent_model_id);
CREATE INDEX ON asset_models (table_name_id);
INSERT INTO table_names (id) VALUES ('asset_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id) VALUES ('asset_models', 'asset_models');

-- Reflexive transitive closure of `asset_models.parent_model_id`, written by
-- the client together with the model. A non-reflexive row names the
-- descendant's parent, and the two foreign keys through it make every row a
-- true ancestor by induction. `check_asset_model_ancestors` makes the set
-- complete at commit.
CREATE TABLE asset_model_ancestors (
    -- Model whose ancestors the row lists.
    descendant_model_id UUID NOT NULL REFERENCES asset_models (id) ON DELETE CASCADE,
    -- Ancestor of the descendant, or the descendant itself.
    ancestor_model_id UUID NOT NULL REFERENCES asset_models (id) ON DELETE CASCADE,
    -- connetto organization owning the descendant.
    organization_id UUID NOT NULL,
    -- Parent of the descendant, through which the ancestor is reached. Null on
    -- the reflexive row.
    parent_model_id UUID,
    PRIMARY KEY (descendant_model_id, ancestor_model_id),
    CHECK ((parent_model_id IS NULL) = (descendant_model_id = ancestor_model_id)),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, descendant_model_id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (descendant_model_id, parent_model_id, organization_id)
        REFERENCES asset_models (id, parent_model_id, organization_id) ON DELETE CASCADE,
    FOREIGN KEY (parent_model_id, ancestor_model_id)
        REFERENCES asset_model_ancestors (descendant_model_id, ancestor_model_id) ON DELETE CASCADE
);
CREATE INDEX ON asset_model_ancestors (ancestor_model_id);
CREATE INDEX ON asset_model_ancestors (parent_model_id, ancestor_model_id);
INSERT INTO table_names (id) VALUES ('asset_model_ancestors');

-- Rejects a model whose ancestor rows are incomplete at commit, or whose
-- parent descends from it, which two models naming each other as parent in
-- one transaction would otherwise pass.
CREATE FUNCTION check_asset_model_ancestors() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM public.asset_models model WHERE model.id = NEW.id) THEN
        RETURN NULL;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM public.asset_model_ancestors own_row
        WHERE own_row.descendant_model_id = NEW.id AND own_row.ancestor_model_id = NEW.id
    ) OR EXISTS (
        SELECT 1 FROM public.asset_model_ancestors parent_row
        WHERE parent_row.descendant_model_id = NEW.parent_model_id
          AND NOT EXISTS (
              SELECT 1 FROM public.asset_model_ancestors own_row
              WHERE own_row.descendant_model_id = NEW.id
                AND own_row.ancestor_model_id = parent_row.ancestor_model_id
          )
    ) THEN
        RAISE EXCEPTION 'asset model % lacks rows in asset_model_ancestors', NEW.id
            USING ERRCODE = 'check_violation';
    END IF;
    IF EXISTS (
        SELECT 1 FROM public.asset_model_ancestors parent_row
        WHERE parent_row.descendant_model_id = NEW.parent_model_id AND parent_row.ancestor_model_id = NEW.id
    ) THEN
        RAISE EXCEPTION 'asset model % descends from its own parent %', NEW.id, NEW.parent_model_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_asset_model_ancestors() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_asset_model_ancestors() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER asset_models_ancestors_complete AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_asset_model_ancestors();

-- Rejects a model whose table is not an asset model table, or whose parent
-- sits on a table that is neither the model's, one the model's extends, nor
-- an admitted parent table of those or a table extending it.
CREATE FUNCTION guard_asset_model() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    model_parent_table_name_id TEXT;
BEGIN
    PERFORM public.require_table_path(NEW.table_name_id, NEW.id, TG_TABLE_NAME);
    SELECT parent.table_name_id INTO model_parent_table_name_id FROM public.asset_models parent WHERE parent.id = NEW.parent_model_id;
    IF model_parent_table_name_id IS NOT NULL AND NOT EXISTS (
        SELECT 1 FROM public.table_name_ancestors path
        WHERE path.descendant_table_name_id = NEW.table_name_id AND path.ancestor_table_name_id = model_parent_table_name_id
    ) AND NOT EXISTS (
        SELECT 1 FROM public.table_name_ancestors path
        JOIN public.table_name_admitted_parents admitted ON admitted.table_name_id = path.ancestor_table_name_id
        JOIN public.table_name_ancestors parent_path ON parent_path.ancestor_table_name_id = admitted.parent_table_name_id
        WHERE path.descendant_table_name_id = NEW.table_name_id AND parent_path.descendant_table_name_id = model_parent_table_name_id
    ) THEN
        RAISE EXCEPTION 'asset model % of table % cannot have parent % of table %',
            NEW.id, NEW.table_name_id, NEW.parent_model_id, model_parent_table_name_id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_asset_model() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_asset_model() FROM PUBLIC;
CREATE TRIGGER asset_models_guard AFTER INSERT ON asset_models
    FOR EACH ROW EXECUTE FUNCTION guard_asset_model();

-- Rejects a row of an asset model subtype table that is not on the path of
-- its model's `table_name_id`.
CREATE FUNCTION check_asset_model_subtype() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.require_table_path(
        (SELECT model.table_name_id FROM public.asset_models model WHERE model.id = NEW.id), NEW.id, TG_TABLE_NAME);
    RETURN NEW;
END;
$$;
ALTER FUNCTION check_asset_model_subtype() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_asset_model_subtype() FROM PUBLIC;


CREATE TABLE physical_asset_models (
    -- Identifier of the asset model this row specialises.
    id UUID PRIMARY KEY REFERENCES asset_models (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('physical_asset_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'physical_asset_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'asset_models'
    UNION SELECT 'physical_asset_models', 'physical_asset_models';
CREATE TRIGGER physical_asset_models_path AFTER INSERT ON physical_asset_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_physical_asset_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'physical_asset_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.physical_asset_models row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'physical_asset_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_physical_asset_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_physical_asset_models_row() FROM PUBLIC;
REVOKE ALL ON physical_asset_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON physical_asset_models TO aps_writer;
CREATE CONSTRAINT TRIGGER physical_asset_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'physical_asset_models')
    EXECUTE FUNCTION require_physical_asset_models_row();
CREATE CONSTRAINT TRIGGER physical_asset_models_kept AFTER DELETE ON physical_asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_physical_asset_models_row();

CREATE TABLE digital_asset_models (
    -- Identifier of the asset model this row specialises.
    id UUID PRIMARY KEY REFERENCES asset_models (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- Media type of the files this model describes.
    mime_type TEXT NOT NULL CHECK (mime_type <> '' AND length(mime_type) <= 255),
    UNIQUE (id, mime_type, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('digital_asset_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'digital_asset_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'asset_models'
    UNION SELECT 'digital_asset_models', 'digital_asset_models';
CREATE TRIGGER digital_asset_models_path AFTER INSERT ON digital_asset_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_digital_asset_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'digital_asset_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.digital_asset_models row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'digital_asset_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_digital_asset_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_digital_asset_models_row() FROM PUBLIC;
REVOKE ALL ON digital_asset_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON digital_asset_models TO aps_writer;
CREATE CONSTRAINT TRIGGER digital_asset_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'digital_asset_models')
    EXECUTE FUNCTION require_digital_asset_models_row();
CREATE CONSTRAINT TRIGGER digital_asset_models_kept AFTER DELETE ON digital_asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_digital_asset_models_row();

CREATE TABLE assets (
    -- Identifier of the connetto `resource` of kind `'ownable'` this asset extends.
    id UUID PRIMARY KEY REFERENCES resources (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- connetto project holding the asset.
    project_id UUID NOT NULL,
    -- Most concrete APS table holding this asset.
    table_name_id TEXT NOT NULL REFERENCES table_names (id),
    -- Model the asset instantiates. Immutable. Deleting a model is refused
    -- while assets of it exist.
    model_id UUID NOT NULL REFERENCES asset_models (id),
    -- Name, unique within the project.
    name TEXT NOT NULL CHECK (name <> '' AND length(name) <= 255),
    -- Free-text description.
    description TEXT CHECK (description <> '' AND length(description) <= 8192),
    UNIQUE (project_id, name),
    UNIQUE (id, model_id),
    UNIQUE (id, model_id, organization_id),
    FOREIGN KEY (id, project_id) REFERENCES resources (id, parent_id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    -- Resource kind fixed to `'ownable'`.
    kind authz.resource_kind NOT NULL DEFAULT 'ownable' CHECK (kind = 'ownable'),
    -- Archive state of the row named by `model_id`.
    model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (model_id, model_archived) REFERENCES asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, kind, archived) REFERENCES resources (organization_id, id, kind, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON assets (model_id);
CREATE INDEX ON assets (table_name_id);
INSERT INTO table_names (id) VALUES ('assets');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id) VALUES ('assets', 'assets');
INSERT INTO table_name_instances (id, instance_table_name_id) VALUES ('asset_models', 'assets');

-- Rejects an asset whose table is not an asset table, or not the instance
-- table of its model's table.
CREATE FUNCTION guard_asset() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    model_table TEXT;
    instance_table TEXT;
BEGIN
    PERFORM public.require_table_path(NEW.table_name_id, NEW.id, TG_TABLE_NAME);
    SELECT model.table_name_id INTO model_table FROM public.asset_models model WHERE model.id = NEW.model_id;
    instance_table := public.instance_table_name(model_table);
    IF instance_table IS DISTINCT FROM NEW.table_name_id THEN
        RAISE EXCEPTION 'asset % does not name its model instance table', NEW.id USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;
ALTER FUNCTION guard_asset() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION guard_asset() FROM PUBLIC;
CREATE TRIGGER assets_guard AFTER INSERT ON assets
    FOR EACH ROW EXECUTE FUNCTION guard_asset();

-- Rejects a row of an asset subtype table that is not on the path of its
-- asset's `table_name_id`.
CREATE FUNCTION check_asset_subtype() RETURNS TRIGGER LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM public.require_table_path(
        (SELECT asset.table_name_id FROM public.assets asset WHERE asset.id = NEW.id), NEW.id, TG_TABLE_NAME);
    RETURN NEW;
END;
$$;
ALTER FUNCTION check_asset_subtype() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_asset_subtype() FROM PUBLIC;


-- Locks an asset row, so the lifetime checks of one asset run one after the
-- other.
CREATE FUNCTION lock_asset(locked_asset_id UUID) RETURNS VOID
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    PERFORM authz.refuse_repeatable_read();
    PERFORM asset.id FROM public.assets asset WHERE asset.id = locked_asset_id FOR NO KEY UPDATE;
END;
$$;
GRANT USAGE ON SCHEMA authz TO aps_writer;
GRANT EXECUTE ON FUNCTION authz.resources_with(authz.role), authz.caller_id(), authz.blocking_organizations() TO aps_writer;
REVOKE ALL ON asset_model_ancestors FROM PUBLIC, app, identity_writer;
REVOKE ALL ON asset_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON asset_models, asset_model_ancestors TO aps_writer;
REVOKE ALL ON assets FROM PUBLIC, app, identity_writer;
GRANT SELECT, UPDATE (id) ON assets TO aps_writer;
ALTER FUNCTION lock_asset(UUID) OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION lock_asset(UUID) FROM PUBLIC;

CREATE TABLE physical_assets (
    -- Identifier of the asset this row specialises.
    id UUID PRIMARY KEY REFERENCES assets (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- Model of the asset, which must be a physical asset model.
    physical_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id),
    FOREIGN KEY (id, physical_asset_model_id, organization_id) REFERENCES assets (id, model_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON physical_assets (physical_asset_model_id);
INSERT INTO table_names (id) VALUES ('physical_assets');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'physical_assets', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'assets'
    UNION SELECT 'physical_assets', 'physical_assets';
INSERT INTO table_name_instances (id, instance_table_name_id) VALUES ('physical_asset_models', 'physical_assets');
CREATE TRIGGER physical_assets_path AFTER INSERT ON physical_assets
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_physical_assets_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'physical_assets'
    ) AND NOT EXISTS (SELECT 1 FROM public.physical_assets row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'physical_assets'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_physical_assets_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_physical_assets_row() FROM PUBLIC;
REVOKE ALL ON physical_assets FROM PUBLIC, app, identity_writer;
GRANT SELECT ON physical_assets TO aps_writer;
CREATE CONSTRAINT TRIGGER physical_assets_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'physical_assets')
    EXECUTE FUNCTION require_physical_assets_row();
CREATE CONSTRAINT TRIGGER physical_assets_kept AFTER DELETE ON physical_assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_physical_assets_row();

CREATE TABLE digital_assets (
    -- Identifier of the asset this row specialises.
    id UUID PRIMARY KEY REFERENCES assets (id) ON DELETE CASCADE,
    -- connetto organization owning the row.
    organization_id UUID NOT NULL,
    -- Model of the asset, which must be a digital asset model.
    digital_asset_model_id UUID NOT NULL REFERENCES digital_asset_models (id),
    FOREIGN KEY (id, digital_asset_model_id, organization_id) REFERENCES assets (id, model_id, organization_id),
    -- Archive state inherited from this resource.
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON digital_assets (digital_asset_model_id);
INSERT INTO table_names (id) VALUES ('digital_assets');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'digital_assets', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id = 'assets'
    UNION SELECT 'digital_assets', 'digital_assets';
INSERT INTO table_name_instances (id, instance_table_name_id) VALUES ('digital_asset_models', 'digital_assets');
CREATE TRIGGER digital_assets_path AFTER INSERT ON digital_assets
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_digital_assets_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'digital_assets'
    ) AND NOT EXISTS (SELECT 1 FROM public.digital_assets row WHERE row.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'digital_assets'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_digital_assets_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_digital_assets_row() FROM PUBLIC;
REVOKE ALL ON digital_assets FROM PUBLIC, app, identity_writer;
GRANT SELECT ON digital_assets TO aps_writer;
CREATE CONSTRAINT TRIGGER digital_assets_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'digital_assets')
    EXECUTE FUNCTION require_digital_assets_row();
CREATE CONSTRAINT TRIGGER digital_assets_kept AFTER DELETE ON digital_assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_digital_assets_row();

GRANT SELECT, INSERT, UPDATE (name, description), DELETE ON asset_models TO app;
ALTER TABLE asset_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE asset_models FORCE ROW LEVEL SECURITY;
CREATE POLICY asset_models_insert ON asset_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND (parent_model_id IS NULL OR parent_model_id = ANY (ARRAY(SELECT authz.resources_with('reader')))));
CREATE POLICY asset_models_update ON asset_models FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY asset_models_delete ON asset_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY asset_models_archived_insert ON asset_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT parent_model_archived);
CREATE POLICY asset_models_archived_update ON asset_models AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY asset_models_archived_delete ON asset_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY asset_models_blocked ON asset_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT ON asset_model_ancestors TO app;
ALTER TABLE asset_model_ancestors ENABLE ROW LEVEL SECURITY;
ALTER TABLE asset_model_ancestors FORCE ROW LEVEL SECURITY;
CREATE POLICY asset_model_ancestors_insert ON asset_model_ancestors FOR INSERT TO app
    WITH CHECK (descendant_model_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND (parent_model_id IS NULL OR parent_model_id = ANY (ARRAY(SELECT authz.resources_with('reader')))));
CREATE POLICY asset_model_ancestors_archived_insert ON asset_model_ancestors AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY asset_model_ancestors_blocked ON asset_model_ancestors AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON physical_asset_models TO app;
ALTER TABLE physical_asset_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE physical_asset_models FORCE ROW LEVEL SECURITY;
CREATE POLICY physical_asset_models_select ON physical_asset_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY physical_asset_models_insert ON physical_asset_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY physical_asset_models_delete ON physical_asset_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY physical_asset_models_archived_insert ON physical_asset_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY physical_asset_models_archived_delete ON physical_asset_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY physical_asset_models_blocked ON physical_asset_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (mime_type), DELETE ON digital_asset_models TO app;
ALTER TABLE digital_asset_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE digital_asset_models FORCE ROW LEVEL SECURITY;
CREATE POLICY digital_asset_models_select ON digital_asset_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY digital_asset_models_insert ON digital_asset_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY digital_asset_models_update ON digital_asset_models FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY digital_asset_models_delete ON digital_asset_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY digital_asset_models_archived_insert ON digital_asset_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY digital_asset_models_archived_update ON digital_asset_models AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY digital_asset_models_archived_delete ON digital_asset_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY digital_asset_models_blocked ON digital_asset_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (name, description), DELETE ON assets TO app;
ALTER TABLE assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE assets FORCE ROW LEVEL SECURITY;
CREATE POLICY assets_select ON assets FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY assets_insert ON assets FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY assets_update ON assets FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY assets_delete ON assets FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY assets_archived_insert ON assets AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT model_archived);
CREATE POLICY assets_archived_update ON assets AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY assets_archived_delete ON assets AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY assets_blocked ON assets AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON physical_assets TO app;
ALTER TABLE physical_assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE physical_assets FORCE ROW LEVEL SECURITY;
CREATE POLICY physical_assets_select ON physical_assets FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY physical_assets_insert ON physical_assets FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND physical_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY physical_assets_delete ON physical_assets FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY physical_assets_archived_insert ON physical_assets AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY physical_assets_archived_delete ON physical_assets AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY physical_assets_blocked ON physical_assets AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON digital_assets TO app;
ALTER TABLE digital_assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE digital_assets FORCE ROW LEVEL SECURITY;
CREATE POLICY digital_assets_select ON digital_assets FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY digital_assets_insert ON digital_assets FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND digital_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY digital_assets_delete ON digital_assets FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY digital_assets_archived_insert ON digital_assets AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY digital_assets_archived_delete ON digital_assets AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY digital_assets_blocked ON digital_assets AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER asset_models_audit AFTER INSERT OR DELETE ON asset_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER asset_models_update_audit AFTER UPDATE ON asset_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER asset_model_ancestors_audit AFTER INSERT OR DELETE ON asset_model_ancestors
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER asset_model_ancestors_update_audit AFTER UPDATE ON asset_model_ancestors
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER physical_asset_models_audit AFTER INSERT OR DELETE ON physical_asset_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER physical_asset_models_update_audit AFTER UPDATE ON physical_asset_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER digital_asset_models_audit AFTER INSERT OR DELETE ON digital_asset_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER digital_asset_models_update_audit AFTER UPDATE ON digital_asset_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER assets_audit AFTER INSERT OR DELETE ON assets
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER assets_update_audit AFTER UPDATE ON assets
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER physical_assets_audit AFTER INSERT OR DELETE ON physical_assets
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER physical_assets_update_audit AFTER UPDATE ON physical_assets
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER digital_assets_audit AFTER INSERT OR DELETE ON digital_assets
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER digital_assets_update_audit AFTER UPDATE ON digital_assets
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
