-- A commercial product is an asset model produced by some brand.
CREATE TABLE commercial_products (
	-- Identifier of the commercial product
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
	-- The brand producing this commercial product
	brand_id UUID NOT NULL REFERENCES brands(id)
);
INSERT INTO ownable_tables (id) VALUES ('commercial_products') ON CONFLICT DO NOTHING;
CREATE TABLE commercial_product_lots (
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	lot TEXT NOT NULL CHECK (lot <> '' AND length(lot) <= 255),
	product_model_id UUID NOT NULL REFERENCES commercial_products(id) ON DELETE CASCADE,
	-- A lot must be unique within the scope of a product model.
	UNIQUE (lot, product_model_id),
	-- The parent_id product model must be a commercial product.
	FOREIGN KEY (id, product_model_id) REFERENCES asset_models(id, parent_model_id)
);
INSERT INTO ownable_tables (id) VALUES ('commercial_product_lots') ON CONFLICT DO NOTHING;