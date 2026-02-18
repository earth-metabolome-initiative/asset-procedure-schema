# Marking Assets Migration Naming

This document defines naming conventions for migrations under
`sql/shared-schema/003-shared-assets/007-marking-assets`.

## Goals

- Keep migration folder names predictable.
- Keep table names aligned with a single canonical stem per marking-asset
  family.
- Ensure every `CREATE TABLE` has a matching registration in `table_names`.

## Conventions

1. Migration folders
- Format: `NNN-<plural-stem-slug>`
- Numeric prefixes are incremental without gaps, starting at `001`.
- `<plural-stem-slug>` is a plural entity stem in kebab-case.
- Example: `panel` -> `001-panels`.

2. Canonical stem
- Stems are derived from migration folder names by singularizing the final
  plural token.
- Keep one marking-asset family per migration folder.

3. Allowed table families per stem
- `<stem>_models`
- `commercial_<stem>_models` (if commercialized)
- `commercial_<stem>_lots` (if lot-tracked)
- `<stem>s` (physical marking assets)

4. Foreign key column naming
- `<stem>_model_id`
- `commercial_<stem>_model_id`
- `commercial_<stem>_lot_id`

5. `table_names` registration
- Every table created under marking assets should have an in-place registration
  in the same migration file:
  `INSERT INTO table_names (id) VALUES ('<table_name>') ON CONFLICT DO NOTHING;`

## Current Migrations

- `001-panels`
- `002-markers`
