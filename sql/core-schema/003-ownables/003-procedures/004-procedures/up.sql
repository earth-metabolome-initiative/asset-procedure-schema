CREATE TABLE procedures (
	-- The ID of this procedure.
	id UUID PRIMARY KEY REFERENCES ownables(id) ON DELETE CASCADE,
	-- The procedure_id template of this procedure.
	procedure_template_id UUID NOT NULL REFERENCES procedure_templates(id),
	-- The parent_id procedure_id (if any) of this procedure.
	parent_procedure_id UUID REFERENCES procedures(id) ON DELETE CASCADE CHECK (id <> parent_procedure_id),
	-- The parent_id procedure_id template (if any) of this procedure.
	parent_procedure_template_id UUID REFERENCES procedure_templates(id) CHECK (
		procedure_template_id <> parent_procedure_template_id
	),
	-- The predecessor_id procedure_id (if any) of this procedure.
	predecessor_procedure_id UUID REFERENCES procedures(id) ON DELETE CASCADE CHECK (id <> predecessor_procedure_id),
	-- The predecessor_id procedure_id template (if any) of this procedure.
	predecessor_procedure_template_id UUID REFERENCES procedure_templates(id) CHECK (
		procedure_template_id <> predecessor_procedure_template_id
	),
	-- We create an index on (procedure_template_id, parent_procedure_template_id) to allow for foreign
	-- keys from the concrete procedures to check that the procedure_id template is correctly aligned.
	UNIQUE (id, procedure_template_id),
	-- We enforce that if a parent_id procedure_id and parent_id procedure_id template are specified,
	-- then the parent_id procedure_id must indeed be of the specified parent_id procedure_id template.
	FOREIGN KEY (
		parent_procedure_id,
		parent_procedure_template_id
	) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that if a predecessor_id procedure_id and predecessor_id procedure_id template are specified,
	-- then the predecessor_id procedure_id must indeed be of the specified predecessor_id procedure_id template.
	FOREIGN KEY (
		predecessor_procedure_id,
		predecessor_procedure_template_id
	) REFERENCES procedures(id, procedure_template_id),
	-- We enforce that if a parent_id procedure_id template is specified, then the parent_id procedure_id template
	-- must indeed be a valid parent_id procedure_id template for the specified procedure_id template.
	FOREIGN KEY (
		parent_procedure_template_id,
		procedure_template_id
	) REFERENCES parent_procedure_templates(parent_id, child_id),
	-- We enforce that if both a predecessor_id procedure_id template and a parent_id procedure_id template are specified,
	-- then there must exist a row in `next_procedure_templates`
	FOREIGN KEY (
		parent_procedure_template_id,
		predecessor_procedure_template_id,
		procedure_template_id
	) REFERENCES next_procedure_templates(parent_id, predecessor_id, successor_id),
	-- We check that either both parent_procedure_id and parent_procedure_template_id are NULL,
	-- or neither is NULL.
	CHECK (
		(
			parent_procedure_id IS NULL
			AND parent_procedure_template_id IS NULL
		)
		OR (
			parent_procedure_id IS NOT NULL
			AND parent_procedure_template_id IS NOT NULL
		)
	),
	-- We check that either both predecessor_procedure_id and predecessor_procedure_template_id are NULL,
	-- or neither is NULL.
	CHECK (
		(
			predecessor_procedure_id IS NULL
			AND predecessor_procedure_template_id IS NULL
		)
		OR (
			predecessor_procedure_id IS NOT NULL
			AND predecessor_procedure_template_id IS NOT NULL
		)
	)
);
INSERT INTO table_names (id) VALUES ('procedures') ON CONFLICT DO NOTHING;
