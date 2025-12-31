CREATE TABLE IF NOT EXISTS container_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
-- Container models which have a known empty weight.
CREATE TABLE IF NOT EXISTS weighed_container_models (
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Mass in kilograms. The empty weight of the container.
    mass REAL NOT NULL CHECK (mass > 0.0)
);
CREATE TABLE IF NOT EXISTS volumetric_container_models (
    id UUID PRIMARY KEY REFERENCES container_models(id) ON DELETE CASCADE,
    -- Volume in liters. The maximum volume of the container.
    volume REAL NOT NULL CHECK (volume > 0.0)
);
CREATE TABLE IF NOT EXISTS containers (
    id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
    container_model_id UUID NOT NULL REFERENCES container_models(id),
    FOREIGN KEY (id, container_model_id) REFERENCES assets(id, model_id)
);
CREATE TABLE IF NOT EXISTS volumetric_containers (
    id UUID PRIMARY KEY REFERENCES containers(id) ON DELETE CASCADE,
    volumetric_container_model_id UUID NOT NULL REFERENCES volumetric_container_models(id),
    -- We ensure that the parent_id table's container_model_id is indeed a volumetric_container_model.
    FOREIGN KEY (id, volumetric_container_model_id) REFERENCES assets(id, model_id)
);
-- Any physical asset whose primary function is to seal, cover, or close a container opening.
CREATE TABLE IF NOT EXISTS container_sealer_models (
    id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);