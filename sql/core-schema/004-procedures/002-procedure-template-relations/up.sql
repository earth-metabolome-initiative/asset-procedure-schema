CREATE TABLE parent_procedure_templates (
	PRIMARY KEY (parent_id, child_id),
	-- The parent_id procedure_id template
	parent_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The child_id procedure_id template
	child_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE CHECK (parent_id <> child_id),
	-- The user who created this relationship
	creator_id UUID NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE next_procedure_templates (
	PRIMARY KEY (parent_id, predecessor_id, successor_id),
	-- The parent_id procedure_id template
	parent_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The predecessor_id procedure_id template
	predecessor_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The successor_id procedure_id template
	successor_id UUID NOT NULL REFERENCES procedure_templates(id) ON DELETE CASCADE,
	-- The user who created this relationship
	creator_id UUID NOT NULL REFERENCES users(id),
	-- The timestamp when this relationship was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the predecessor_id procedure
	FOREIGN KEY (parent_id, predecessor_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that the parent_id procedure_id is indeed a parent_id of the successor_id procedure
	FOREIGN KEY (parent_id, successor_id) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that the parent is not the same as the predecessor.
	CHECK (parent_id <> predecessor_id),
	-- We enforce that the parent is not the same as the successor.
	CHECK (parent_id <> successor_id),
	-- We enforce that the predecessor and successor are not the same
	CHECK (predecessor_id <> successor_id)
);