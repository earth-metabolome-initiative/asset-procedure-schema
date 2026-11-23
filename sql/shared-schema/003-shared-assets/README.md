# Shared Assets

![Connetto resources](https://img.shields.io/badge/access-connetto_resources-blue)

Containers, commercial products, samples, devices, supplies, photographs and marking assets extend the core asset tables. Devices measure or process assets, supplies support operations, and marking assets identify things or locations.

## Schema conventions

- Use client-generated identifiers and primary-key extensions for model and asset subtypes. Include `organization_id` and the composite foreign key to `resources (organization_id, id)` on every table.
- Root `ownables` also carry `project_id`, `table_name_id` and the composite foreign key to `ownables (id, project_id)` with `ON UPDATE CASCADE ON DELETE CASCADE`. Names are unique within a project.
- Preserve composite foreign keys matching model identities to `asset_models.parent_model_id` and `assets.model_id`. Identity-defining model references have no `UPDATE` grant.
- Store quantities as `DOUBLE PRECISION` with units in their names, such as `mass_kg`, `volume_l` and `diameter_m`. Compatibility `quantity` is a maximum count.
- Register every table with `INSERT INTO table_names (id) VALUES ('<table_name>');`. Triggers never write another table.
- Include commercial models when vendor identity matters and commercial lots when batch traceability matters.

## Authorization

Every table grants `SELECT`, `INSERT` and `DELETE` to `app`, with column-level `UPDATE` only for mutable non-key fields. Enable and force row-level security on each table.

Use permissive `<table>_select`, `<table>_insert`, `<table>_update` and `<table>_delete` policies with `id = ANY (ARRAY(SELECT authz.resources_with('<role>')))`. The roles are `reader`, `writer`, `writer` and `admin`, respectively, and the update policy exists only when an update grant exists.

Add restrictive `<table>_archived_insert`, `<table>_archived_update` and `<table>_archived_delete` policies using `NOT authz.is_archived(id)` for granted writes. Add restrictive `<table>_blocked` for all commands using `NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))` in both `USING` and `WITH CHECK`.

## Required Migration Documentation

Each `up.sql` starts with `Migration:`, `Purpose:`, an `APS placement` ASCII tree rooted at connetto `ownables`, and a `Commercial tables decision` stating whether vendor models and lots are needed. Put a short meaning-focused comment above each table and column for generated Rust documentation.

Choose a contiguous `NNN-<plural-stem-slug>` directory within the appropriate family and follow its `NAMING.md`.

## Codex prompt

```text
Create the requested asset family in sql/shared-schema/003-shared-assets/<TARGET_FOLDER>/up.sql.

Use the schema and authorization conventions in this README and the family's NAMING.md.
Include Migration, Purpose, APS placement rooted at ownables, and Commercial tables decision in the header.
State whether vendor models and lots are needed.
Put one short meaning-focused comment above each table and column for generated Rust documentation.
Use PK-extension identities, organization/resource foreign keys, client-generated identifiers and unit-suffixed DOUBLE PRECISION quantities.
Register every table in table_names.
Grant only mutable columns for updates and enable and force RLS on every table.
Add reader/writer/admin policies through authz.resources_with, restrictive archived write policies, and a restrictive organization-blocked policy for all commands.
Keep identity-defining model references immutable and retain composite model consistency foreign keys.
No trigger may write another table.
Add behavior coverage and report changed files, schema decisions and checks actually run.
```
