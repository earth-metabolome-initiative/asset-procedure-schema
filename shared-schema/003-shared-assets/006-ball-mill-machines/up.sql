CREATE TABLE IF NOT EXISTS ball_mill_machine_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_models (
	id UUID PRIMARY KEY,
	ball_mill_machine_model_id UUID NOT NULL REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES commercial_products(id) ON DELETE CASCADE,
	FOREIGN KEY (id, ball_mill_machine_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS commercial_ball_mill_machine_lots (
	id UUID PRIMARY KEY,
	FOREIGN KEY (id) REFERENCES commercial_product_lots(id) ON DELETE CASCADE,
	FOREIGN KEY (id) REFERENCES ball_mill_machine_models(id) ON DELETE CASCADE,
	commercial_ball_mill_machine_model_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_models(id),
	FOREIGN KEY (id, commercial_ball_mill_machine_model_id) REFERENCES asset_models(id, parent_model_id)
);
CREATE TABLE IF NOT EXISTS ball_mill_machines (
	id UUID PRIMARY KEY REFERENCES physical_assets (id) ON DELETE CASCADE,
	commercial_ball_mill_machine_lot_id UUID NOT NULL REFERENCES commercial_ball_mill_machine_lots (id),
	FOREIGN KEY (id, commercial_ball_mill_machine_lot_id) REFERENCES assets (id, model_id)
);
CREATE TABLE IF NOT EXISTS bead_models (
	id UUID PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
	-- Diameter in millimeters
	diameter REAL NOT NULL CHECK (diameter > 0.0)
);