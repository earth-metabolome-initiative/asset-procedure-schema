# Consumables Migration Naming

This document defines naming conventions for migrations under
`sql/shared-schema/003-shared-assets/005-consumables`.

## Goals

- Keep migration folder names predictable.
- Keep table names aligned with a single canonical stem per consumable family.
- Ensure every `CREATE TABLE` has a matching registration in `table_names`.

## Conventions

1. Migration folders
- Format: `NNN-<plural-stem-slug>`
- Numeric prefixes are incremental without gaps, starting at `001`.
- `<plural-stem-slug>` is a plural entity stem in kebab-case.
- Example: `personal_protective_equipment` -> `001-personal-protective-equipments`.

2. Canonical stem
- Stems are derived from migration folder names by singularizing the final
  plural token.
- Keep one consumable family per migration folder.

3. Allowed table families per stem
- `<stem>_models`
- `commercial_<stem>_models` (if commercialized)
- `commercial_<stem>_lots` (if lot-tracked)
- `<stem>s` (for physical consumable assets)

4. Foreign key column naming
- `<stem>_model_id`
- `commercial_<stem>_model_id`
- `commercial_<stem>_lot_id`

5. `table_names` registration
- Every table created under consumables should have an in-place registration in
  the same migration file:
  `INSERT INTO table_names (id) VALUES ('<table_name>') ON CONFLICT DO NOTHING;`

## Current Migrations

- `001-personal-protective-equipments`
- `002-packaging-models`
- `003-beads`
