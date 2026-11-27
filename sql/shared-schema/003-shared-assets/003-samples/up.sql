-- Migration: Shared Assets / Samples
-- Purpose: Samples for project inventories.
-- APS placement
--   `resources` of kind `'ownable'`
--       +-- asset_models
--           +-- physical_asset_models
--               +-- sample_source_models
--               +-- sample_models
--       +-- assets
--           +-- physical_assets
--               +-- sample_sources
--               +-- samples
-- Commercial tables decision: No vendor model or lot specialization is required.

-- Catalog of model definitions for assets that can be sampled from.
CREATE TABLE sample_source_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO table_names (id) VALUES ('sample_source_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'sample_source_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'sample_source_models', 'sample_source_models';
CREATE TRIGGER sample_source_models_path AFTER INSERT ON sample_source_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_sample_source_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'sample_source_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.sample_source_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'sample_source_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_sample_source_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_sample_source_models_row() FROM PUBLIC;
REVOKE ALL ON sample_source_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON sample_source_models TO aps_writer;
CREATE CONSTRAINT TRIGGER sample_source_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'sample_source_models')
    EXECUTE FUNCTION require_sample_source_models_row();
CREATE CONSTRAINT TRIGGER sample_source_models_kept AFTER DELETE ON sample_source_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_sample_source_models_row();

