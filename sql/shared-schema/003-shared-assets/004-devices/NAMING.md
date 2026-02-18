# Devices Migration Naming

This document defines naming conventions for migrations under
`sql/shared-schema/003-shared-assets/004-devices`.

## Goals

- Keep migration folder names predictable.
- Keep table names aligned with a single canonical stem per device family.
- Ensure every `CREATE TABLE` has a matching registration in `table_names`.

## Conventions

1. Migration folders
- Format: `NNN-<plural-stem-slug>`
- Numeric prefixes are incremental without gaps, starting at `001`.
- `<plural-stem-slug>` is a plural entity stem in kebab-case.
- Example: `phone_device` -> `010-phone-devices`.

2. Canonical stem
- Stems are derived automatically from migration folder names by singularizing
  the final plural token.
- No Rust code changes are required when adding a new migration if the folder
  follows this naming format.

3. Allowed table families per stem
- `<stem>_models`
- `commercial_<stem>_models` (if commercialized)
- `commercial_<stem>_lots` (if lot-tracked)
- `<stem>s` (physical assets; project legacy may use irregular English, e.g. `equipments`)

4. Foreign key column naming
- `<stem>_model_id`
- `commercial_<stem>_model_id`
- `commercial_<stem>_lot_id`

5. `table_names` registration
- Every table created under devices must have an in-place registration in the
  same migration file:
  `INSERT INTO table_names (id) VALUES ('<table_name>') ON CONFLICT DO NOTHING;`

## Current Migrations

- `001-weighing-devices`
- `002-volume-measuring-devices`
- `003-ball-mill-machines`
- `004-centrifuges`
- `005-freeze-dryers`
- `006-freezers`
- `007-geopositioning-devices`
- `008-cameras`
- `009-personal-protective-equipments`
- `010-phone-devices`

## Enforcement

Run:

```bash
cargo run -p sql-migration-lint -- devices
```

This linter validates contiguous numbering, folder naming, stem/table naming,
and in-file `table_names` coverage for all device migrations.
If a migration appears to mix multiple entities, the linter emits warnings with
split suggestions.

Optional strict mode:

```bash
cargo run -p sql-migration-lint -- devices --require-full-device-chain
```

In strict mode, each stem must include all four table families:
`<stem>_models`, `commercial_<stem>_models`, `commercial_<stem>_lots`, and
physical `<stem>s`.
