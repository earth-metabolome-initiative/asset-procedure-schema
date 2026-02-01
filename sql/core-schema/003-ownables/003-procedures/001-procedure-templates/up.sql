CREATE TABLE procedure_templates (
	-- Identifier of the procedure_id template
	id UUID PRIMARY KEY REFERENCES namespaced_ownables(id) ON DELETE CASCADE,
	-- Version of the procedure_id template.
	version INTEGER NOT NULL DEFAULT 1,
	-- Whether this procedure_id template is deprecated and should not be used for new procedures
	deprecated BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO table_names (id)
VALUES ('procedure_templates') ON CONFLICT DO NOTHING;