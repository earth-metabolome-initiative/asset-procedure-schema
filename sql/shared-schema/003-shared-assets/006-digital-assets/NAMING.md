# Digital Assets Migration Naming

This document defines naming conventions for migrations under
`sql/shared-schema/003-shared-assets/006-digital-assets`.

## Goals

- Keep migration folder names predictable.
- Keep table names aligned with a single canonical stem per digital asset
  family.
- Ensure every `CREATE TABLE` has a matching registration in `table_names`.

## Conventions

1. Migration folders
- Format: `NNN-<plural-stem-slug>`
- Numeric prefixes are incremental without gaps, starting at `001`.
- `<plural-stem-slug>` is a plural entity stem in kebab-case.
- Example: `photograph` -> `001-photographs`.

2. Canonical stem
- Stems are derived from migration folder names by singularizing the final
  plural token.
- Keep one digital-asset family per migration folder.

3. Allowed table families per stem
- `<stem>_models` (model-level specializations under `digital_asset_models`)
- `<stem>s` (asset-level specializations under `digital_assets`)

4. Foreign key column naming
- `<stem>_model_id`

5. `table_names` registration
- Every table created under digital assets should have an in-place registration
  in the same migration file:
  `INSERT INTO table_names (id) VALUES ('<table_name>') ON CONFLICT DO NOTHING;`

## Current Migrations

- `001-photographs`
