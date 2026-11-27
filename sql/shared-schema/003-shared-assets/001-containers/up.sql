-- Migration: Shared Assets / Containers
-- Purpose: Containers for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- container_models
--                   +-- weighed_container_models
--                   +-- volumetric_container_models
--               +-- container_sealer_models
--       +-- container_compatibility_rules
--       +-- assets
--           +-- physical_assets
--               +-- containers
--                   +-- volumetric_containers
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Catalog of reusable container models for physical container assets.
CREATE TABLE container_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('container_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'container_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'container_models', 'container_models';
CREATE TRIGGER container_models_path AFTER INSERT ON container_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_container_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'container_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.container_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'container_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_container_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_models_row() FROM PUBLIC;
REVOKE ALL ON container_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON container_models TO aps_writer;
CREATE CONSTRAINT TRIGGER container_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'container_models')
    EXECUTE FUNCTION require_container_models_row();
CREATE CONSTRAINT TRIGGER container_models_kept AFTER DELETE ON container_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_container_models_row();

-- Rule set that defines which physical asset models can be contained by a specific container model and in what quantity.
CREATE TABLE container_compatibility_rules (
    -- Stable rule identifier inherited from ``resources` of kind `'ownable'``.
    id UUID PRIMARY KEY REFERENCES resources (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Project containing this ownable, the container model's project.
    project_id UUID NOT NULL,
    -- Concrete APS table holding this row.
    table_name_id TEXT NOT NULL REFERENCES table_names (id) CHECK (table_name_id = 'container_compatibility_rules'),
    -- Container model that receives contained assets.
    container_model_id UUID NOT NULL REFERENCES container_models (id) ON DELETE RESTRICT,
    -- Physical asset model allowed inside the container model.
    contained_asset_model_id UUID NOT NULL REFERENCES physical_asset_models (id) ON DELETE RESTRICT,
    -- Maximum allowed count of `contained_asset_model_id` in the container.
    quantity SMALLINT NOT NULL DEFAULT 1 CHECK (quantity > 0),
    -- Whether a template slot or a placement relies on the rule, which
    -- then keeps its quantity and stays.
    in_use BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (container_model_id, project_id, contained_asset_model_id),
    CHECK (
    container_model_id <> contained_asset_model_id
    ),
    FOREIGN KEY (id, project_id) REFERENCES resources (id, parent_id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- Moving a container model moves its rules in the same transaction.
    FOREIGN KEY (container_model_id, project_id) REFERENCES asset_models (id, project_id) DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (organization_id, container_model_id, container_model_archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    kind authz.resource_kind NOT NULL DEFAULT 'ownable' CHECK (kind = 'ownable'),
    container_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    contained_asset_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (container_model_id, container_model_archived) REFERENCES container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (contained_asset_model_id, contained_asset_model_archived) REFERENCES physical_asset_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, kind, archived) REFERENCES resources (organization_id, id, kind, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON container_compatibility_rules (contained_asset_model_id);
CREATE INDEX ON container_compatibility_rules (table_name_id);
CREATE INDEX ON container_compatibility_rules (organization_id, container_model_id);
INSERT INTO table_names (id) VALUES ('container_compatibility_rules');

-- Refuses changing or deleting a definition something already relies on.
CREATE FUNCTION refuse_frozen_definition() RETURNS TRIGGER
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    RAISE EXCEPTION 'definition % is used, so it is frozen', OLD.id USING ERRCODE = 'check_violation';
END;
$$;
ALTER FUNCTION refuse_frozen_definition() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION refuse_frozen_definition() FROM PUBLIC;
CREATE TRIGGER container_compatibility_rules_frozen BEFORE DELETE ON container_compatibility_rules
    FOR EACH ROW WHEN (OLD.in_use) EXECUTE FUNCTION refuse_frozen_definition();
CREATE TRIGGER container_compatibility_rules_frozen_update BEFORE UPDATE OF id, table_name_id, container_model_id, contained_asset_model_id, quantity, in_use ON container_compatibility_rules
    FOR EACH ROW WHEN (OLD.in_use AND (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.table_name_id IS DISTINCT FROM NEW.table_name_id
        OR OLD.container_model_id IS DISTINCT FROM NEW.container_model_id
        OR OLD.contained_asset_model_id IS DISTINCT FROM NEW.contained_asset_model_id
        OR OLD.quantity IS DISTINCT FROM NEW.quantity
        OR OLD.in_use IS DISTINCT FROM NEW.in_use)) EXECUTE FUNCTION refuse_frozen_definition();

-- Specialization of container models with known empty mass.
CREATE TABLE weighed_container_models (
    -- Stable model identifier inherited from `container_models`.
    id UUID PRIMARY KEY REFERENCES container_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Empty container mass.
    mass_kg DOUBLE PRECISION NOT NULL CHECK (mass_kg > 0.0 AND mass_kg < 'Infinity'::double precision),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('weighed_container_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighed_container_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('container_models')
    UNION SELECT 'weighed_container_models', 'weighed_container_models';
CREATE TRIGGER weighed_container_models_path AFTER INSERT ON weighed_container_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_weighed_container_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighed_container_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighed_container_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighed_container_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighed_container_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighed_container_models_row() FROM PUBLIC;
REVOKE ALL ON weighed_container_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON weighed_container_models TO aps_writer;
CREATE CONSTRAINT TRIGGER weighed_container_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighed_container_models')
    EXECUTE FUNCTION require_weighed_container_models_row();
CREATE CONSTRAINT TRIGGER weighed_container_models_kept AFTER DELETE ON weighed_container_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighed_container_models_row();

-- Specialization of container models with known maximum volume.
CREATE TABLE volumetric_container_models (
    -- Stable model identifier inherited from `container_models`.
    id UUID PRIMARY KEY REFERENCES container_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Maximum internal volume.
    volume_l DOUBLE PRECISION NOT NULL CHECK (volume_l > 0.0 AND volume_l < 'Infinity'::double precision),
    -- Whether a transfer template or a run relies on the volume, which then
    -- stays as it is.
    in_use BOOLEAN NOT NULL DEFAULT FALSE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('volumetric_container_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'volumetric_container_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('container_models')
    UNION SELECT 'volumetric_container_models', 'volumetric_container_models';
CREATE TRIGGER volumetric_container_models_path AFTER INSERT ON volumetric_container_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();
CREATE TRIGGER volumetric_container_models_frozen BEFORE DELETE ON volumetric_container_models
    FOR EACH ROW WHEN (OLD.in_use) EXECUTE FUNCTION refuse_frozen_definition();
CREATE TRIGGER volumetric_container_models_frozen_update BEFORE UPDATE OF id, volume_l, in_use ON volumetric_container_models
    FOR EACH ROW WHEN (OLD.in_use AND (OLD.id IS DISTINCT FROM NEW.id
        OR OLD.volume_l IS DISTINCT FROM NEW.volume_l
        OR OLD.in_use IS DISTINCT FROM NEW.in_use)) EXECUTE FUNCTION refuse_frozen_definition();

CREATE FUNCTION require_volumetric_container_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'volumetric_container_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.volumetric_container_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'volumetric_container_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_volumetric_container_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_volumetric_container_models_row() FROM PUBLIC;
REVOKE ALL ON volumetric_container_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON volumetric_container_models TO aps_writer;
CREATE CONSTRAINT TRIGGER volumetric_container_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'volumetric_container_models')
    EXECUTE FUNCTION require_volumetric_container_models_row();
CREATE CONSTRAINT TRIGGER volumetric_container_models_kept AFTER DELETE ON volumetric_container_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_volumetric_container_models_row();

-- Container models with both empty mass and maximum volume.
CREATE TABLE weighed_volumetric_container_models (
    -- Model shared by the weighed and volumetric subtypes.
    id UUID PRIMARY KEY REFERENCES weighed_container_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    FOREIGN KEY (id) REFERENCES volumetric_container_models (id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('weighed_volumetric_container_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'weighed_volumetric_container_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('weighed_container_models', 'volumetric_container_models')
    UNION SELECT 'weighed_volumetric_container_models', 'weighed_volumetric_container_models';
CREATE TRIGGER weighed_volumetric_container_models_path AFTER INSERT ON weighed_volumetric_container_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_weighed_volumetric_container_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'weighed_volumetric_container_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.weighed_volumetric_container_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'weighed_volumetric_container_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_weighed_volumetric_container_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_weighed_volumetric_container_models_row() FROM PUBLIC;
REVOKE ALL ON weighed_volumetric_container_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON weighed_volumetric_container_models TO aps_writer;
CREATE CONSTRAINT TRIGGER weighed_volumetric_container_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'weighed_volumetric_container_models')
    EXECUTE FUNCTION require_weighed_volumetric_container_models_row();
CREATE CONSTRAINT TRIGGER weighed_volumetric_container_models_kept AFTER DELETE ON weighed_volumetric_container_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_weighed_volumetric_container_models_row();

-- Physical containers tracked in inventory and execution contexts.
CREATE TABLE containers (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Container model instantiated by this physical container.
    container_model_id UUID NOT NULL REFERENCES container_models (id),
    FOREIGN KEY (id, container_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    container_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (container_model_id, container_model_archived) REFERENCES container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON containers (container_model_id);
INSERT INTO table_names (id) VALUES ('containers');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('container_models', 'containers');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'containers', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'containers', 'containers';
CREATE TRIGGER containers_path AFTER INSERT ON containers
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_containers_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'containers'
    ) AND NOT EXISTS (SELECT 1 FROM public.containers subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'containers'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_containers_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_containers_row() FROM PUBLIC;
REVOKE ALL ON containers FROM PUBLIC, app, identity_writer;
GRANT SELECT ON containers TO aps_writer;
CREATE CONSTRAINT TRIGGER containers_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'containers')
    EXECUTE FUNCTION require_containers_row();
CREATE CONSTRAINT TRIGGER containers_kept AFTER DELETE ON containers
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_containers_row();

-- Physical containers whose model explicitly carries volumetric metadata.
CREATE TABLE volumetric_containers (
    -- Stable asset identifier inherited from `containers`.
    id UUID PRIMARY KEY REFERENCES containers (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Volumetric model instantiated by this container.
    volumetric_container_model_id UUID NOT NULL REFERENCES volumetric_container_models (id),
    FOREIGN KEY (id, volumetric_container_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    volumetric_container_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (volumetric_container_model_id, volumetric_container_model_archived) REFERENCES volumetric_container_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON volumetric_containers (volumetric_container_model_id);
INSERT INTO table_names (id) VALUES ('volumetric_containers');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('volumetric_container_models', 'volumetric_containers');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'volumetric_containers', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('containers')
    UNION SELECT 'volumetric_containers', 'volumetric_containers';
CREATE TRIGGER volumetric_containers_path AFTER INSERT ON volumetric_containers
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_volumetric_containers_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'volumetric_containers'
    ) AND NOT EXISTS (SELECT 1 FROM public.volumetric_containers subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'volumetric_containers'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_volumetric_containers_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_volumetric_containers_row() FROM PUBLIC;
REVOKE ALL ON volumetric_containers FROM PUBLIC, app, identity_writer;
GRANT SELECT ON volumetric_containers TO aps_writer;
CREATE CONSTRAINT TRIGGER volumetric_containers_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'volumetric_containers')
    EXECUTE FUNCTION require_volumetric_containers_row();
CREATE CONSTRAINT TRIGGER volumetric_containers_kept AFTER DELETE ON volumetric_containers
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_volumetric_containers_row();

-- Catalog of models whose primary role is sealing container openings.
CREATE TABLE container_sealer_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('container_sealer_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'container_sealer_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'container_sealer_models', 'container_sealer_models';
CREATE TRIGGER container_sealer_models_path AFTER INSERT ON container_sealer_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_container_sealer_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'container_sealer_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.container_sealer_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'container_sealer_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_container_sealer_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_container_sealer_models_row() FROM PUBLIC;
REVOKE ALL ON container_sealer_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON container_sealer_models TO aps_writer;
CREATE CONSTRAINT TRIGGER container_sealer_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'container_sealer_models')
    EXECUTE FUNCTION require_container_sealer_models_row();
CREATE CONSTRAINT TRIGGER container_sealer_models_kept AFTER DELETE ON container_sealer_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_container_sealer_models_row();

GRANT SELECT, INSERT, DELETE ON container_models TO app;
ALTER TABLE container_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_models FORCE ROW LEVEL SECURITY;
CREATE POLICY container_models_select ON container_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_models_insert ON container_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY container_models_delete ON container_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY container_models_archived_insert ON container_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY container_models_archived_delete ON container_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY container_models_blocked ON container_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

REVOKE ALL ON container_compatibility_rules FROM PUBLIC, app, identity_writer;
GRANT SELECT, INSERT (id, organization_id, project_id, table_name_id, container_model_id, contained_asset_model_id, quantity), UPDATE (quantity), DELETE
    ON container_compatibility_rules TO app;
ALTER TABLE container_compatibility_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_compatibility_rules FORCE ROW LEVEL SECURITY;
CREATE POLICY container_compatibility_rules_select ON container_compatibility_rules FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR container_model_id = ANY (ARRAY(SELECT public.readable_models())));
CREATE POLICY container_compatibility_rules_insert ON container_compatibility_rules FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND container_model_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND contained_asset_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_compatibility_rules_update ON container_compatibility_rules FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND container_model_id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY container_compatibility_rules_delete ON container_compatibility_rules FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin')))
        AND container_model_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY container_compatibility_rules_archived_insert ON container_compatibility_rules AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT container_model_archived AND NOT contained_asset_model_archived);
CREATE POLICY container_compatibility_rules_archived_update ON container_compatibility_rules AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY container_compatibility_rules_archived_delete ON container_compatibility_rules AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY container_compatibility_rules_blocked ON container_compatibility_rules AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (mass_kg), DELETE ON weighed_container_models TO app;
ALTER TABLE weighed_container_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighed_container_models FORCE ROW LEVEL SECURITY;
CREATE POLICY weighed_container_models_select ON weighed_container_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighed_container_models_insert ON weighed_container_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY weighed_container_models_update ON weighed_container_models FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY weighed_container_models_delete ON weighed_container_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighed_container_models_archived_insert ON weighed_container_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY weighed_container_models_archived_update ON weighed_container_models AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY weighed_container_models_archived_delete ON weighed_container_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY weighed_container_models_blocked ON weighed_container_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT (id, organization_id, volume_l), UPDATE (volume_l), DELETE ON volumetric_container_models TO app;
ALTER TABLE volumetric_container_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE volumetric_container_models FORCE ROW LEVEL SECURITY;
CREATE POLICY volumetric_container_models_select ON volumetric_container_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volumetric_container_models_insert ON volumetric_container_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY volumetric_container_models_update ON volumetric_container_models FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY volumetric_container_models_delete ON volumetric_container_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY volumetric_container_models_archived_insert ON volumetric_container_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY volumetric_container_models_archived_update ON volumetric_container_models AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT archived) WITH CHECK (NOT archived);
CREATE POLICY volumetric_container_models_archived_delete ON volumetric_container_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY volumetric_container_models_blocked ON volumetric_container_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON containers TO app;
ALTER TABLE containers ENABLE ROW LEVEL SECURITY;
ALTER TABLE containers FORCE ROW LEVEL SECURITY;
CREATE POLICY containers_select ON containers FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY containers_insert ON containers FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY containers_delete ON containers FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY containers_archived_insert ON containers AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT container_model_archived);
CREATE POLICY containers_archived_delete ON containers AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY containers_blocked ON containers AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON volumetric_containers TO app;
ALTER TABLE volumetric_containers ENABLE ROW LEVEL SECURITY;
ALTER TABLE volumetric_containers FORCE ROW LEVEL SECURITY;
CREATE POLICY volumetric_containers_select ON volumetric_containers FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volumetric_containers_insert ON volumetric_containers FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND volumetric_container_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY volumetric_containers_delete ON volumetric_containers FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY volumetric_containers_archived_insert ON volumetric_containers AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT volumetric_container_model_archived);
CREATE POLICY volumetric_containers_archived_delete ON volumetric_containers AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY volumetric_containers_blocked ON volumetric_containers AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON container_sealer_models TO app;
ALTER TABLE container_sealer_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE container_sealer_models FORCE ROW LEVEL SECURITY;
CREATE POLICY container_sealer_models_select ON container_sealer_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY container_sealer_models_insert ON container_sealer_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY container_sealer_models_delete ON container_sealer_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY container_sealer_models_archived_insert ON container_sealer_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY container_sealer_models_archived_delete ON container_sealer_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY container_sealer_models_blocked ON container_sealer_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON weighed_volumetric_container_models TO app;
ALTER TABLE weighed_volumetric_container_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE weighed_volumetric_container_models FORCE ROW LEVEL SECURITY;
CREATE POLICY weighed_volumetric_container_models_select ON weighed_volumetric_container_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY weighed_volumetric_container_models_insert ON weighed_volumetric_container_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY weighed_volumetric_container_models_delete ON weighed_volumetric_container_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY weighed_volumetric_container_models_archived_insert ON weighed_volumetric_container_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY weighed_volumetric_container_models_archived_delete ON weighed_volumetric_container_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY weighed_volumetric_container_models_blocked ON weighed_volumetric_container_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER container_models_audit AFTER INSERT OR DELETE ON container_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_models_update_audit AFTER UPDATE ON container_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_compatibility_rules_audit AFTER INSERT OR DELETE ON container_compatibility_rules
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_compatibility_rules_update_audit AFTER UPDATE ON container_compatibility_rules
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighed_container_models_audit AFTER INSERT OR DELETE ON weighed_container_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighed_container_models_update_audit AFTER UPDATE ON weighed_container_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volumetric_container_models_audit AFTER INSERT OR DELETE ON volumetric_container_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volumetric_container_models_update_audit AFTER UPDATE ON volumetric_container_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighed_volumetric_container_models_audit AFTER INSERT OR DELETE ON weighed_volumetric_container_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER weighed_volumetric_container_models_update_audit AFTER UPDATE ON weighed_volumetric_container_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER containers_audit AFTER INSERT OR DELETE ON containers
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER containers_update_audit AFTER UPDATE ON containers
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volumetric_containers_audit AFTER INSERT OR DELETE ON volumetric_containers
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER volumetric_containers_update_audit AFTER UPDATE ON volumetric_containers
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_sealer_models_audit AFTER INSERT OR DELETE ON container_sealer_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER container_sealer_models_update_audit AFTER UPDATE ON container_sealer_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