-- Physical assets that can act as concrete sample sources.
CREATE TABLE sample_sources (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Source model instantiated by this physical source asset.
    sample_source_model_id UUID NOT NULL REFERENCES sample_source_models (id),
    FOREIGN KEY (id, sample_source_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    sample_source_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (sample_source_model_id, sample_source_model_archived) REFERENCES sample_source_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON sample_sources (sample_source_model_id);
INSERT INTO table_names (id) VALUES ('sample_sources');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('sample_source_models', 'sample_sources');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'sample_sources', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'sample_sources', 'sample_sources';
CREATE TRIGGER sample_sources_path AFTER INSERT ON sample_sources
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_sample_sources_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'sample_sources'
    ) AND NOT EXISTS (SELECT 1 FROM public.sample_sources subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'sample_sources'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_sample_sources_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_sample_sources_row() FROM PUBLIC;
REVOKE ALL ON sample_sources FROM PUBLIC, app, identity_writer;
GRANT SELECT ON sample_sources TO aps_writer;
CREATE CONSTRAINT TRIGGER sample_sources_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'sample_sources')
    EXECUTE FUNCTION require_sample_sources_row();
CREATE CONSTRAINT TRIGGER sample_sources_kept AFTER DELETE ON sample_sources
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_sample_sources_row();

-- Catalog of sample models and the source-model family they require.
CREATE TABLE sample_models (
    -- Stable model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Source-model family that sample instances of this model must reference.
    sample_source_model_id UUID NOT NULL REFERENCES sample_source_models (id),
    UNIQUE (id, sample_source_model_id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    sample_source_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (id, archived),
    FOREIGN KEY (sample_source_model_id, sample_source_model_archived) REFERENCES sample_source_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON sample_models (sample_source_model_id);
INSERT INTO table_names (id) VALUES ('sample_models');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'sample_models', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'sample_models', 'sample_models';
CREATE TRIGGER sample_models_path AFTER INSERT ON sample_models
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_sample_models_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'sample_models'
    ) AND NOT EXISTS (SELECT 1 FROM public.sample_models subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'sample_models'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_sample_models_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_sample_models_row() FROM PUBLIC;
REVOKE ALL ON sample_models FROM PUBLIC, app, identity_writer;
GRANT SELECT ON sample_models TO aps_writer;
CREATE CONSTRAINT TRIGGER sample_models_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'sample_models')
    EXECUTE FUNCTION require_sample_models_row();
CREATE CONSTRAINT TRIGGER sample_models_kept AFTER DELETE ON sample_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_sample_models_row();

-- Requires a sample model's source family to descend from its parent's. Sample
-- models are never updated, so the check of each child covers its pair.
CREATE FUNCTION check_sample_model_family() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.sample_models child
        JOIN public.asset_models model ON model.id = child.id
        JOIN public.sample_models parent ON parent.id = model.parent_model_id
        WHERE child.id = NEW.id
          AND NOT EXISTS (
              SELECT 1 FROM public.asset_model_ancestors family
              WHERE family.descendant_model_id = child.sample_source_model_id
                AND family.ancestor_model_id = parent.sample_source_model_id
          )
    ) THEN
        RAISE EXCEPTION 'sample model source family must descend from its parent family'
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION check_sample_model_family() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION check_sample_model_family() FROM PUBLIC;
GRANT SELECT ON sample_models TO aps_writer;
CREATE CONSTRAINT TRIGGER sample_models_family AFTER INSERT OR UPDATE ON sample_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_sample_model_family();

-- Physical samples with their model's required source family.
CREATE TABLE samples (
    -- Stable asset identifier inherited from `physical_assets`.
    id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Sample model instantiated by this physical sample.
    sample_model_id UUID NOT NULL,
    -- Required source-model family of the sample model.
    sample_source_model_id UUID NOT NULL REFERENCES sample_source_models (id),
    FOREIGN KEY (id, sample_model_id) REFERENCES assets (id, model_id) ON DELETE CASCADE,
    FOREIGN KEY (sample_model_id, sample_source_model_id) REFERENCES sample_models (id, sample_source_model_id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    sample_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    sample_source_model_archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (sample_model_id, sample_model_archived) REFERENCES sample_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (sample_source_model_id, sample_source_model_archived) REFERENCES sample_source_models (id, archived) ON UPDATE CASCADE,
    FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE INDEX ON samples (sample_model_id, sample_source_model_id);
CREATE INDEX ON samples (sample_source_model_id);
INSERT INTO table_names (id) VALUES ('samples');
INSERT INTO table_name_instances (id, instance_table_name_id)
    VALUES ('sample_models', 'samples');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'samples', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_assets')
    UNION SELECT 'samples', 'samples';
CREATE TRIGGER samples_path AFTER INSERT ON samples
    FOR EACH ROW EXECUTE FUNCTION check_asset_subtype();

CREATE FUNCTION require_samples_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.assets root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'samples'
    ) AND NOT EXISTS (SELECT 1 FROM public.samples subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'samples'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_samples_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_samples_row() FROM PUBLIC;
REVOKE ALL ON samples FROM PUBLIC, app, identity_writer;
GRANT SELECT ON samples TO aps_writer;
CREATE CONSTRAINT TRIGGER samples_row AFTER INSERT ON assets
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'samples')
    EXECUTE FUNCTION require_samples_row();
CREATE CONSTRAINT TRIGGER samples_kept AFTER DELETE ON samples
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_samples_row();

GRANT SELECT, INSERT, DELETE ON sample_source_models TO app;
ALTER TABLE sample_source_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE sample_source_models FORCE ROW LEVEL SECURITY;
CREATE POLICY sample_source_models_select ON sample_source_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY sample_source_models_insert ON sample_source_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY sample_source_models_delete ON sample_source_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY sample_source_models_archived_insert ON sample_source_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived);
CREATE POLICY sample_source_models_archived_delete ON sample_source_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY sample_source_models_blocked ON sample_source_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON sample_sources TO app;
ALTER TABLE sample_sources ENABLE ROW LEVEL SECURITY;
ALTER TABLE sample_sources FORCE ROW LEVEL SECURITY;
CREATE POLICY sample_sources_select ON sample_sources FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY sample_sources_insert ON sample_sources FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND sample_source_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY sample_sources_delete ON sample_sources FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY sample_sources_archived_insert ON sample_sources AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT sample_source_model_archived);
CREATE POLICY sample_sources_archived_delete ON sample_sources AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY sample_sources_blocked ON sample_sources AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON sample_models TO app;
ALTER TABLE sample_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE sample_models FORCE ROW LEVEL SECURITY;
CREATE POLICY sample_models_select ON sample_models FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY sample_models_insert ON sample_models FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND sample_source_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY sample_models_delete ON sample_models FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY sample_models_archived_insert ON sample_models AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT sample_source_model_archived);
CREATE POLICY sample_models_archived_delete ON sample_models AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY sample_models_blocked ON sample_models AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, DELETE ON samples TO app;
ALTER TABLE samples ENABLE ROW LEVEL SECURITY;
ALTER TABLE samples FORCE ROW LEVEL SECURITY;
CREATE POLICY samples_select ON samples FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY samples_insert ON samples FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND sample_model_id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        AND sample_source_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY samples_delete ON samples FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY samples_archived_insert ON samples AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT archived AND NOT sample_model_archived AND NOT sample_source_model_archived);
CREATE POLICY samples_archived_delete ON samples AS RESTRICTIVE FOR DELETE TO app
    USING (NOT archived);
CREATE POLICY samples_blocked ON samples AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER sample_source_models_audit AFTER INSERT OR DELETE ON sample_source_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER sample_source_models_update_audit AFTER UPDATE ON sample_source_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER sample_sources_audit AFTER INSERT OR DELETE ON sample_sources
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER sample_sources_update_audit AFTER UPDATE ON sample_sources
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER sample_models_audit AFTER INSERT OR DELETE ON sample_models
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER sample_models_update_audit AFTER UPDATE ON sample_models
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER samples_audit AFTER INSERT OR DELETE ON samples
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER samples_update_audit AFTER UPDATE ON samples
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
