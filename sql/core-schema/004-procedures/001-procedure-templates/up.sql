-- Meta-table with the unique names of procedure template tables, to be referenced by procedure templates
-- and facilitate DAG traversal.
CREATE TABLE procedure_template_tables (id TEXT PRIMARY KEY CHECK (id <> ''));
CREATE TABLE procedure_templates (
	-- Identifier of the procedure_id template
	id UUID PRIMARY KEY DEFAULT uuidv7(),
	-- The most concrete table variant descendant of this procedure_id template,
	-- which allows for rapidly determining the type of a procedure_id template
	-- without having to execute multiple queries.
	procedure_template_table_id TEXT DEFAULT 'procedure_templates' NOT NULL REFERENCES procedure_template_tables(id),
	-- Version of the procedure_id template.
	version INTEGER NOT NULL DEFAULT 1,
	-- Human-readable name of the procedure_id template
	name TEXT UNIQUE NOT NULL CHECK (name <> ''),
	-- Human-readable description of the procedure_id template
	description TEXT NOT NULL CHECK (description <> ''),
	-- The user who created this procedure_id template
	creator_id UUID NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure_id template was created
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- The user who last updated this procedure_id template
	editor_id UUID NOT NULL REFERENCES users(id),
	-- The timestamp when this procedure_id template was last updated
	edited_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP CHECK (created_at <= edited_at),
	-- Whether this procedure_id template is deprecated and should not be used for new procedures
	deprecated BOOLEAN NOT NULL DEFAULT FALSE,
	-- We enforce that the name and description are distinct to avoid lazy duplicates
	CHECK (name <> description)
);
INSERT INTO procedure_template_tables (id) VALUES ('procedure_templates') ON CONFLICT DO NOTHING;

CREATE OR REPLACE FUNCTION update_procedure_templates_edited_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.edited_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_procedure_templates_edited_at
BEFORE UPDATE ON procedure_templates
FOR EACH ROW EXECUTE FUNCTION update_procedure_templates_edited_at();