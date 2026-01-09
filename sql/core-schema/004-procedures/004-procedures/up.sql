-- Meta-table with the unique names of procedure tables, to be referenced by procedures
-- and facilitate DAG traversal.
CREATE TABLE procedure_tables (id TEXT PRIMARY KEY CHECK (id <> ''));
CREATE TABLE procedures (
	-- The ID of this procedure.
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
	-- The name of the most concrete table this procedure_id is associated with.
	procedure_table_id TEXT DEFAULT "procedures" NOT NULL REFERENCES procedure_tables(id),
	-- User who created this procedure.
	creator_id UUID NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure_id was created.
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- User who last updated this procedure.
	editor_id UUID NOT NULL REFERENCES users(id),
	-- Timestamp when this procedure_id was last updated.
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We check that the created_at is before or equal to edited_at.
	CHECK (created_at <= edited_at),
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
	),
	-- We check that if the previous procedure_id is specified, then the parent_id procedure_id must also be specified.
	CHECK (
		(
			predecessor_procedure_id IS NULL
			AND parent_procedure_id IS NULL
		)
		OR (
			parent_procedure_id IS NOT NULL
			AND predecessor_procedure_id IS NOT NULL
		)
	)
);
INSERT INTO procedure_tables (id) VALUES ('procedures') ON CONFLICT DO NOTHING;