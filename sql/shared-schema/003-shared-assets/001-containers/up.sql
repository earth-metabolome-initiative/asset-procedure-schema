CREATE TABLE container_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
INSERT INTO asset_model_tables (id) VALUES ('container_models') ON CONFLICT DO NOTHING;
CREATE TABLE container_compatibility_rules (
    container_model_id UUID NOT NULL REFERENCES container_models(id) ON DELETE CASCADE,
    contained_asset_model_id UUID NOT NULL REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    -- The maximal quantity of the right trackable that can be associated with the left trackable.
    quantity SMALLINT NOT NULL DEFAULT 1 CHECK (quantity > 0),
    creator_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (container_model_id, contained_asset_model_id),
    CHECK (
        container_model_id <> contained_asset_model_id
    )
);
-- Container models which have a known empty weight.
CREATE TABLE weighed_container_models (
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Mass in kilograms. The empty weight of the container.
    mass REAL NOT NULL CHECK (mass > 0.0)
);
INSERT INTO asset_model_tables (id) VALUES ('weighed_container_models') ON CONFLICT DO NOTHING;
CREATE TABLE volumetric_container_models (
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Volume in liters. The maximum volume of the container.
    volume REAL NOT NULL CHECK (volume > 0.0)
);
INSERT INTO asset_model_tables (id) VALUES ('volumetric_container_models') ON CONFLICT DO NOTHING;
CREATE TABLE containers (
    id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
    container_model_id UUID NOT NULL REFERENCES container_models(id),
    FOREIGN KEY (id, container_model_id) REFERENCES assets(id, model_id)
);
INSERT INTO asset_tables (id) VALUES ('containers') ON CONFLICT DO NOTHING;
CREATE TABLE volumetric_containers (
    id UUID PRIMARY KEY REFERENCES containers(id) ON DELETE CASCADE,
    volumetric_container_model_id UUID NOT NULL REFERENCES volumetric_container_models(id),
    -- We ensure that the parent_id table's container_model_id is indeed a volumetric_container_model.
    FOREIGN KEY (id, volumetric_container_model_id) REFERENCES assets(id, model_id)
);
INSERT INTO asset_tables (id) VALUES ('volumetric_containers') ON CONFLICT DO NOTHING;
-- Any physical asset whose primary function is to seal, cover, or close a container opening.
CREATE TABLE container_sealer_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
INSERT INTO asset_model_tables (id) VALUES ('container_sealer_models') ON CONFLICT DO NOTHING;