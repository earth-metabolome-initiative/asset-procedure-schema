-- A commercial product is an asset model produced by some brand.
CREATE TABLE IF NOT EXISTS commercial_products (
	-- Identifier of the commercial product
	id UUID PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
	-- The brand producing this commercial product
	brand_id UUID NOT NULL REFERENCES brands(id)
);