-- Migration: Shared Assets / Commercial Products
-- Purpose: Commercial Products for project inventories.
-- APS placement
--   ownables
--       +-- asset_models
--           +-- commercial_products
--           +-- physical_asset_models
--               +-- commercial_product_lots
-- Commercial tables decision: Vendor models and lots provide purchase traceability.

-- Catalog of branded product models that extend APS asset models.
CREATE TABLE commercial_products (
    -- Stable model identifier inherited from `asset_models`.
    id UUID PRIMARY KEY REFERENCES asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- Brand responsible for manufacturing or distributing this product model.
    brand_id UUID NOT NULL REFERENCES brands (id),
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_products (brand_id);
INSERT INTO table_names (id) VALUES ('commercial_products');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_products', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('asset_models')
    UNION SELECT 'commercial_products', 'commercial_products';
CREATE TRIGGER commercial_products_path AFTER INSERT ON commercial_products
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_products_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_products'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_products subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_products'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_products_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_products_row() FROM PUBLIC;
GRANT SELECT ON commercial_products TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_products_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_products')
    EXECUTE FUNCTION require_commercial_products_row();
CREATE CONSTRAINT TRIGGER commercial_products_kept AFTER DELETE ON commercial_products
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_products_row();

-- Catalog of lot-level model variants under a commercial product model.
CREATE TABLE commercial_product_lots (
    -- Stable lot-model identifier inherited from `physical_asset_models`.
    id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE,
    -- Organization containing this resource.
    organization_id UUID NOT NULL,
    -- connetto project holding the lot, kept equal to its ownable's.
    project_id UUID NOT NULL,
    -- Manufacturer or supplier lot label.
    lot TEXT NOT NULL CHECK (lot <> '' AND length(lot) <= 255),
    -- Commercial product model associated with this lot.
    product_model_id UUID NOT NULL REFERENCES commercial_products (id) ON DELETE RESTRICT,
    UNIQUE (project_id, lot, product_model_id),
    FOREIGN KEY (id, product_model_id) REFERENCES asset_models (id, parent_model_id) ON DELETE CASCADE,
    FOREIGN KEY (id, project_id) REFERENCES ownables (id, project_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (organization_id, id) REFERENCES resources (organization_id, id) ON DELETE CASCADE
);
CREATE INDEX ON commercial_product_lots (product_model_id);
INSERT INTO table_names (id) VALUES ('commercial_product_lots');
INSERT INTO table_name_ancestors (descendant_table_name_id, ancestor_table_name_id)
    SELECT 'commercial_product_lots', ancestor_table_name_id FROM table_name_ancestors WHERE descendant_table_name_id IN ('physical_asset_models')
    UNION SELECT 'commercial_product_lots', 'commercial_product_lots';
INSERT INTO table_name_admitted_parents (table_name_id, parent_table_name_id)
    VALUES ('commercial_product_lots', 'commercial_products');
CREATE TRIGGER commercial_product_lots_path AFTER INSERT ON commercial_product_lots
    FOR EACH ROW EXECUTE FUNCTION check_asset_model_subtype();

CREATE FUNCTION require_commercial_product_lots_row() RETURNS TRIGGER
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $$
DECLARE
    row_id UUID := CASE TG_OP WHEN 'DELETE' THEN OLD.id ELSE NEW.id END;
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.asset_models root
        JOIN public.table_name_ancestors path ON path.descendant_table_name_id = root.table_name_id
        WHERE root.id = row_id AND path.ancestor_table_name_id = 'commercial_product_lots'
    ) AND NOT EXISTS (SELECT 1 FROM public.commercial_product_lots subtype WHERE subtype.id = row_id) THEN
        RAISE EXCEPTION '% names a table extending % but has no row there', row_id, 'commercial_product_lots'
            USING ERRCODE = 'foreign_key_violation';
    END IF;
    RETURN NULL;
END;
$$;
ALTER FUNCTION require_commercial_product_lots_row() OWNER TO aps_writer;
REVOKE EXECUTE ON FUNCTION require_commercial_product_lots_row() FROM PUBLIC;
GRANT SELECT ON commercial_product_lots TO aps_writer;
CREATE CONSTRAINT TRIGGER commercial_product_lots_row AFTER INSERT ON asset_models
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW WHEN (NEW.table_name_id = 'commercial_product_lots')
    EXECUTE FUNCTION require_commercial_product_lots_row();
CREATE CONSTRAINT TRIGGER commercial_product_lots_kept AFTER DELETE ON commercial_product_lots
    DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION require_commercial_product_lots_row();

GRANT SELECT, INSERT, UPDATE (brand_id), DELETE ON commercial_products TO app;
ALTER TABLE commercial_products ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_products FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_products_select ON commercial_products FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_products_insert ON commercial_products FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND brand_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_products_update ON commercial_products FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND brand_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_products_delete ON commercial_products FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_products_archived_insert ON commercial_products AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(brand_id));
CREATE POLICY commercial_products_archived_update ON commercial_products AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(brand_id));
CREATE POLICY commercial_products_archived_delete ON commercial_products AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_products_blocked ON commercial_products AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

GRANT SELECT, INSERT, UPDATE (lot), DELETE ON commercial_product_lots TO app;
ALTER TABLE commercial_product_lots ENABLE ROW LEVEL SECURITY;
ALTER TABLE commercial_product_lots FORCE ROW LEVEL SECURITY;
CREATE POLICY commercial_product_lots_select ON commercial_product_lots FOR SELECT TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('reader')))
        OR project_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_product_lots_insert ON commercial_product_lots FOR INSERT TO app
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND product_model_id = ANY (ARRAY(SELECT authz.resources_with('reader'))));
CREATE POLICY commercial_product_lots_update ON commercial_product_lots FOR UPDATE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('writer'))))
    WITH CHECK (id = ANY (ARRAY(SELECT authz.resources_with('writer')))
        AND project_id = ANY (ARRAY(SELECT authz.resources_with('writer'))));
CREATE POLICY commercial_product_lots_delete ON commercial_product_lots FOR DELETE TO app
    USING (id = ANY (ARRAY(SELECT authz.resources_with('admin'))));
CREATE POLICY commercial_product_lots_archived_insert ON commercial_product_lots AS RESTRICTIVE FOR INSERT TO app
    WITH CHECK (NOT authz.is_archived(id) AND NOT authz.is_archived(product_model_id));
CREATE POLICY commercial_product_lots_archived_update ON commercial_product_lots AS RESTRICTIVE FOR UPDATE TO app
    USING (NOT authz.is_archived(id)) WITH CHECK (NOT authz.is_archived(id));
CREATE POLICY commercial_product_lots_archived_delete ON commercial_product_lots AS RESTRICTIVE FOR DELETE TO app
    USING (NOT authz.is_archived(id));
CREATE POLICY commercial_product_lots_blocked ON commercial_product_lots AS RESTRICTIVE FOR ALL TO app
    USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))))
    WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));

-- Records every change in connetto's `audit_events`.
CREATE TRIGGER commercial_products_audit AFTER INSERT OR DELETE ON commercial_products
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_products_update_audit AFTER UPDATE ON commercial_products
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_product_lots_audit AFTER INSERT OR DELETE ON commercial_product_lots
    FOR EACH ROW EXECUTE FUNCTION authz_write.record_audit_event();
CREATE TRIGGER commercial_product_lots_update_audit AFTER UPDATE ON commercial_product_lots
    FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION authz_write.record_audit_event();
